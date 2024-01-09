# Phantom Pulse

Phantom Pulse is a Rust-powered Discord bot designed to monitor your system resources via sysinfo on Discord.

## Commands

- [x] `!!sysinfo` - Displays system information, including your OS, processor, memory, and storage.
  
- [x] `!!usage` - Shows CPU and memory usage.
  
- [x] `!!uptime` - Reveals how long the system has been running.
  

## How To Run

1. Create a `keys.json` file inside the `config` folder.

2. Place your Discord API key inside the `keys.json` file:

    ```json
    {
        "discord_api_key": "Your Discord API key"
    }
    ```

3. To execute the project, type `cargo run --release` in the terminal or compile and run it in the same manner.

## Contribution

- Prior to creating a pull request, ensure code formatting with `cargo fmt`.
