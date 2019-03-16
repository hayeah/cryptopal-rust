https://cryptopals.com/sets/2/challenges/9

PKCS#7 padding.

See: https://tools.ietf.org/html/rfc2315

```
01 -- if l mod k = k-1
02 02 -- if l mod k = k-2
.
.
.
k k ... k k -- if l mod k = 0
```
