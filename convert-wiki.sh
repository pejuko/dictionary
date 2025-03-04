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

convert() {
  local SOURCE_LANGUAGE=$1
  local TARGET_LANGUAGE=$2
  local PATTERN=$3

  local DIR="$BASE_DIR/$SOURCE_LANGUAGE-$TARGET_LANGUAGE/$TIMESTAMP"

  cargo run --release --\
    -o $DIR \
    -f \
    -w data/enwiktionary-$TIMESTAMP-pages-articles.xml.bz2 -wp "$PATTERN" \
    -t "English-$PATTERN Dictionary (pejuko)" \
    -sl $SOURCE_LANGUAGE \
    -tl $TARGET_LANGUAGE \
    -a pejuko \
    && \
    wine ~/.wine/drive_c/users/pejuko/Local\ Settings/Application\ Data/Amazon/Kindle\ Previewer\ 3/lib/fc/bin/kindlegen.exe -c1 -gen_ff_mobi7 -dont_append_source "$DIR/content.opf"

  if [ -f "$DIR/content.mobi" ]; then
    mv "$DIR/content.mobi" "$DIR/$SOURCE_LANGUAGE-$TARGET_LANGUAGE-pejuko-$TIMESTAMP.mobi"
  fi
}

convert en cs Czech
convert en de German
convert en es Spanish
convert en sk Slovak
convert en zh Mandarin

