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
azuki freeze -i INPUT_FILENAME [-o OUTPUT_FILENAME]
```

- short command name supported (`f`, `fr`, `fre`, `free` and `freez`)
- output defaults to `INPUT_FILENAME.frozen`

#### From stdin

```bash
azuki freeze [-o OUTPUT_FILENAME] < INPUT_FILENAME
```

- output defaults to stdout


### Microwave (Extract)

```bash
azuki microwave -i INPUT_FILENAME.frozen
```

- short command name supported (`m`, `mi`, `mic`, `micr`, `micro`, `microw`, `microwa` and `microwav`)
- output defaults to `INPUT_FILENAME.microwaved`

#### From stdin

```bash
azuki microwave [-o OUTPUT_FILENAME] < INPUT_FILENAME
```

- output defaults to stdout
