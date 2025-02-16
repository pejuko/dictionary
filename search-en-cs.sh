#!/usr/bin/env bash

cargo run --release --\
    -i data/en-cs.txt \
    -p US:data/en_US.txt -p UK:data/en_UK.txt \
    -t "English-Czech Dictionary (pejuko)" \
    -sl en \
    -tl cs \
    -a pejuko \
    -s "$1"
