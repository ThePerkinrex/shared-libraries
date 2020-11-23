.PHONY: build

build: dylibs/c/main.c dylibs/rust/main.rs dylibs/v/main.v
	$(MAKE) -C dylibs/c/ build
	$(MAKE) -C dylibs/rust/ build
	$(MAKE) -C dylibs/v/ build

clean:
	rm dylibs/*.so
