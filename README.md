# Basic Base64 implementation in Rust

This repository holds the completed project accompanying the blogpost [Implementing Base64 from scratch in Rust](https://tiemenwaterreus.com/posts/implementing-base64-in-rust/). It is by no means a perfect implementation of Base64 but rather meant as a learning project. Feedback and PRs are welcomed 🙂

## Usage
This project is built and run using cargo.

**Compiling and running the binary**

```bash
# encoding
echo 'fluffy pancakes' | cargo run -- encode
> Zmx1ZmZ5IHBhbmNha2Vz

# and the reverse
echo 'Zmx1ZmZ5IHBhbmNha2Vz' | cargo run -- decode
> fluffy pancakes
```

**Running tests**

```bash
cargo test
```

## License
[MIT](./LICENSE)
