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
    clipboard-translator translate [FLAGS] <primary-lang> [ARGS]

FLAGS:
    -c, --copy          Copy translated text to clipboard
    -h, --help          Prints help information
    -n, --not-notify    Do not send the system notification with translated text
        --selected      Translate selected text instead of text from clipboard. Requires 'xsel' module on the system to
                        be installed
    -V, --version       Prints version information

ARGS:
    <primary-lang>      Target language to translate
    <secondary-lang>    Language used to fallback translation (Used as source language for 'MyMemory' translator
                        type)
    <translator>        Translator used. Available translators: 'Google', 'MyMemory'
```

Examples:

```bash
clipboard-translator translate en
clipboard-translator translate en ru
clipboard-translator translate en ru Google
clipboard-translator translate en ru Google --selected
clipboard-translator translate en ru Google --selected --not-notify
clipboard-translator translate en ru MyMemory --copy
```

## Configuring

Just add the command `clipboard-translator translate en` as a hotkey on your system

### Translators

* Google (Internal API)
* [MyMemory](https://mymemory.translated.net/)

## Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

## License
[MIT](https://choosealicense.com/licenses/mit/)