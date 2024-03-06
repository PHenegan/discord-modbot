# Discord-ModBot
(creative name, I know)

A Discord server I'm in keeps getting those scam links to other discord servers.
I didn't think it would be very hard to make a bot which checks messages for discord links,
so here we are.

I may keep adding features to this, but it isn't really intended to be used at scale.

## Installation

Rust 1.76 is required for this project. The easiest way to manage rust versions
is using `rustup`. See guides for that program on installing specific versions of a rust toolkit.

A discord bot token is also required, and can be obtained from Discord's Developer Portal.

The token will be read from the environment variable `DISCORD_TOKEN`.
To set this environment variable and start the discord bot, you can run the following
commands in a Linux/MacOS shell to compile the program and get the binary executable.
```
cargo build
export DISCORD_TOKEN=paste_here
./target/discord-modbot
```

## Notes
I've only tested this on Linux, the commands above might be slightly different on Windows,
but you can use Rustup and Cargo on Windows, so the only thing that would change is the executable
name (it would end in `.exe`) and maybe the command for setting environment variables.

## Projects to Check Out
This bot relies on these libraries to function, they're worth checking out!
- [Serenity](https://github.com/serenity-rs/serenity) - A library for interacting with the Discord API from Rust
- [Poise](https://github.com/serenity-rs/poise) - A library built around Serenity for making commands easily
