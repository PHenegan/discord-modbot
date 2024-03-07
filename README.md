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

The project uses [Shuttle](https://www.shuttle.rs), so for local development the project will
load variables from `Secrets.dev.toml`. When deploying to shuttle, secrets will go into
`Secrets.toml`. Your secrets file will go in the root directory, and will only have 1 line - the following:
```
DISCORD_TOKEN = 'your-token-here'
```

Now, you can build and run the program from any operating system with the following two commands
(Note - on Windows the executable might be called `discord-modbot.exe` instead of `discord-modbot`,
and the '/' will be replaced with '\\')
```
cargo build
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
- [Shuttle](https://www.shuttle.rs) - A deployment platform for rust with a free tier for community developers
