#!/bin/sh

shopt -s globstar
for f in **; do
    if [[ $f == *\.* ]]; 
    then
        git mv $f $f.old;
    fi;
done
