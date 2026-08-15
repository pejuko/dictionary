#!/usr/bin/env bash

. ./common.sh

DIR="data/kindle-en-de"

cargo run --release --\
    -o $DIR \
    -f \
    -p US:data/en_US.txt -p UK:data/en_UK.txt \
    -w data/enwiktionary-20250201-pages-articles.xml.bz2 -wp German \
    -t "English-German Dictionary (pejuko)" \
    -sl en \
    -tl de \
    -a pejuko \
    && \
    wine "$KINDLEGEN" -c1 -gen_ff_mobi7 -dont_append_source $DIR/content.opf
