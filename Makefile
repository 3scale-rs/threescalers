out/usr/lib64/libthreescalers.so:
	cargo cinstall --destdir=out --prefix=/usr --libdir=/usr/lib64

.PHONY: so-build
so-build: out/usr/lib64/libthreescalers.so
	strip -s out/usr/lib64/libthreescalers.so.*.*

.PHONY: so-install
so-install: out/usr/lib64/libthreescalers.so
	sudo chown -R root: out
	sudo cp -av out/* /

.PHONY: so-clean
so-clean:
	-sudo rm -rf out/

.PHONY: so-distclean
so-distclean: so-clean
	sudo rm -rf /usr/lib64/libthreescalers*
	sudo rm -rf /usr/lib64/pkgconfig/threescalers.pc
	-sudo rmdir /usr/lib64/pkgconfig
	sudo rm -rf /usr/include/threescalers
	-sudo rmdir /usr/include
