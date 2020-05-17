# Discord

![build status](https://img.shields.io/github/workflow/status/jos-b/discord_api/Test & Lint?style=for-the-badge)
![Crates.io](https://img.shields.io/crates/v/discord_api?style=for-the-badge)
![GitHub](https://img.shields.io/github/license/jos-b/discord_api?style=for-the-badge)


Library and binary providing interaction with features of the Discord API.

- [Documentation](https://docs.rs/discord_api/)
- [Crates.io](https://crates.io/crates/discord_api)

To use this library head to the crates.io link above and copy the snippet to enter into your `Cargo.toml`.

## Examples

### Fetch guild name from invite

```rust
use discord_api::get_invite;

let invite = get_invite("python").await?;

println!("Invite for: {}", invite.guild?.name);
```

### Fetch guild features from invite

```rust
use discord_api::get_invite;

let invite = get_invite("python").await?;

let features = invite.guild?.features;

// do something with features!
```

More examples can be found in the command line utility which maintains feature parity with the library.

## Command line utility

### Feature Fetching

![invite features](https://raw.githubusercontent.com/jos-b/discord/master/screenshots/invite_features.png)

### Welcome Screen Fetching

![welcome screen](https://raw.githubusercontent.com/jos-b/discord/master/screenshots/invite_welcome.png)

## Usage

1. Clone the repository
2. Build with `cargo build`
3. Profit! Run the binary generated to see all available options
