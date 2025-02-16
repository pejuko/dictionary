#!/usr/bin/env bash

DIR="data/kindle-en-cs"

cargo run --release --\
    -i data/en-cs.txt \
    -o $DIR \
    -f \
    -p US:data/en_US.txt -p UK:data/en_UK.txt \
    -w data/enwiktionary-20250201-pages-articles.xml.bz2 -wp Czech \
    -t "English-Czech Dictionary (pejuko)" \
    -sl en \
    -tl cs \
    -a pejuko \
    && \
    wine ~/.wine/drive_c/users/pejuko/Local\ Settings/Application\ Data/Amazon/Kindle\ Previewer\ 3/lib/fc/bin/kindlegen.exe -c1 -gen_ff_mobi7 -dont_append_source $DIR/content.opf
