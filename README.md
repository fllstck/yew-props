# Yew Props

An example web application build with [the Yew framework](https://yew.rs/docs/en/).

It illustrates props and nested components.

## ListGroup Component

This component uses Bootstrap's `list-group` class and accepts `ListGroupItem`'s as `children` prop.

## ListGroupItem Component

This component uses Bootstrap's `list-group-item` and `active` class and accepts `children` and `active` props.

## Bonus

It uses `wee_alloc` and some build optimizations in the `Cargo.toml` to shave about 25% of the binary size.

## Prerequisites

- [rustup](https://rustup.rs/)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/)

## Building

    $ wasm-pack build --target web --out-name wasm --out-dir ./static
