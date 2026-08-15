# dictionary CLI

A Rust binary that converts multiple sources (tab-delimited, pron files, Wiktionary XML) into a single Kindle `.mobi` dictionary.

## Running

Release builds are required — processing wiktionary XML is slow.

```sh
cargo run --release -- [args...]
```

See `cli_config.rs` for the argument parsing. Key flags: `-i` (tab file), `-w` (wiktionary, always pair with `-wp`), `-o` / `-ro` (output), `-s` (search), `-f` (force overwrite), `-p` (pronunciation), `-p US:data/en_US.txt` uses `:` as name/file separator, `-t`, `-a`, `-h`.

## Constraints

- Only the `en` → `cs` language pair is implemented (`dictionary/language.rs` returns `None` for unknown sources).
- The CLI checks that file paths passed to tab-file arguments exist; non-existent paths yield the error text "File does not exist".
- On Linux the `.mobi` step runs `kindlegen` (a Windows exe) via `wine`, see `convert-en-cs.sh` for the reference pattern.
- `data/` is the output directory and is gitignored.
- Word keys are always lowercased.
