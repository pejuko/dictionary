#!/usr/bin/env bash

. ./common.sh

DIR="data/kindle-wiki-en-cs"

cargo run --release --\
    -o $DIR \
    -f \
    -w data/enwiktionary-20250201-pages-articles.xml.bz2 -wp Czech \
    -t "English-Czech Dictionary (pejuko)" \
    -sl en \
    -tl cs \
    -a pejuko \
    && \
    wine "$KINDLEGEN" -c1 -gen_ff_mobi7 -dont_append_source $DIR/content.opf
