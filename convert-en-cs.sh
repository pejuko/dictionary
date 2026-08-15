#!/usr/bin/env bash

. ./common.sh

TIMESTAMP="$1"

if [ -z "$TIMESTAMP" ]; then
    # echo "Usage: convert-en-cs.sh <WIKTIONARY TIMESTAMP>"
    # echo "Example:"
    # echo "    ./convert-en-cs.sh 20250301"
    # echo "    ./convert-en-cs.sh latest"
    # exit 1
    TIMESTAMP="latest"
    # let's check if we have latest file or download the latest
    cargo run --release
fi

DIR="data/kindle-en-cs"

cargo run --release --\
    -i data/en-cs.txt \
    -o $DIR \
    -f \
    -p US:data/en_US.txt -p UK:data/en_UK.txt \
    -w data/enwiktionary-$TIMESTAMP-pages-articles.xml.bz2 -wp Czech \
    -t "English-Czech Dictionary GNU/FDL (pejuko)" \
    -sl en \
    -tl cs \
    -a pejuko \
    && \
    wine "$KINDLEGEN" -c1 -gen_ff_mobi7 -dont_append_source $DIR/content.opf
