# MD5 Implementation in Rust 🦀💥

This repository contains a custom implementation of [RFC 1321: _The MD5 Message-Digest Algorithm_ (1992)][rfc1321]
written in Rust based on [Wikipedia's pseudocode][wiki]. I prioritized **clarity and simplicity** over **performance and
security**.

## ⚠️ DO NOT USE THIS ⚠️

MD5 is **fatally broken** and **entirely unfit for purpose**.

Attacks are trivial using modern hardware.

This code must not be used.

---

If you're **absolutely positive** you need MD5, use an established implementation instead.

[rfc1321]: https://tools.ietf.org/html/rfc1321
[wiki]: https://en.wikipedia.org/wiki/MD5#Pseudocode
