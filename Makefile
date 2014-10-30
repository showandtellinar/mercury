.PHONY: clean lib test

all: main

test: main lib
	time ./main ~/.bitcoin/blocks/blk00000.dat

main: main.rs lib
	rustc --opt-level 2 -L . main.rs

lib: src/*
	rustc --opt-level 2 src/lib.rs

clean: 
	rm *.rlib
	rm main
