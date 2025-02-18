# Dictionary converter

## About

This app converts various sources into dictionary format suitable for e-readers.
Currently only kindle format is supported.

As an input you can specify tab delimited file, tab delimited pronunciation files
and wiktionary data. All sources can be specified at the same time and the app
will mix them in one dictionary.

The internal structure of the dictionary is "word class" -> "meaning" -> "translation".
However, kindle presentation is for clarity and more concise display organised differently.

## Where to get dictionary data

* *Wiktionary*

  Download latest English Wiktionary backup file from:
  [Wikimedia Downloads](https://dumps.wikimedia.org/backup-index.html)
  search for `enwiktionary`, click on it and on the page find `pages-articles.xml.bz2`
  file.

* *Pronunciation files*

  There is a project on github.com collecting pronunciation data. Go to
  [ipa-dict](https://github.com/open-dict-data/ipa-dict)

## How to convert generated files into Kindle .mobi file

For this you need to have
[Kindle Previewer](https://www.amazon.com/Kindle-Previewer/b?ie=UTF8&node=21381691011)
installed.

### Linux

Download Windows version and install it with `wine`.
Then you can take an inspiration in [[convert-en-cs.sh]](convert-en-cs.sh) file.

### MacOS

Download Mac version and run:
```sh
/Applications/Kindle\ Previewer\ 3.app/Contents/lib/fc/bin/kindlegen -c1 -gen_ff_mobi7 -dont_append_source data/kindle-en-cs/content.opf
```

### Windows

Download Windows version and run:
```sh
"C:\users\pejuko\Local Settings\Application Data\Amazon\Kindle Previewer 3\lib\fc\bin\kindlegen.exe" -c1 -gen_ff_mobi7 -dont_append_source data/kindle-en-cs/content.opf
```

## How to run the app

It is recommended to build the app in release mode.
Processing wiktionary data may be very slow.
On modern computers it takes around 3 minutes and another 2 minutes for kindlegen.

To generate English-Czech dictionary run:
```sh
cargo run --release -- -w data/enwiktionary.xml.bz2 -wp Czech -o data/kindle-en-cs -t "English-Czech dictionary" -a pejuko
```

## Getting help

Run:
```sh
cargo run -- -h
```

Or try to find inspiration in [convert-en-cs.sh](convert-en-cs.sh) file.

## License

Code and documentation: MIT

Dictionaries published here are available under the
[Creative Commons Attribution-ShareAlike License](https://creativecommons.org/licenses/by-sa/4.0/)
and are generated solely from wiktionary data.
