# Egg timer

This is a demo project for my tutorial talk from at [Rust Nation '24](https://www.rustnationuk.com/).

It is a countdown timer app, to demonstrate how [Crux](https://github.com/redbadger/crux/) approaches testing
side-effects using the real time "wall clock" needed for the countdown.

## Slides

The slides from the talk are [available as PDF](./slides.pdf) in this repo, and provide background to the demo.

## Get in touch

The best way to get in touch with questions is via the [Crux community Zulip chat](http://crux-community.zulipchat.com/).

## Get started

You will need:

- Rust. There's a `rust-toolchain.toml`, so necessary targets should install for you
- `pnpm` for the web shell - see [installation options](https://pnpm.io/installation)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/installer/)

## Branches

### main branch

The `main` branch of this repo is a starting point for the tutorial, where
all the boilerplate setup has been done, and a static UI has been prepared
for the timer.

To run the demo, you need to run the type generation from the `shared_types`
folder

```sh
cd shared_types && cargo build
```

You can then run the web UI by going into the web folder and runing

```sh
pnpm i
pnpm run dev
```

### wip branch

To step through the demo, check out the commits on the `wip` branch, one by one. You can also read the changes on [PR #1](https://github.com/charypar/rust-nation-2024-egg-timer/pull/1). Most of the commits are focused on the core of the app in a `shared/src/app.rs`,
except for the very last one, which links the UI to the core.

Every step of the way, tests of the `shared` crate verify that that the changes made are working.
