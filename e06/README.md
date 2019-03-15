https://cryptopals.com/sets/1/challenges/6

3. For each KEYSIZE, take the first KEYSIZE worth of bytes, and the second KEYSIZE worth of bytes, and find the edit distance between them. Normalize this result by dividing by KEYSIZE.

The reason this works is probably because if we assume that if the key is random, it scrambles the bits. Given aligned blocks, the bits are scrambled the same way, and the Hamming Distance would be the same as the ptext. BUT if we guess the wrong keysize, the non-aligned blocks would have higher distance because the distance of the misaligned keys are added to it.

See: https://crypto.stackexchange.com/questions/8115/repeating-key-xor-and-hamming-distance
