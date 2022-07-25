<img align="right" src="assets/favicon.png" width="150px" />

# Smithereens

Smithereens is a digested open-source data visualizer tool for your Smash results. 

## What Is the Plan for It?

The platform will make use of the [StartGG API](https://developer.smash.gg/docs/intro/) and, through separated online and offline sections, will show you:
- how well you usually perform (i.e., overperform, perform, or underperform).
- total # of wins,
- total # of losses,
- total # of wins by DQs,
- total # of losses by DQs,
- winrate,
- placements, and
- what competitor type you are (e.g., 0-2er, 1-2er, 2-2er, etc.).

Aside from player stats, Smithereens also allows you to see the hidden seeding for an event.

In the future, I hope to make it possible for you to claim your profile (i.e., via matching Twitch, Discord, or Twitter credentials from your StartGG account) to allow you to customize your profile and make it truly your own.

## What Can It Do Right Now?

Smithereens is not yet available as a website, only as a command-line tool. With it, you can:
- ✅ view the hidden seeding for an event, 
- ❎ view a player's overall results,
- ❎ view a 'digested' tournament result, and
- ❎ claim your profile, and make it your own.

> Note:
>> ✅: means that the feature has already been implemented.
>
>> ❎: means that the feature is yet to be implemented.

## Getting Started

Currently, the only way to get Smithereens on your computer is to build it from source. To do so, you need to have the Rust toolchain installed on your machine. To install it, follow instructions [here](https://www.rust-lang.org/tools/install).

After that, inside of Smithereens' repository root, run: `cargo build --release`. With that, you'll have Smithereens' binary (i.e., `smithe`) built under `target/release`. You can run it, like: `./target/release/smithe`. Afther that, feel free to move the binary to a more convenient place on your machine (i.e., perhaps to a place included in your `PATH`, so you can run it simply with `smithe`).

> Note: In the future, there will be releases for `smithe`, so this process will be better streamlined.