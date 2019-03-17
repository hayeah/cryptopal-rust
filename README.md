Rust solutions to https://cryptopals.com/ challenges.

# Editor

The `aesni` package requires rustflags to be set. For VSCode to work, add:

```
"rust.rustflags": "-Ctarget-feature=+aes,+ssse3"
```

For cargo to work, add in `~/.cargo/config`:

```
[build] # or [target.$triple]
rustflags = ["-Ctarget-feature=+aes,+ssse3"]
```
