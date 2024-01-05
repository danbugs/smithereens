<img align="right" src="docs/imgs/android-chrome-192x192.png" width="150px" />

# Smithereens

Smithereens is a digested open-source data visualizer tool for your Smash Ultimate results. 

## Why use Smithereens?

- Smithereens is accessible both via CLI, and web app.
- Smithereens is open-source, meaning there is no barrier to entry to contribute and improve it based on what you want to see in the app.
- Smithereens is not afilliated with any organization or sponsor, which I hope will increase the resilience of the project in the scene.
- Smithereens is free and there are no ads anywhere in the platform.
- Smithereens was thoroughly tested. First, by ensuring we're always at over 50% local test coverage, and, second, by running it via CLI non-stop for multiple days as 300 parallel jobs on a Kubernetes cluster without failure while aggregating player results. This allowed me to ensure we handle all edge cases (e.g., even load intensive requests for players like rm8, whose profile actually does not load on platforms similar to this).
- Smithereens has cool features like:
    - Twitter share (i.e., share your result on Twitter as a Twitter Card - no more need for screenshotting),
    - fun metrics (e.g., what competitor type you are (0-2er, 1-2er, 2-2er, etc.)), and
    - more!

## Installing the CLI

Please, check the [build from source pre-requisites section](#building-from-source-pre-requisites) before proceeding.

```sh
git clone https://github.com/danbugs/smithereens
cd smithereens
cargo build --release
cargo install --path .
smithe --help
```

## Contributing

Please, check the [build from source pre-requisites section](#building-from-source-pre-requisites) before proceeding.

To get setup for contributing, run:
```sh
git clone https://github.com/danbugs/smithereens
cd smithereens
cargo build --release
cargo test --package smithe_lib --no-default-features -- --exact --nocapture --test-threads=1
```

If all tests pass, you should be good to go!

## Building from source pre-requisites

- Rust toolchain with the `wasm32-unknown-unknown` target,
- Diesel-Rs,
- Trunk-Rs, and
- Postgres (system library).

> If you run into any issues while setting this up, please considering making a PR (or at least an issue) to improve this document and help avoid others having to go through the same hurdles you did. It's been a while since I had to get setup from scratch for this project, so this section could be missing required steps or items.
