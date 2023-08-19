# azuki

## Prepare

- rustc, cargo (>= 1.71.1)

## Build

```bash
cargo build --release
# binary generated at ./target/release/azuki
```

## Run

### Freeze (Compress)

```bash
azuki freeze INPUT_FILENAME[.EXT]
# file generated at INPUT_FILENAME[.EXT].frozen
```

### Microwave (Extract)

```bash
azuki microwave INPUT_FILENAME[.EXT].frozen
# file generated at INPUT_FILENAME[.EXT]
```
