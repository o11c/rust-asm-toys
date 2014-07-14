default: lib/libasm.rlib bin/hello bin/arg_dump
lib/libasm.rlib: src/lib.rs
bin/hello: examples/hello.rs lib/libasm.rlib lib/start.o
bin/arg_dump: examples/arg_dump.rs lib/libasm.rlib lib/start.o

bin/%:
	mkdir -p bin
	rustc -C link_args='-nostartfiles lib/start.o' -O $< -o $@ -L lib
lib/lib%.rlib:
	mkdir -p lib
	rustc -O --crate-type=rlib $< -o $@
lib/start.o: src/start.c
	cc -O -c $< -o $@
clean:
	rm -rf obj bin lib
