INSTALL_DIR = /usr/bin/
BINARYNAME = crowbook
BINARY = target/release/$(BINARYNAME)

PKGNAME= $(BINARYNAME)
MAINTAINER = "Élisabeth Henry \<liz.henry@ouvaton.org\>"
PKGVERSION = "0.4.1-unreleased"
PKGLICENSE = "LGPL-2.1+"

default: $(BINARY)

clean:
	cargo clean

$(BINARY): src/lib/lib*.rs src/bin/*.rs Cargo.toml
	cargo build --release

package:
	checkinstall -D --install=no --pkgname $(PKGNAME) --pkgversion $(PKGVERSION) --pkglicense $(PKGLICENSE) --maintainer $(MAINTAINER)

install: $(BINARY)
	install -d $(DESTDIR)$(INSTALL_DIR)
	install $(BINARY) $(DESTDIR)$(INSTALL_DIR)

uninstall:
	rm $(DESTDIR)/usr/bin/$(BINARYNAME)
