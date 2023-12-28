# Installation guide

## Installing Rust on your operating system

This Rust's official guide to installing Rust (and Cargo - the main Rust toolchain):
https://www.rust-lang.org/tools/install

There are also many other guides on the internet.

## Running the Lexical Analyzer

1) Add as many input files as you want in the *input* folder.
They must have a ".ou" extension. ex:  hello.ou 

2) Run the command `cargo run` from the root of the project. (cla/)
You will know you are in the root of the project if you can see the input, output, and src
folders.

3) Rust will compile and run, your output files should be available in the *output* folder.
Each output file corresponds to one output file, with the same name but a ".tok" extension.

