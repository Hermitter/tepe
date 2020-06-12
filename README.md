# Overview

Tepe is a CLI that lets you send messages and files over Telegram.

# TODO

- [x] Allow for `token` and multiple `chat` flags.
- [ ] Improve README.
- [ ] Proper error handling.
- [ ] Code cleanup.

# Installation

### 1. Download Tepe

Be sure you've [installed Rust](http://rustup.rs/) on your computer.

```bash
git clone https://github.com/Hermitter/tepe && cd tepe
cargo install --path .
```

### 2. Create Your Bot & Export The Bot Token

Talk to [@Botfather](https://t.me/botfather) and go through some dialog options until you've successfully created a bot. You should receive a token in the format of `123456789:blablabla`

Once your bot is created, export its token as an environmental variable.

```bash
# Unix-like
export TEPE_TELEGRAM_BOT_TOKEN=__Place_Bot_Token_Here__
```

```bash
# Windows
set TEPE_TELEGRAM_BOT_TOKEN=__Place_Bot_Token_Here__
```

### 3. Find Your Chat ID

Use the following command to have your bot wait for a user response. Once received, the bot will print the `chat_id` and exit.

```
tepe test
```

You can start talking to your bot by visiting (https://t.me/YOUR_BOT_NAME_HERE).

### 4. Send Messages

Export your `chat_id` by exposing it as an environmental variable

```bash
# Unix-like
export TEPE_TELEGRAM_CHAT_ID=__Place_Chat_Id_Here__
```

```bash
# Windows
set TEPE_TELEGRAM_CHAT_ID=__Place_Chat_Id_Here__
```

You're now ready to send messages!

```bash
# Example: $ tepe send ./taxes.txt -m "here are your taxes"

USAGE:
    tepe send [OPTIONS] [--] [files]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --chat <chat_ids>...    Telegram chat id.
    -m, --message <message>     String to pass into a Telegram message.
    -t, --token <token>         Telegram bot token.

ARGS:
    <files>...
```
