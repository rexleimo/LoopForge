# Install & Update

## Recommended path

For most users, the safest order is:

1. install `loopforge`
2. run `loopforge --help`
3. run `loopforge init`
4. run `loopforge config validate`
5. run `loopforge doctor`

## Option A: Download a prebuilt binary (recommended)

1. Download the archive for your OS from GitHub Releases.
2. Extract it.
3. Put `loopforge` (or `loopforge.exe`) somewhere on your `PATH`.

Then:

```bash
loopforge --help
loopforge init
```

## Option B: Install from source (Cargo)

```bash
cargo install --path crates/loopforge-cli --locked
loopforge --help
```

## Update

- If you installed via Releases: download a newer archive and replace the binary.
- If you installed via Cargo: re-run `cargo install --path crates/loopforge-cli --locked`.
