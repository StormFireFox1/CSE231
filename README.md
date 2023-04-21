# CSE 231

The programming language implementations for CSE 131/231, now in Rust!

Note this repo will contain all iterations, and tag each "final" release
of each programming language appropriately.

The current version of the language is Cobra.

## Usage

Note this repo is currently only functional without changes on M1 MacBooks.
To make it work on other machines, modify the Makefile as follows:
- change `nasm`'s target to `elf64`
- remove `rustc`'s target option

You might also need to install `nasm` from your own package manager.

If you are on an M1 MacBook, use the below code block to run and test
the programming language's implementation:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install nasm
cargo test -- --test-threads 1
```

## Credits

- [This Edstem post](https://edstem.org/us/courses/38748/discussion/2976772) yielded
a bugfix for one of the tests related to the "let" construct in the Boa version of the language.