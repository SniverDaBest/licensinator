.PHONY: install

install:
	mkdir -p /usr/local/licensinator
	install -D -m 755 -o root target/debug/licensinator /usr/local/licensinator/licensinator
	install -D -m 755 -o root license_list.json /usr/local/licensinator/license_list.json
	install -D -m 755 -o root AGPLv3-LICENSE /usr/local/licensinator/AGPLv3-LICENSE
	install -D -m 755 -o root LGPLv3-LICENSE /usr/local/licensinator/LGPLv3-LICENSE
	install -D -m 755 -o root GPLv3-LICENSE  /usr/local/licensinator/GPLv3-LICENSE
	install -D -m 755 -o root BSD2-LICENSE /usr/local/licensinator/BSD2-LICENSE
	install -D -m 755 -o root BSD3-LICENSE /usr/local/licensinator/BSD3-LICENSE
	install -D -m 755 -o root BSD4-LICENSE /usr/local/licensinator/BSD4-LICENSE
	install -D -m 755 -o root UNLICENSE-LICENSE /usr/local/licensinator/UNLICENSE-LICENSE
	install -D -m 755 -o root APACHE2.0-LICENSE /usr/local/licensinator/APACHE2.0-LICENSE
	install -D -m 755 -o root MPL2.0-LICENSE /usr/local/licensinator/MPL2.0-LICENSE
	install -D -m 755 -o root BOOST-LICENSE /usr/local/licensinator/BOOST-LICENSE
	install -D -m 755 -o root MIT-LICENSE /usr/local/licensinator/MIT-LICENSE
