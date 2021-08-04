# parrot

Middleware that accepts logs via `stdin` and redirects them to Telegram, based on a configurable set of conditions.

## Usage

```console
USAGE:
    parrot [OPTIONS] --telegram-api-key <telegram-api-key> --telegram-chat-id <telegram-chat-id>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --matcher <matcher>                      List of conditions to match against [default: deny_info,reorg,deny_snap]
        --telegram-api-key <telegram-api-key>    Telegram bot API key
        --telegram-chat-id <telegram-chat-id>    Telegram chat id where alerts should be sent
```
