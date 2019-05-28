# Cracking the Coding Interview with Rust

This repository contains code for my learnings on Rust while solving some programming problems. The problems are based on the book "Cracking the Coding Interview" by Gayle Laakmann McDowell. You can find the existing solutions [here](https://github.com/careercup/CtCI-6th-Edition).

## Prerequisites

To run the code, you'll need an up-to-date version of Rust. The recommended way of
installing Rust is [using a tool called rustup](https://rustup.rs/):

```ShellSession
$ curl https://sh.rustup.rs -sSf | sh
```

Once you have Rust installed, you can build and run the tests.

### Running the Code

```ShellSession
$ cargo build
$ cargo test
```

You can run the tests for a specific question like so:

```ShellSession
$ cargo test --bin ch3-p2_stack-with-min
```

You can also run the tests for a specific library like so:

```ShellSession
$ cargo test --lib stack
```

## Credits:

The data structure implementations are largely influenced by the data structures in this book: [Hands on Data Structures and Algorithms with Rust book](https://github.com/PacktPublishing/Hands-On-Data-Structures-and-Algorithms-with-RUST). It's been very helpful reading that resource alongside CtCI.

There is another Rustacean who has tackled these problems in Rust [here](https://github.com/brndnmtthws/cracking-the-coding-interview-rust). He has a cool twitch channel where his effort was broadcasted live, and I found his solutions helpful to review while I was working through these problems. Although I think the solutions in this repo are more thorough and idiomatic.

The Linked Lists section in Chapter 2 was largely influenced by this article: https://rust-unofficial.github.io/too-many-lists/
