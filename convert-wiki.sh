#!/usr/bin/env bash

BASE_DIR="data/kindle-wiki"
TIMESTAMP="$1"
#TIMESTAMP="20250301"

if [ -z "$TIMESTAMP" ]; then
    echo "Usage: convert-wiki.sh <WIKTIONARY TIMESTAMP>"
    echo "Example:"
    echo "    ./convert-wiki.sh 20250301"
    exit 1
fi

build() {
    local DIR=$1
    local REVERSE_DIR=$2

    wine ~/.wine/drive_c/users/pejuko/Local\ Settings/Application\ Data/Amazon/Kindle\ Previewer\ 3/lib/fc/bin/kindlegen.exe -c1 -gen_ff_mobi7 -dont_append_source "$DIR/content.opf"
    wine ~/.wine/drive_c/users/pejuko/Local\ Settings/Application\ Data/Amazon/Kindle\ Previewer\ 3/lib/fc/bin/kindlegen.exe -c1 -gen_ff_mobi7 -dont_append_source "$REVERSE_DIR/content.opf"
}

convert() {
    local SOURCE_LANGUAGE=$1
    local TARGET_LANGUAGE=$2
    local PATTERN=$3
    local TARGET_NAME=$4

    if [ -z "$TARGET_NAME" ]; then
        TARGET_NAME=$PATTERN
        TARGET_CODE=$TARGET_LANGUAGE
    else
        TARGET_CODE="$TARGET_LANGUAGE-${PATTERN,,}"
    fi

    local DIR="$BASE_DIR/$SOURCE_LANGUAGE-$TARGET_CODE/$TIMESTAMP"
    local REVERSE_DIR="$BASE_DIR/$TARGET_CODE-$SOURCE_LANGUAGE/$TIMESTAMP"

    cargo run --release --\
        -o $DIR \
        -ro $REVERSE_DIR \
        -f \
        -w data/enwiktionary-$TIMESTAMP-pages-articles.xml.bz2 -wp "$PATTERN" \
        -t "English-$TARGET_NAME Dictionary (pejuko)" \
        -rt "$TARGET_NAME-English Dictionary (pejuko)" \
        -sl $SOURCE_LANGUAGE \
        -tl $TARGET_LANGUAGE \
        -a pejuko \
        && \
        build $DIR $REVERSE_DIR

    if [ -f "$DIR/content.mobi" ]; then
        mv "$DIR/content.mobi" "$DIR/$SOURCE_LANGUAGE-$TARGET_CODE-pejuko-$TIMESTAMP.mobi"
    fi

    if [ -f "$REVERSE_DIR/content.mobi" ]; then
        mv "$REVERSE_DIR/content.mobi" "$REVERSE_DIR/$TARGET_CODE-$SOURCE_LANGUAGE-pejuko-$TIMESTAMP.mobi"
    fi
}

convert en cs Czech
convert en de German
convert en es Spanish
convert en sk Slovak
convert en zh Mandarin
convert en sh Cyrillic Serbo-Croatian
convert en sh Roman Serbo-Croatian
