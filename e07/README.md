https://cryptopals.com/sets/1/challenges/7

In `.cargo/config`, add rust flags to enable CPU AES instructions:

```
[build] # or [target.$triple]
rustflags = ["-Ctarget-feature=+aes,+ssse3"]
```
