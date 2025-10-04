use chrono::{Utc, Datelike};
use serde::Deserialize;
use std::{
    env, fmt::{self, Formatter}, fs::{self, File}, io::{self, Read, Write}, path::{Path, PathBuf}, process, str::FromStr
};

#[derive(Debug, Deserialize)]
struct JsonLicense {
    name: String,
    desc: String,
    path: Option<String>,
}

#[derive(Clone, Debug)]
pub struct License {
    name: String,
    desc: String,
    content: Option<String>,
}

impl fmt::Display for License {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}{}",
            self.name,
            self.desc,
            if self.content.is_none() {
                " (no file)"
            } else {
                ""
            }
        )
    }
}

pub enum LicenseType {
    BSD2Clause,
    BSD3Clause,
    BSD4Clause,
    MIT,
    GPLv3,
    LGPLv3,
    AGPLv3,
    Boost,
    MPL2_0,
    Apache2_0,
    Unlicense,
    Unknown,
    None,
}

fn get_licenses() -> Option<Vec<License>> {
    let ep = env::current_exe().unwrap();
    let mut sep = ep.to_str().unwrap().split("/").collect::<Vec<&str>>();
    sep.pop();
    let mut estr = String::new();

    for x in sep {
        estr.push('/');
        estr.push_str(x);
        estr.push('/');
    }
    let exepath = estr.clone();
    estr.push_str("/license_list.json");

    let lp = PathBuf::from_str(estr.as_str()).unwrap();
    let mut file = match File::open(lp) {
        Ok(f) => f,
        Err(_) => {
            match File::open("license_list.json") {
                Ok(f) => f,
                Err(e) => {
                    eprintln!(
                        "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to open '{}'! Error: \x1b[31m{}\x1b[0m",
                        estr, e
                    );
                    return None;
                }
            }
        }
    };

    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => {}
        Err(e) => {
            eprintln!(
                "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to read '{}'! Error: \x1b[31m{}\x1b[0m",
                estr, e
            );
            return None;
        }
    }

    let jls = match serde_json::from_str::<Vec<JsonLicense>>(buf.as_str()) {
        Ok(j) => j,
        Err(e) => {
            eprintln!(
                "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to parse license data list! Error: \x1b[31m{}\x1b[0m",
                e
            );
            return None;
        }
    };

    let mut licenses = Vec::new();

    for jl in jls {
        if jl.path.is_some() {
            let jlpstring = jl.path.unwrap();
            let jlstr = jlpstring.as_str();
            let jlpath = PathBuf::from_str(jlstr).unwrap();
            let tf = File::open(jlpath);
            if tf.is_err() {
                let mut exejlstr = String::new();
                exejlstr.push_str(exepath.as_str());
                exejlstr.push_str(jlstr);
                let jlpath = PathBuf::from_str(exejlstr.as_str()).unwrap();
                let mut jlfile = match File::open(jlpath) {
                    Ok(f) => f,
                    Err(e) => {
                        eprintln!(
                            "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to open '{}'! Error: \x1b[31m{}\x1b[0m",
                            exejlstr, e
                        );
                        return None;
                    }
                };
                let mut buf = String::new();
                match jlfile.read_to_string(&mut buf) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to read '{}'! Error: \x1b[31m{}\x1b[0m",
                            exejlstr, e
                        );
                        return None;
                    }
                }

                let license = License {
                    name: jl.name,
                    desc: jl.desc,
                    content: Some(buf),
                };
                licenses.push(license);
                continue;
            }
            let mut jlfile = tf.unwrap();
            let mut buf = String::new();
            match jlfile.read_to_string(&mut buf) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to read '{}'! Error: \x1b[31m{}\x1b[0m",
                        estr, e
                    );
                    return None;
                }
            }

            let license = License {
                name: jl.name,
                desc: jl.desc,
                content: Some(buf),
            };
            licenses.push(license);
        } else {
            licenses.push(License {
                name: jl.name,
                desc: jl.desc,
                content: None,
            });
        }
    }

    return Some(licenses);
}

fn first_two_lines(s: String) -> Vec<String> {
    return vec![s.split("\n").collect::<Vec<&str>>()[0].to_string(), s.split("\n").collect::<Vec<&str>>()[1].to_string()];
}

fn license_name_to_type(name: String) -> LicenseType {
    return match name.as_str() {
        "BSD 2-Clause License" => LicenseType::BSD2Clause,
        "BSD 3-Clause License" => LicenseType::BSD3Clause,
        "BSD 4-Clause License" => LicenseType::BSD4Clause,
        "MIT License" => LicenseType::MIT,
        "GNU GPL v3" => LicenseType::GPLv3,
        "GNU AGPL v3" => LicenseType::AGPLv3,
        "GNU LGPL v3" => LicenseType::LGPLv3,
        "Mozilla Public License 2.0" => LicenseType::MPL2_0,
        "Apache License 2.0" => LicenseType::Apache2_0,
        "Boost Software License 1.0" => LicenseType::Boost,
        "The Unlicense" => LicenseType::Unlicense,
        _ => { LicenseType::Unknown },
    };
}

fn check_current_license(licenses: &Vec<License>) -> Option<LicenseType> {
    if match fs::exists("LICENSE") {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to check if file exists! Error: \x1b[31m{}\x1b[0m",
                e
            );
            return None;
        }
    } {
        let mut file = match File::open(PathBuf::from_str("LICENSE").unwrap()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to open local license file! Error: \x1b[31m{}\x1b[0m",
                    e
                );
                return None;
            }
        };

        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to read local license file! Error: \x1b[31m{}\x1b[0m",
                    e
                );
                return None;
            }
        }

        let fl = first_two_lines(buffer);
        
        for l in licenses.clone() {
            match l.content {
                Some(c) => {
                    let fll = first_two_lines(c);
                    if fl == fll {
                        return Some(license_name_to_type(l.name));
                    }
                }
                None => continue,
            }
        }
        return Some(LicenseType::Unknown);
    } else if match fs::exists("LICENSE.txt") {
        Ok(e) => e,
        Err(e) => {
            eprintln!(
                "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to check if file exists! Error: \x1b[31m{}\x1b[0m",
                e
            );
            return None;
        }
    } {
        let mut file = match File::open(PathBuf::from_str("LICENSE.txt").unwrap()) {
            Ok(f) => f,
            Err(e) => {
                eprintln!(
                    "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to open local license file! Error: \x1b[31m{}\x1b[0m",
                    e
                );
                return None;
            }
        };

        let mut buffer = String::new();
        match file.read_to_string(&mut buffer) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "[\x1b[31mFAIL\x1b[0m]\n\x1b[31m(X_X) [ERROR]:\x1b[0m Unable to read local license file! Error: \x1b[31m{}\x1b[0m",
                    e
                );
                return None;
            }
        }

        let fl = first_two_lines(buffer);
        
        for l in licenses.clone() {
            match l.content {
                Some(c) => {
                    let fll = first_two_lines(c);
                    if fl == fll {
                        return Some(license_name_to_type(l.name));
                    }
                }
                None => continue,
            }
        }
        return Some(LicenseType::Unknown);
    } else {
        return Some(LicenseType::None);
    }
}

fn license_type_to_name(ltype: &LicenseType) -> Option<String> {
    return match ltype {
        LicenseType::BSD2Clause => Some("BSD 2-Clause License".to_string()),
        LicenseType::BSD3Clause => Some("BSD 3-Clause License".to_string()),
        LicenseType::BSD4Clause => Some("BSD 4-Clause License".to_string()),
        LicenseType::MIT => Some("MIT License".to_string()),
        LicenseType::GPLv3 => Some("GNU GPL v3".to_string()),
        LicenseType::AGPLv3 => Some("GNU AGPL v3".to_string()),
        LicenseType::LGPLv3 => Some("GNU LGPL v3".to_string()),
        LicenseType::MPL2_0 => Some("Mozilla Public License 2.0".to_string()),
        LicenseType::Apache2_0 => Some("Apache License 2.0".to_string()),
        LicenseType::Boost => Some("Boost Software License 1.0".to_string()),
        LicenseType::Unlicense => Some("The Unlicense".to_string()),
        _ => { None },
    };
}

fn license_from_type(ltype: LicenseType, licenses: &Vec<License>) -> Option<License> {
    for l in licenses {
        return if l.name == license_type_to_name(&ltype).unwrap_or("Unknown".to_string()) { Some(l.clone()) } else { continue; };
    }

    return None;
}

fn get_username_from_input() -> String {
    print!("Enter your name (for license): ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();

    let name = name.trim();
    if name.is_empty() {
        return "Unknown".to_string();
    }

    return name.to_string();
}

fn get_current_year() -> String {
    let x = Utc::now().year();
    return format!("{}", x);
}

fn fill_placeholders(template: &str, user: &str, year: &str) -> String {
    let mut s = template.replace("[year]", year);
    s = s.replace("[user]", user);
    return s;
}

fn backup_existing_license(path: &Path) -> io::Result<()> {
    if path.exists() {
        let backup_name = format!(
            "LICENSE.bak.{}",
            Utc::now().format("%Y%m%d%H%M%S")
        );
        fs::rename(path, backup_name)?;
    }
    return Ok(());
}

fn write_license_file(path: &Path, content: &str) -> io::Result<()> {
    fs::write(path, content)?;
    return Ok(());
}

fn main() {
    println!(
        "\x1b[33mLicensinator\x1b[0m - \x1b[32mv{}\x1b[0m",
        env!("CARGO_PKG_VERSION")
    );

    print!("Initializing licenses...      ");
    io::stdout().flush().unwrap();
    let licenses = match get_licenses() {
        Some(l) => l,
        None => process::exit(1),
    };
    println!("[ \x1b[32mOK \x1b[0m]");

    print!("Checking local license...     ");
    io::stdout().flush().unwrap();
    let ltype = match check_current_license(&licenses) {
        Some(t) => t,
        None => { process::exit(1); }
    };
    match ltype {
        LicenseType::Unknown => println!("[ \x1b[30m?? \x1b[0m]"),
        LicenseType::None => println!("[ \x1b[30mNA \x1b[0m]"),
        _ => println!("[ \x1b[32mOK \x1b[0m]"),
    }

    println!("Local license: {}", license_from_type(ltype, &licenses).unwrap_or(License { name: "Unknown/None".to_string(), desc: "Unknown/None".to_string(), content: None }));
    println!();
    println!("Available tools:");
    println!(" 1) list");
    println!(" 2) modify/create");
    println!(" 3) quit");
    println!();

    let mut buf = String::new();

    loop {
        print!("Which tool do you want to use? [1-3]: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buf).unwrap();

        match buf.as_str().trim() {
            "1" => {
                println!("Licenses:");
                for l in &licenses {
                    println!(" - {}", l);
                }
            }

            "2" => {
                println!("Which of these licenses would you like to use?");
                for (i, license) in licenses.iter().enumerate() {
                    println!(" {}) {}", i + 1, license);
                }

                loop {
                    buf.clear();

                    print!("Select a license [1-{}] or 'q' to cancel: ", licenses.len());
                    io::stdout().flush().unwrap();
                    io::stdin().read_line(&mut buf).unwrap();

                    let input = buf.trim().to_lowercase();
                    if input == "q" {
                        println!("Canceled.");
                        break;
                    }

                    let idx: usize = match input.parse::<usize>() {
                        Ok(num) if num >= 1 && num <= licenses.len() => num - 1,
                        _ => {
                            println!("Invalid selection. Try again.");
                            continue;
                        }
                    };

                    let selected = &licenses[idx];

                    if selected.content.is_none() {
                        println!("That license has no template file associated with it.");
                        break;
                    }

                    let user = get_username_from_input();
                    let year = get_current_year();

                    let filled = fill_placeholders(
                        selected.content.as_ref().unwrap(),
                        &user,
                        &year,
                    );

                    let license_path = Path::new("LICENSE");
                    if let Err(e) = backup_existing_license(license_path) {
                        eprintln!("Warning: could not back up old LICENSE file: {}", e);
                    }

                    match write_license_file(license_path, &filled) {
                        Ok(_) => {
                            println!(
                                "License '{}' installed successfully to '{}'.",
                                selected.name,
                                license_path.display()
                            );
                        }
                        Err(e) => {
                            eprintln!("Failed to write license: {}", e);
                        }
                    }

                    break;
                }
            }

            "3" => process::exit(0),

            _ => {
                println!("Invalid input! Please enter a number from 1 - 3.");
            }
        }

        buf.clear();
    }
}
