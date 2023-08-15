# textmate-grammar-rs

WIP: Draft: NotYetImplemented: a Rust parser for TextMate grammar files.

All that can currently be done with this is run some tests which attempt to parse all the TextMate grammar files from [Shiki](https://github.com/shikijs/shiki).

To see the coverage:

```
cargo test
```

Currently 3 grammar files can't be parsed for mundane reasons which can be fixed with some tweaking.

38 additional grammar files can't be parsed because they contain regular expressions incompatible with Oniguruma, the regex engine used by TextMate grammars.

The remaining 132 grammar files are parsed successfully.

Only fields defined in the [TextMage language grammar definition](https://macromates.com/manual/en/language_grammars) are captured.  Non-standard fields are (currently) skipped.

