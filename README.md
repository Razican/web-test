# MySupport application

This repository contains the MySupport application. This repository contains the code used in the
"Rust for Modern Web Development" (tentative name) book.

## Building the frontend

To build the frontend, you will need to install Rust using [rustup.rs](https://rustup.rs/) and then
install the dependencies:

```bash
rustup target add wasm32-unknown-unknown &&\
cargo install trunk wasm-bindgen-cli &&\
trunk watch
```

You can also use `trunk build` for a one time build or `trunk build --release` to build an
optimized version of the frontend.

## Building the backend

Once Rust is installed, you just need to build / run the backend:

```bash
cargo run --bin backend
```

You can also pass the `--release` flag in order to build an optimized version of the backend. You
can then see the results in <http://127.0.0.1:8000>.

# Contributing

The chapter branches only accept contributions regarding fixes or dependency updates. Nevertheless,
the `main` branch accepts contributions for improved versions of the application. Feel free to fork
the repository and open a pull request.

# License

The code in this repository is licensed under either of the following, at your option:

- Apache License, Version 2.0, ([LICENSE-APACHE.md](LICENSE-APACHE.md) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT.md](LICENSE-MIT.md) or <https://opensource.org/licenses/MIT>)
