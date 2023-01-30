## A discord bot for reading lines of code from github repositories

# Requirements

This project requires the [Rust compiler](https://www.rust-lang.org/).

# Project Setup

The following will clone the project into the child directory `discord_github_bot` in the current working directory.

```
git clone https://github.com/krogenth/discord_github_bot.git
cd discord_github_bot
```

Rename the `config.ron.example` to `config.ron`.

# project Build/Run

To build the project, run the command: `cargo build`.
To run the project, run the command: `cargo run`.

# Interaction

Create your own discord bot through the [discord developer portal](https://discord.com/developers/applications).

First, create a new application, then add a bot to the application.
The token for the bot can be revealed under the `BUILD-A-BOT` section.
This value should replace the `discord_token` value in `config.ron`.