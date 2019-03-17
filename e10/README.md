https://cryptopals.com/sets/2/challenges/10

![](1202px-CBC_encryption.svg.png)

Implement AES128-CBC

encrypt:

```
mix = iv
ctext1 = aes_encrypt(mix ^ ptext1, key)

mix = ctext1
ctext2 = aes_encrypt(mix ^ ptext2, key)
...
```

decrypt

```
mix = iv
ptext1 = aes_decrypt(ctext1, key)
ptext1 = ptext1 ^ mix

mix = ctext1
ptext2 = aes_decrypt(ctext2, key)
ptext1 = ptext1 ^ mix
...
```

API

```rust
struct CTREncrypter {

}

let mut encrypter = CTREncrypter::new(iv, key)
encrypter.encrypt_block(&mut block)
encrypter.encrypt(&data) // pad data with PKCS7

let mut decrypter = CTRDecryper::new(iv)
decrypter.decrypt_block(&mut block)
decrypter.decrypt(&mut data) // unpad data with PKCS7
```
