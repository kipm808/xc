[![Latest Release](https://img.shields.io/github/v/release/kipm808/xc?display_name=tag&sort=semver)](https://github.com/kipm808/xc/releases)

# xc

Copy file or stdin content to the system clipboard.

## Usage

```sh
# Copy a file
xc file.txt

# Copy from stdin
echo "hello" | xc
cat file.txt | xc
```

## Install

```sh
cargo install --path .
```

## Dependencies

- [arboard](https://github.com/1Password/arboard) — cross-platform clipboard access

