# boty_rs

A Discord bot that talks like a Peruvian friend (I'm Peruvian). Written in Rust.

## About

Boty is a side project I built to mess around with my friends on Discord. It sits in your server, reads the chat, and jumps into the conversation when someone mentions its name. It has a dry sense of humor, gives its opinion, and talks like a person in a group chat, not like an assistant.

It's not a product and it's not meant to be serious. If you want a bot that's fun to have around in a friends' server, you can run your own.

## Features

- Replies when a message contains `boty`, using the recent chat as context.
- `/hablar` slash command for one-off questions, and `/hola`.
- Per-server memory persisted to disk. Once the history grows past 30 messages, the oldest ones are summarized by the LLM so the bot keeps long-term context without an ever-growing prompt.
- Web search through [Tavily](https://tavily.com) when the model doesn't know something.
- Backed by the Gemini API. The LLM sits behind an `LLMProvider` trait, so other providers can be added.

## Requirements

- Rust 1.88 or newer (edition 2024, let chains)
- A Discord bot token
- A [Gemini API key](https://aistudio.google.com/apikey)
- A [Tavily API key](https://app.tavily.com) (the free tier is enough)

## Setup

### 1. Create the Discord bot

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications) and create a new application.
2. Under **Bot**, copy the token and enable **Message Content Intent**. Without it the bot can't read messages.
3. Under **OAuth2 → URL Generator**, select the scopes `bot` and `applications.commands`, plus the permissions **View Channels**, **Send Messages** and **Read Message History**.
4. Open the generated URL to invite the bot to your server.

### 2. Build

```sh
git clone https://github.com/andzzio/boty_rs.git
cd boty_rs
cargo build --release
```

### 3. Configure

```sh
cp Config.example.toml Config.toml
```

Edit `Config.toml` and fill in your keys:

```toml
[keys]
discord_token = "YOUR_DISCORD_BOT_TOKEN"
llm_token = "YOUR_GEMINI_API_KEY"
tavily_token = "YOUR_TAVILY_API_KEY"
```

### 4. Run

```sh
./target/release/boty_rs
```

By default it reads `Config.toml` from the current directory. To use another path:

```sh
./target/release/boty_rs --config-path /path/to/Config.toml
```

Slash commands are registered globally on startup, so they can take a little while to show up the first time.

## Usage

| Trigger | What it does |
| --- | --- |
| Any message containing `boty` | Boty reads the recent chat and replies in the channel |
| `/hablar <prompt>` | Asks Boty something directly, without chat history |
| `/hola [user]` | Greets you, or the given user |
| `hola`, `bye` / `chau` | Quick canned greetings |

## How memory works

Boty records every message it can see in a server, along with its own replies. History is stored per server in `history/<guild_id>/chat_state.json`, relative to the directory you run the bot from. When a server's history exceeds 30 messages, the oldest 15 are merged into a running summary by the LLM.

## Privacy

Everything Boty can read is stored in plain text in `history/`. Messages are also sent to Google (Gemini) as context, and search queries are sent to Tavily. Only add it to servers where everyone is fine with that.

## Deployment

Boty is a single long-running process that only makes outbound connections, so there are no ports to open. Run it however you prefer: a spare machine, a VPS, a container, a systemd service.

## License

[MIT](LICENSE)
