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
    -n               Do not send the system notification with translated text
    -V, --version    Prints version information

ARGS:
    <primary-lang>      Target language to translate
    <secondary-lang>    Language used to fallback translation
    <translator>        Translator used. Available translators: 'Google'
    <selected>          Translate selected text instead of text from clipboard. Requires 'xsel' module on the system
                        to be installed
```

Examples:

```bash
clipboard-translator en
clipboard-translator en ru
clipboard-translator en ru Google
clipboard-translator en ru Google --selected
clipboard-translator en ru Google --selected --not-notify
```

## Configuring

Just add the command `clipboard-translator en` as a hotkey on your system

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)