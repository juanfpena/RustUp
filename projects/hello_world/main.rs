// a function is defined in a similar syntax as JS, using curly brackets "{}"
// "main" is special, it's the code that runs in every executable Rust program
// it has no parameters and returns nothing
fn main() {
    println!("Hello, world!"); // this line does all the work, it prints text on the screen
}
// indentation is created with 4 spaces, not tab
// println! calls a "Rust macro", a normal function would be defined as "println"
// expressions are ended with a semicolon ";"

// to run the program, run in shell:
// ```rustc main.rs```
// this compiles main.rs into a binary executable file which is ran using ```./main```
// executables can be thrown to someone who doesn't have rust installed and they can still be ran
