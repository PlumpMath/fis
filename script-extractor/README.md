# script-extractor

Extracts structured information from a movie script.

## Quickstart

You will need to have Rust and Cargo installed.  Use `multirust` if
your system packages are too old.

```
$ pdftohtml -xml <some-script>.pdf
$ cargo run <some-script>.xml
```

To compare the output of the extractor to the pages from the script,
use `compare.sh`:

```
$ cargo build
$ ./compare.sh path-to-script.pdf > compare.html
```

## Documentation

Either use the [online docs] or generate them offline using cargo:
```
$ cargo doc --no-deps --open
```

[online docs]: https://heylu.github.io/fis/docs/script_extractor/