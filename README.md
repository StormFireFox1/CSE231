# CSE 231

The programming language implementations for CSE 131/231, now in Rust!

Note this repo will contain all iterations, and tag each "final" release
of each programming language appropriately.

The current version of the language is Egg Eater (Extended for Green Snake).

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
- [This Edstem post](https://edstem.org/us/courses/38748/discussion/3020429) provided
useful tests for overflow issues
- Yuchen Jing provided me with the tip of checking for stack alignment, which prompted the fix
of ensuring the stack pointer was pushed to the stack before calling `snek_error`.
- Professor Politz for the discussion on printing cyclical tuples, and providing starter code for
the implementation of printing tuples from the Rust runtime, present
on [the GitHub repository for class starter code](https://github.com/ucsd-compilers-s23/lecture1/blob/b97536112db34a61c6fbd73edf39e6365e794e12/runtime/start.rs#L19C2-L37).
- Yuchen Jing for our discussion regarding the semantic interpretation of
structural equality that made the most sense, along with
[this Edstem post](https://edstem.org/us/courses/38748/discussion/3200017) for
clarifying the exact required implementation of structural equality boiling down to "observational equality”.
