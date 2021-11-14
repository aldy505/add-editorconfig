# add-editorconfig

Small and simple CLI app to generate .editorconfig based on a given settings.

## Usage

```sh
# Will create an .editorconfig in the current directory
$ add-editorconfig

# Will create an .editorconfig with the default config from .editorconfig that
# exists on the home directory.
$ add-editorconfig default

# Show help
$ add-editorconfig help

# Show current version
$ add-editorconfig version
```

## Installation

Yes, you'll need to build from source.

1. Clone this repository.
2. Make sure to have Rust in your system. At this point of time, I have v1.54.0.
3. Run `cargo build --all-targets --release`.
4. Run `sudo cp -v target/release/add-editorconfig /usr/local/bin`.
If you're not running in Linux, try to check how to move file on your own.
5. Check it via `add-editorconfig version`.

## License

[MIT](./LICENSE)
