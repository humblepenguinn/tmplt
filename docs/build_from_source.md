# Build from source

Verify that you have Rust installed:

```sh
$ rustc --version
```

If `rust` is not installed, follow the instructions on [the Offical Rust website](https://www.rust-lang.org/tools/install).

Clone this repository:
```sh
$ git clone https://github.com/humblepenguinn/tmplt.git
$ cd tmplt
```

Build the project:

```sh
$ cargo build
```

Now, Check to see if it worked:
```sh
$ cargo run -- version
```

You can also install the project:

```sh
$ cargo install --path .
$ tmplt version
```






