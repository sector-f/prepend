PREFIX			?= /usr
RM				?= rm -f
INSTALL_DIR		?= install -m755 -d
INSTALL_PROG	?= install -m755
INSTALL_FILE	?= install -m644

all:
	cargo build --release

install:
	cargo build --release
	$(INSTALL_DIR) $(DESTDIR)$(PREFIX)/bin
	$(INSTALL_DIR) $(DESTDIR)$(PREFIX)/share/man/man1
	$(INSTALL_DIR) $(DESTDIR)$(PREFIX)/share/licenses/prepend
	$(INSTALL_PROG) target/release/prepend $(DESTDIR)$(PREFIX)/bin/prepend
	$(INSTALL_FILE) prepend.1 $(DESTDIR)$(PREFIX)/share/man/man1/prepend.1
	$(INSTALL_FILE) LICENSE $(DESTDIR)$(PREFIX)/share/licenses/prepend/LICENSE

uninstall:
	$(RM) $(DESTDIR)$(PREFIX)/bin/prepend
	$(RM) $(DESTDIR)$(PREFIX)/share/man/man1/prepend.1
	$(RM) $(DESTDIR)$(PREFIX)/share/licenses/prepend

clean:
	$(RM) -r target
