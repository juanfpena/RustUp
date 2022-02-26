# The Rust Programming Language

## Hello, World!

A function is defined in a similar syntax as JS, using curly braces ("{}"). The function "main" is special, its the code that runs in every executable Rust program. It has no parameters and returns nothing.

```rs
fn main() {
    println!("Hello, world!");
}
```

The second line is the one which does all the work, as it prints text on the screen. Indentation between curly braces is done with 4 spaces, not with tab as in languages like Python. 

Notice the "!" at the end of the "println" procedure. This calls a "Rust macro", instead of a normal function. A normal function can be called with "println".

Expressions are ended with a semicolon "; ", which specifies separation between expressions. It's normal to use a semicolon after almost every line.

to run the program, run in shell:

```sh
$ rustc main.rs
```

this compiles main.rs into a binary executable file which is ran using 

```sh
$ ./main
```

executables can be thrown to someone who doesn't have rust installed and they can still be ran

## Cargo

Cargo is Rust's package managing tool. What we call libraries in Python, we call "dependencies" in Rust.

```rs
// check if Cargo's installed
cargo --version
```

### Creating a Cargo Project

Navigate to the projects directory and run

```sh
$ cargo new hello_cargo
$ cd hello_cargo
```

This creates a directory called "hello_cargo" (as that is what we named our new project). Cargo created two files and a directory:

* Cargo.toml
* src (which contains a *main.rs* file)

[TOML](https://github.com/toml-lang/toml) (*Tom's Obvious, Minimal Language*) is Cargo's configuration format. The file includes:

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

The first line, **[package]**, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections.

The next three lines set the configuration information Cargo needs to compile your program: the name, the version, and the edition of Rust to use.

The last line, **[dependencies]**, is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.

If we open *src/main.rs* we'll see the same code we created in the first hello world file

```rs
fn main() {
    println!("Hello, world!");
}
```

So far the differences between the file we wrote manually and the package we created is that Cargo placed the code inside the *src* folder and it created a TOML file. Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code. Using Cargo helps you organize your projects. 

If one starts a project without using Cargo, one can simply organize it by placing the *.rs* file inside an *src* folder and creating an appropriate *Cargo.toml* file.

### Building and Running a Cargo Project

Now let’s look at what’s different when we build and run the “Hello, world!” program with Cargo! From your hello_cargo directory, build your project by entering the following command:

```sh
$ cargo build
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 2.85 secs
```

This command creates an executable file in target/debug/hello_cargo (or target\debug\hello_cargo.exe on Windows) rather than in your current directory. You can run the executable with this command:

```sh
$ ./target/debug/hello_cargo
Hello, world!
```
