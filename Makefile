default: build

build:
	cargo build --release

install:
	mkdir -p ${DESTDIR}/usr/local/bin
	cp target/release/redirect ${DESTDIR}/usr/local/bin/redirect

clean:
	rm Cargo.lock
	rm -r target
