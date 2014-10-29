.PHONY: clean lib

all: main

main: main.rs lib
	rustc -L . main.rs

lib: src/*
	rustc src/lib.rs

clean: 
	rm *.rlib
	rm main
