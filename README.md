# Ygglkan

Vulkan-based miner for Yggdrasil addresses

## Basic usage

```shell
cargo build --release && target/release/ygglkan -b 2048 -p
```

## Options

- -b, --batch-size <BATCH_SIZE> Block size. Each block has 64 keys [default: 1024]
- -p, --print-stats Print hashrate stats

## Benchmarks

- AMD Radeon RX 6800 XT: 3.0 MH/s (8192 batch)
