# gz-rs

[![crates.io](https://img.shields.io/crates/v/gz.svg)](https://crates.io/crates/gz)

Rust binding for Gazebo (not Gazebo Classic). Currently, the only supported combination is Gazebo Garden and Linux.

## Dependencies

- [Gazebo Garden](https://gazebosim.org/docs/garden/install)

## Examples

```no_rust
$ cargo run --example sub
Hello, world! 0
Hello, world! 1
Hello, world! 2
Hello, world! 3
Hello, world! 4
:
```

Execute the following in another terminal

```no_rust
$ cargo run --example pub
Publishing: 0
Publishing: 1
Publishing: 2
Publishing: 3
Publishing: 4
:
```