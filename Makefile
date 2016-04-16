all:
	cargo build --release

install:
	cargo build --release
	install -m755 -d $(DESTDIR)/usr/bin
	install -m755 target/release/prepend $(DESTDIR)/usr/bin/prepend

uninstall:
	rm $(DESTDIR)/usr/bin/prepend

clean:
	rm -r target
