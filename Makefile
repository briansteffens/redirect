default: build

build:
	cargo build --release

install:
	mkdir -p ${DESTDIR}/usr/bin
	cp target/release/redirect ${DESTDIR}/usr/bin/redirect

clean:
	rm Cargo.lock
	rm -r target
