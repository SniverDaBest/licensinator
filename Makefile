.PHONY: install

install:
	mkdir -p /usr/local/licensinator
	install -D -m 755 -o root target/debug/licensinator /usr/local/licensinator/licensinator
	install -D -m 755 -o root license_list.json /usr/local/licensinator/license_list.json
	install -D -m 755 -o root licenses/AGPLv3-LICENSE /usr/local/licensinator/licenses/AGPLv3-LICENSE
	install -D -m 755 -o root licenses/LGPLv3-LICENSE /usr/local/licensinator/licenses/LGPLv3-LICENSE
	install -D -m 755 -o root licenses/GPLv3-LICENSE  /usr/local/licensinator/licenses/GPLv3-LICENSE
	install -D -m 755 -o root licenses/BSD2-LICENSE /usr/local/licensinator/licenses/BSD2-LICENSE
	install -D -m 755 -o root licenses/BSD3-LICENSE /usr/local/licensinator/licenses/BSD3-LICENSE
	install -D -m 755 -o root licenses/BSD4-LICENSE /usr/local/licensinator/licenses/BSD4-LICENSE
	install -D -m 755 -o root licenses/UNLICENSE-LICENSE /usr/local/licensinator/licenses/UNLICENSE-LICENSE
	install -D -m 755 -o root licenses/APACHE2.0-LICENSE /usr/local/licensinator/licenses/APACHE2.0-LICENSE
	install -D -m 755 -o root licenses/MPL2.0-LICENSE /usr/local/licensinator/licenses/MPL2.0-LICENSE
	install -D -m 755 -o root licenses/BOOST-LICENSE /usr/local/licensinator/licenses/BOOST-LICENSE
	install -D -m 755 -o root licenses/MIT-LICENSE /usr/local/licensinator/licenses/MIT-LICENSE
