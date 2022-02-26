# The Rust Programming Language

## Basic Usage

A function is defined in a similar syntax as JS, using curly brackets "{}". The function "main" is special, its the code that runs in every executable Rust program. It has no parameters and returns nothing.

```rs
fn main() {
    println!("Hello, world!");
}
```

this line does all the work, it prints text on the screen
Indentation is created with 4 spaces, not tab
println! calls a "Rust macro", a normal function would be defined as "println"
expressions are ended with a semicolon "; "

to run the program, run in shell:

```sh
rustc main.rs
```

this compiles main.rs into a binary executable file which is ran using 

```sh
./main
```

executables can be thrown to someone who doesn't have rust installed and they can still be ran

## Cargo

Cargo is Rust's package managing tool. What we call libraries in Python, we call "dependencies" in Rust.

```rs
// check if Cargo's installed
cargo --version
```

### Creating a Cargo project

Navigate to the projects directory and run

```sh
cargo new hello_cargo
cd hello_cargo
```

This creates a directory called "hello_cargo" (as that is what we named our new project). Cargo created two files and a directory:

* Cargo.toml
* src (which contains a *main.rs* file)

[TOML](https://github.com/toml-lang/toml) (*Tom's Obvious, Minimal Language*) is Cargo's configuration format. The file includes

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

The first line, ` `  ` [package] `  ` ` , is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use.

The last line, ` ` ` [dependencies] ` ` `, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.
