Byte-at-a-time ECB decryption (Simple)

https://cryptopals.com/sets/2/challenges/12

```
AES-128-ECB(your-string || unknown-string, random-key)
```

# Random Prefix

In exercise 11, a random prefix is added

> Under the hood, have the function append 5-10 bytes (count chosen randomly)
> before the plaintext and 5-10 bytes after the plaintext.

```
AES-128-ECB(prefix || your-string || unknown-string, random-key)
```

To deal with the prefix, we choose this ptext prefix:

```
[prefix][rand(0..blocksize)]['1' * blocksize]['1' * blocksize]
```

If the second block and third block are the same, we've found alignment. Strip the first three blocks (i.e. prefix, padding, padding), and we can proceed with the byte-by-byte breaking.

# random suffix

To deal with the random suffix, maybe try to decode each byte twice. If n times are the same, then proceed to the next.

Otherwise end.

# byte-by-byte

Char 0:

```
['0' * 15][c0] [c1][c2][c3]...[c15]
```

Char 1:

```
['0' * 14][c0][c1]  [c2][c3]...[c15]
```

Char 15:

```
[c0][c1]  [c2][c3]...[c15]
```

Once we know one whole block (c0-c15), the next char in the next block(c16):

```
// 16 bytes
['0' * 15][c0]

// 16 bytes
[c1][c2][c3]...[c15][c16]

[c17][c18]...
```
