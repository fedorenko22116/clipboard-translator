# clipboard-translator

Clipboard-translator is a cross-platform tool to translate text from clipboard and show it as native notification.
## Installation

Install rust && cargo

```bash
curl https://sh.rustup.rs -sSf | sh
```

Install and compile binary on your system

```bash
cargo install --git https://github.com/22116/clipboard-translator
```

## Usage

There is a main command `translate`

```bash
# clipboard-translator translate -h
clipboard-translator-translate 
Show translated text from clipboard as notification

USAGE:
  key-translator translate <primary-lang> [ARGS]

FLAGS:
  -h, --help       Prints help information
  -V, --version    Prints version information

ARGS:
  <primary-lang>      Target language to translate
  <secondary-lang>    Language used to fallback translation
  <translator>        Translator used. Available translators: 'Google'
```

Examples:

```bash
clipboard-translator en
clipboard-translator en ru
clipboard-translator en ru Google
```

## Configuring

Just add the command `clipboard-translator en` as a hotkey on your system

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)