# Roc & Masonry-rs Experiment

An experiment to integrate [roc-lang](https://github.com/roc-lang/roc) and [masonry-rs](https://github.com/PoignardAzur/masonry-rs).

- I wanted to learn more about `cargo` and linking roc platforms. 
- I wanted to see if I could get something working that was *almost* as simple as `cargo run`

## Getting Started 

**Generate glue**

Note we use the `RustGlue.roc` spec from a parent directory here, you can copy this from [roc-lang/roc](https://github.com/roc-lang/roc/tree/main/crates/glue/src) instead of cloning the whole repository.

Note we use `platform/main-glue.roc` here as glue generation is a WIP and doesn't work correctly for the platform.

```
$ roc glue ../roc/crates/glue/src/RustGlue.roc platform/src/glue platform/main-glue.roc
🎉 Generated type declarations in:

	platform/src/glue
```

**Cargo to Build & Run**

Run with `cargo run`
Build with `cargo build`