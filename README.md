# Deprecated: use faster OpenCL alternative: https://github.com/averyanalex/yggdrasil-vanity

# Ygglkan

Vulkan-based miner for Yggdrasil addresses

## Basic usage

```shell
cargo run --release -- -b 2048 -s
```

## Regex matching

Pass -r "regex" argument (you can do it multiple times of you want search for multiple patterns) to search only for adresses matching given regex.

Example:

```shell
ygglkan -r "" -r "^([0-9a-f]*:){2}:" -r "^([0-9a-f]*:){2}[0-9a-f]{0,2}:0:" -r "^([0-9a-f]*:){3}0:" -r "1234:5678"
```

## Benchmarks

- AMD Radeon RX 6800 XT: 3.0 MH/s (8192 batch)
