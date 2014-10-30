.PHONY: clean lib test

all: main

test: main lib
	./main ~/.bitcoin/blocks/blk00000.dat

main: main.rs lib
	rustc -L . main.rs

lib: src/*
	rustc src/lib.rs



clean: 
	rm *.rlib
	rm main
