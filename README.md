Limonite
========

This is a compiler for the Limonite programming language.

Limonite is a relatively basic programming language written in rust using LLVM as a backend.

* Uses a custom tokenizer and parser.
* LLVM compilation to Rust source is being looked into.
* Syntax is by no means final. See sample.lim for examples.

## Building
1. Make sure you have installed all the dependencies.
	* Rust (nightly)
	* Cargo
	* git

2. Download and build Limonite

    Run the following commands

        git clone git@github.com:TheDan64/limonite.git
        cd limonite
        cargo build
