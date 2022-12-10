# grams

<img src="assets/logo.png" width="300px">

Welcome to the `grams` repository!

# 🌈 What is `grams`?

Grams desktop app and way to mainline `chat.openai.com` into you're day to day life. 

`grams` was originally a tool I built for myself to so could lower the effort needed to integrate this awesome powerful semi-intelligent AI into my daily life. I hope you find it useful too!

## 💦 Features
- [X] Hotkey to bring app to front `cmd + '`
- [X] Record chat logs
- [X] Felt cute - adds the dark mode I wanted

## 👆 Logo
Above is the `grams` logo (it was generated by DALLE by asking for _"a colorful abstract logo as a 3d render that invokes happiness"_)

## 📸 Pics or it didn't happen

### Hotkey to bring app to front
_just press `cmd + '` to bring the app to front_

<img src="assets/hotkey-focus.gif">

### Log chat to local file
<img src="assets/log-chat.gif">


## 🏁 Getting Started

### The easy way (for users)

```bash
# for Intel Macs
curl -s -O -L \
https://github.com/drbh/grams/releases/download/beta/Grams.0.0.1.dmg && \
open Grams.0.0.1.dmg

# for Apple Silicon Macs
curl -s -O -L \
https://github.com/drbh/grams/releases/download/beta/Grams.0.0.1-amd64.dmg && \
open Grams.0.0.1-amd64.dmg
```

### The hard way (for developers)


_note: you need to have `cargo` installed to use this tool and it's only targeting OSX (it might work for other operating systems but is not tested)_

```bash
cargo run
```

### 🤷‍♀️ FAQ

1. Google login does not work due to OAuth issues. OpenAI accounts work fine.
2. Need to require `Accessiblity` permissions to work properly. This enables the hot-keys. For now you need to manually enable it in `System Preferences > Security & Privacy > Privacy > Accessibility` (you should be prompted to do this when you install the app)
3. `CTRL + C` and other hotkeys do not work yet in the chat window. You can right click and copy/paste for now.
4. Chat logs are not formatted and may miss `try again` messages. This is a known issue and will be fixed with a more comprehensive logging solution. For now the logs are `txt` files in the `~/Application Support/Grams` directory.