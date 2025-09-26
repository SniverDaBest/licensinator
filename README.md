>[!TIP]
> This is an early WIP! Not every feature is implemented yet.

[![](https://tokei.rs/b1/github/SniverDaBest/licensinator)](https://github.com/SniverDaBest/licensinator)

# Info
Licensinator is a smallish Rust program to check the current license of a project with one command.\
It will ~~eventually~~ soon have the ability to change and/or create license files in your own projects!

# Bugs
There's probably a million bugs in this code, but I just don't really have the time to find and fix them all at once.\
If you find one, create a GitHub Issue, or if you want to fix it, make a fork and create a Pull Request!\

## Code Quality
If you want to increase the code quality, go ahead. I write really, REALLY bad code.

# Implemented Licenses
I can't implement every single license under the sun, that'd be insane. But we can implement a large chunk of the commonly used ones.\
Here's a list of what I've added so far:
* AGPLv3
* LGPLv3
* GPLv3
* BSD 2-Clause
* BSD 3-Clause
* BSD 4-Clause
* Unlicense
* MIT
* MPL 2.0
* Apache 2.0
* Boost

## TODO
I want to implement these licenses next:
* Zlib
* LGPLv2.0
* LGPLv2.1
* AGPLv2
* GPLv2
* ... and more!

# Installation
>[!CRITICAL]
> This assumes you're using Linux.

If you want to install Licensinator, you need to run the following code in a terminal:
```bash
cargo build
sudo make install

# if you use bash
echo "export PATH=$PATH:/usr/local/licensinator/" >> ~/.bashrc

# if you use zsh
echo "export PATH=$PATH:/usr/local/licensinator/" >> ~/.zshrc
```

# License
This program is licensed under the BSD 2-Clause License. See `LICENSE`.\
Or maybe run `licensinator` after installing, and see that it works for yourself!