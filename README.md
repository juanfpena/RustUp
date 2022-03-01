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

First run the following command:

```sh
$ rustc main.rs
```

This compiles *main.rs* into a binary executable file which is ran using:

```sh
$ ./main
```

Executables can be thrown to someone who doesn't have rust installed and they can still be ran

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

Running cargo build for the first time also causes Cargo to create a new file at the top level: *Cargo.lock*. This file keeps track of the exact versions of dependencies in your project. This project doesn’t have dependencies, so the file is a bit sparse. You won’t ever need to change this file manually; Cargo manages its contents for you.

We just built a project with **cargo build** and ran it with **./target/debug/hello_cargo**, but we can also use **cargo run** to compile the code and then run the resulting executable all in one command:

```sh
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/hello_cargo`
Hello, world!
```

Cargo also provides a command called **cargo check**. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```sh
$ cargo check
   Checking hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32 secs
```

Why would you not want an executable? Often, **cargo check** is much faster than cargo build, because it skips the step of producing an executable.

### Building for Release

When your project is finally ready for release, you can use **cargo build --release** to compile it with optimizations. This command will create an executable in *target/release* instead of *target/debug*. The optimizations make your Rust code run faster, but turning them on lengthens the time it takes for your program to compile. This is why there are two different profiles: one for development, when you want to rebuild quickly and often, and another for building the final program you’ll give to a user that won’t be rebuilt repeatedly and that will run as fast as possible. If you’re benchmarking your code’s running time, be sure to run **cargo build --release** and benchmark with the executable in *target/release*.

## Guessing Game

We can now create our first interactive Rust program. Our program will create a random number and the user will take their guess for which number it is. The program will then notify the user if their assumption is correct or not.

```rs
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
```

The first part of our program consists of asking the user for an input, processing that input and verifying if such input was provided in the expected form.

First things first. To obtain user input and then print the result as output, we need to bring the **io** input/output library into scope. The **io** library comes from the standard library, known as **std**:

```rs
use std::io;
```

Rust tries to keep at a minimum the amount of pre loaded packages, though its also not efficient to make the user import every single type of data they will use. As such, Rust offers the prelude dependency (**std::prelude**), which is automatically imported every time and contains a series of [useful features](https://doc.rust-lang.org/stable/std/prelude/index.html). If a set of features is not included in the **std::prelude** library, one has to import it manually (as we did with **std::io**).

### Storing Values with Variables

Jumping over the *println!()* procedure that was already explained previously, the next step is to create a variable for storing the users input, like so:

```rs
let mut guess = String::new();
```

The **let** statement is used to create a variable, for example:

```rs
let apples = 5;
```

In Rust, variables are immutable by default; so if we want a variable to be mutable, we can add **mut** before the variable name.

Now returning to our specific case, we created a new mutable variable called *guess*, which is bound to a new, empty instance of a **String**, a UTF-8 encoded bit of text. The **::** syntax in **::new** line indicates that **new** is an associated function of the **String** type. An associated function is a function that’s implemented on a type, in this case **String**. This **new** function creates a new, empty string. 

After creating the variable, what's next is handling the user input, thats done with **io::stdin()**:

```rs
io::stdin()
    .read_line(&mut guess)
```

If **std::io** wasn't imported at the start of the file, we could still use **stdin** if we call it as **std::io::stdin**. This function returns an instance of **std::io:: Stdin**, which is a type that represents a handle to the standard input for your terminal.

Next, the line **.read_line(&mut guess)** calls the **read_line** method on the standard input handle to get input from the user. We’re also passing &mut guess as the argument to **read_line** to tell it what string to store the user input in. The full job of **read_line** is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content.

The **&** indicates that this argument is a *reference*, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References, same as variables, are mutable by default, so it's necessary to write **&mut guess** rather than **&guess** to make it work.

### Handling Potential Failure with the *Result* Type

We’re still working on this line of code. Although we’re now discussing a third line of text, it’s still part of a single logical line of code. The next part is this method:

```rs
.expect("Failed to read line");
```

We could have written this code as:

```rs
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

Though its often good practice to separate **.method_name()** expressions in several lines.

As mentioned earlier, **read_line** puts whatever the user enters into the string we pass to it, but it also returns a value—in this case, an **io:: Result**. Rust has a number of types named **Result** in its standard library: a generic Result as well as specific versions for submodules, such as **io:: Result**. The **Result** types are *enumerations*, often referred to as *enums*, which can have a fixed set of possibilities known as variants. Enums are often used with **match**, a conditional that makes it convenient to execute different code based on which variant an enum value is when the conditional is evaluated.

Result’s variants are **Ok** or **Err**. The **Ok** variant indicates the operation was successful, and inside **Ok** is the successfully generated value. The **Err** variant means the operation failed, and **Err** contains information about how or why the operation failed.

Values of the **Result** type, like values of any type, have methods defined on them. An instance of **io:: Result** has an **expect** method that you can call. If this instance of **io:: Result** is an **Err** value, expect will cause the program to crash and display the message that you passed as an argument to expect. If the read_line method returns an **Err**, it would likely be the result of an error coming from the underlying operating system. If this instance of **io:: Result** is an **Ok** value, expect will take the return value that **Ok** is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in the user’s input.

If you don’t call **expect**, the program will compile, but you’ll get a warning:

```sh
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_must_use)]` on by default
   = note: this `Result` may be an `Err` variant, which should be handled

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust warns that you haven’t used the **Result** value returned from **read_line**, indicating that the program hasn’t handled a possible error.

The right way to suppress the warning is to actually write error handling, but in our case we just want to crash this program when a problem occurs, so we can use **expect**. 

### Printing Values with println! Placeholders

Aside from the closing curly bracket, there’s only one more line to discuss in the code so far:

```rs
println!("You guessed: {}", guess);
```

This line prints the string that now contains the user’s input. The {} set of curly brackets is a placeholder. One can use as many curly braces as they want, given that the first set correspond to the first variable and so on.

### Testing the First Part

Let’s test the first part of the guessing game with **cargo run**:

```sh
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
Guess the number!
Please input your guess.
6
You guessed: 6
```

At this point, the first part of the game is done: we’re getting input from the keyboard and then printing it.

### Generating a Secret Number

Then we'll need to be able to generate a random number. The standard library does not offer such functionality, though the Rust team offers us a **rand** crate that does the job. Before we can write code that uses **rand**, we need to modify the *Cargo.toml* file to include the **rand** crate as a dependency.

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.3"
```

In this case, we specify the **rand** crate with the semantic version specifier **0.8.3**. Cargo understands [Semantic Versioning](https://semver.org/) (sometimes called SemVer), which is a standard for writing version numbers. The number **0.8.3** is actually shorthand for **^0.8.3**, which means any version that is at least **0.8.3** but below **0.9.0**. Cargo considers these versions to have public APIs compatible with version **0.8.3**, and this specification ensures you’ll get the latest patch release that will still compile with the code in this chapter. Any version **0.9.0** or greater is not guaranteed to have the same API as what the following examples use.

Now, without changing any of the code, let’s build the project:

```sh
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.3
  Downloaded libc v0.2.86
  Downloaded getrandom v0.2.2
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.10
  Downloaded rand_chacha v0.3.0
  Downloaded rand_core v0.6.2
   Compiling rand_core v0.6.2
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

When we include an external dependency, Cargo fetches the latest versions of everything that dependency needs from the *registry*, which is a copy of data from [Crates.io](Crates.io). Crates.io is where people in the Rust ecosystem post their open source Rust projects for others to use.

After updating the registry, Cargo checks the **[dependencies]** section and downloads any crates listed that aren’t already downloaded. In this case, although we only listed **rand** as a dependency, Cargo also grabbed other crates that **rand** depends on to work. After downloading the crates, Rust compiles them and then compiles the project with the dependencies available.

Cargo has a mechanism that ensures you can rebuild the same artifact every time you or anyone else builds your code: Cargo will use only the versions of the dependencies you specified until you indicate otherwise. For example, say that next week version 0.8.4 of the **rand** crate comes out, and that version contains an important bug fix, but it also contains a regression that will break your code. To handle this, Rust creates the *Cargo.lock* file the first time you run **cargo build**, so we now have this in the *guessing_game* directory.

When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the *Cargo.lock* file. When you build your project in the future, Cargo will see that the *Cargo.lock* file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically. In other words, your project will remain at **0.8.3** until you explicitly upgrade, thanks to the *Cargo.lock* file.

When you do want to update a crate, Cargo provides the command **update**, which will ignore the Cargo.lock file and figure out all the latest versions that fit your specifications in Cargo.toml. Cargo will then write those versions to the Cargo.lock file. Otherwise, by default, Cargo will only look for versions greater than **0.8.3** and less than **0.9.0**. If the **rand** crate has released the two new versions **0.8.4** and **0.9.0** you would see the following if you ran **cargo update**:

```sh
$ cargo update
    Updating crates.io index
    Updating rand v0.8.3 -> v0.8.4
```

Cargo ignores the **0.9.0** release. At this point, you would also notice a change in your *Cargo.lock* file noting that the version of the **rand** crate you are now using is **0.8.4**. To use **rand** version **0.9.0** or any version in the **0.9.x** series, you’d have to update the Cargo.toml file to look like this instead:

```toml
[dependencies]
rand = "0.9.0"
```

The next time you run **cargo build**, Cargo will update the registry of crates available and reevaluate your **rand** requirements according to the new version you have specified.

The next step is to update src/main.rs, as shown:

```rs
use std::io;
use rand::Rng; // we add the rand crate

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101); // we generate a secret number

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}

```

First, we add the line **use rand:: Rng**. The **Rng** trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.

Next, we’re adding two lines in the middle. In the first line, we call the **rand::thread_rng** function that gives us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the **gen_range** method on the random number generator. This method is defined by the **Rng** trait that we brought into scope with the **use rand:: Rng** statement. The **gen_range** method takes a range expression as an argument and generates a random number in the range. The kind of range expression we’re using here takes the form **start..end** and is inclusive on the lower bound but exclusive on the upper bound, so we need to specify **1..101** to request a number between 1 and 100. Alternatively, we could pass the range **1..=100**, which is equivalent.

Note: You won’t just know which traits to use and which methods and functions to call from a crate, so each crate has documentation with instructions for using it. Another neat feature of Cargo is that running the **cargo doc --open** command will build documentation provided by all of your dependencies locally and open it in your browser. If you’re interested in other functionality in the **rand** crate, for example, run **cargo doc --open** and click **rand** in the sidebar on the left.

We'll then want to match our guess to the randomly generated number:

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --snip--

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
```

First we add another **use** statement, bringing a type called **std::cmp:: Ordering** into scope from the standard library. The **Ordering** type is another enum and has the variants **Less**, **Greater**, and **Equal**. These are the three outcomes that are possible when you compare two values.

Then we add five new lines at the bottom that use the **Ordering** type. The **cmp** method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing the **guess** to the **secret_number**. Then it returns a variant of the **Ordering** enum we brought into scope with the use statement. We use a **match** expression to decide what to do next based on which variant of **Ordering** was returned from the call to **cmp** with the values in **guess** and **secret_number**.

A **match** expression is made up of *arms*. An arm consists of a *pattern* to match against, and the code that should be run if the value given to **match** fits that arm’s pattern. Rust takes the value given to **match** and looks through each arm’s pattern in turn. Patterns and the **match** construct are powerful Rust features that let you express a variety of situations your code might encounter and make sure that you handle them all. 

Let’s walk through an example with the **match** expression we use here. Say that the user has guessed 50 and the randomly generated secret number this time is 38. When the code compares 50 to 38, the **cmp** method will return **Ordering:: Greater**, because 50 is greater than 38. The **match** expression gets the **Ordering:: Greater** value and starts checking each arm’s pattern. It looks at the first arm’s pattern, **Ordering:: Less**, and sees that the value **Ordering:: Greater** does not match Ordering:: Less, so it ignores the code in that arm and moves to the next arm. The next arm’s pattern is **Ordering:: Greater**, which does match **Ordering:: Greater**! The associated code in that arm will execute and print Too big! to the screen. The **match** expression ends because it has no need to look at the last arm in this scenario.

However, the code in Listing 2-4 won’t compile yet. Let’s try it:

```sh
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.3
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> src/main.rs:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                     ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |
   = note: expected reference `&String`
              found reference `&{integer}`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```

The core of the error states that there are *mismatched types*. Rust has a strong, static type system. However, it also has type inference. When we wrote **let mut guess = String::new()**, Rust was able to infer that **guess** should be a **String** and didn’t make us write the type. The **secret_number**, on the other hand, is a number type. A few of Rust’s number types can have a value between 1 and 100: **i32**, a 32-bit number; **u32**, an unsigned 32-bit number; **i64**, a 64-bit number; as well as others. Unless otherwise specified, Rust defaults to an **i32**, which is the type of **secret_number** unless you add type information elsewhere that would cause Rust to infer a different numerical type. The reason for the error is that Rust cannot compare a string and a number type.

Ultimately, we want to convert the **String** the program reads as input into a real number type so we can compare it numerically to the secret number. We do so by adding this line to the **main** function body:

```rs
    // --snip--

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
```

The line is:

```rs
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

We create a variable named **guess**. Rust allows us to *shadow* the previous value of **guess** with a new one. Shadowing lets us reuse the **guess** variable name rather than forcing us to create two unique variables, such as **guess_str** and **guess** for example. We’ll cover this in more detail in Chapter 3, but for now know that this feature is often used when you want to convert a value from one type to another type.

We bind this new variable to the expression **guess.trim().parse()**. The **guess** in the expression refers to the original **guess** variable that contained the input as a string. The trim method on a **String** instance will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the **u32**, which can only contain numerical data. The user must press enter to satisfy read_line and input their guess, which adds a newline character to the string. For example, if the user types 5 and presses enter, **guess** looks like this: **5\n**.

The **parse** method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using **let guess: u32**. The colon (**:**) after **guess** tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the **u32** seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number. You’ll learn about other number types in Chapter 3. Additionally, the **u32** annotation in this example program and the comparison with **secret_number** means that Rust will infer that **secret_number** should be a **u32** as well. So now the comparison will be between two values of the same type!

The **parse** method will only work on characters that can logically be converted into numbers and so can easily cause errors. If, for example, the string contained **A👍%**, there would be no way to convert that to a number. Because it might fail, the **parse** method returns a **Result** type, much as the **read_line** method does (discussed earlier in “Handling Potential Failure with the **Result** Type”). We’ll treat this **Result** the same way by using the expect method again. If **parse** returns an **Err** **Result** variant because it couldn’t create a number from the string, the expect call will crash the game and print the message we give it. If **parse** can successfully convert the string to a number, it will return the **Ok** variant of **Result**, and expect will return the number that we want from the **Ok** value.

Let’s run the program now:

```sh
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 58
Please input your guess.
  76
You guessed: 76
Too big!

```

### Allowing Multiple Guesses with Looping

The **loop** keyword creates an infinite loop. We’ll add a loop to give users more chances at guessing the number:

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    // --snip--

    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");

        // --snip--

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
}
```

As you can see, we’ve moved everything from the guess input prompt onward into a loop. Be sure to indent the lines inside the loop another four spaces each and run the program again. The program will now ask for another guess forever, which actually introduces a new problem. 

The user could always interrupt the program by using the keyboard shortcut ctrl-c. But there’s another way to escape this insatiable monster, as mentioned in the **parse** discussion in “Comparing the Guess to the Secret Number”: if the user enters a non-number answer, the program will crash. We can take advantage of that to allow the user to quit, as shown here:

```sh
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 59
Please input your guess.
45
You guessed: 45
Too small!
Please input your guess.
60
You guessed: 60
Too big!
Please input your guess.
59
You guessed: 59
You win!
Please input your guess.
quit
thread 'main' panicked at 'Please type a number!: ParseIntError { kind: InvalidDigit }', src/main.rs:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

Typing **quit** will quit the game, but as you’ll notice so will entering any other non-number input. This is suboptimal to say the least; we want the game to also stop when the correct number is guessed.

Let’s program the game to quit when the user wins by adding a **break** statement:

```rs
        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

Adding the **break** line after **You win!** makes the program exit the loop when the user guesses the secret number correctly. Exiting the loop also means exiting the program, because the loop is the last part of **main**.

To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, let’s make the game ignore a non-number so the user can continue guessing:

```rs
        // --snip--

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // --snip--
```

We switch from an **expect** call to a **match** expression to move from crashing on an error to handling the error. Remember that **parse** returns a **Result** type and **Result** is an **enum** that has the variants **Ok** or **Err**. We’re using a match expression here, as we did with the **Ordering** result of the **cmp** method.

If **parse** is able to successfully turn the string into a number, it will return an **Ok** value that contains the resulting number. That **Ok** value will match the first arm’s pattern, and the **match** expression will just return the **num** value that **parse** produced and put inside the **Ok** value. That number will end up right where we want it in the new **guess** variable we’re creating.

If **parse** is not able to turn the string into a number, it will return an **Err** value that contains more information about the error. The **Err** value does not match the **Ok(num)** pattern in the first match arm, but it does match the **Err(_)** pattern in the second arm. The underscore, **_**, is a catchall value; in this example, we’re saying we want to match all **Err** values, no matter what information they have inside them. So the program will execute the second arm’s code, **continue**, which tells the program to go to the next iteration of the **loop** and ask for another **guess**.

Now everything in the program should work as expected:

```sh
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
Guess the number!
The secret number is: 61
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
99
You guessed: 99
Too big!
Please input your guess.
foo
Please input your guess.
61
You guessed: 61
You win!
```

With one tiny final tweak, we will finish the guessing game. Recall that the program is still printing the secret number. That worked well for testing, but it ruins the game. Let’s delete the println! that outputs the secret number:

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Variables and Mutability

As mentioned before, by default, variables are immutable. When a variable is immutable, once a value is bound to a name, that value can't be changed. This'll be illustrated in a directory called *variables*.

The *main.rs* file in *variables* contains the following code:

```rs
fn main() {

    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

When ran with **cargo run**, this program raises an error:

```sh
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

The error is clear, we "cannot assign twice to immutable variable", and points out were *x* was already bound to another value.

The idea behind immutability is to make code easier to reason through. If one part of the code operates on the idea that a variable will always have the same value, and another part of the code changes such value, it's possible that the first part won't work as intended.

However mutability can be useful. One can make them mutable by **mut** in front of the variable name. Adding **mut** also conveys intent to future readers of the code by indicating that other parts of the code will be changing this variable’s value.

If we modify the code in *main.rs* as intended:

```rs
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

We can then run the code with the expected output:

```sh
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
The value of x is: 5
The value of x is: 6
```

There are multiple trade-offs to consider in addition to the prevention of bugs. For example, in cases where you’re using large data structures, mutating an instance in place may be faster than copying and returning newly allocated instances. With smaller data structures, creating new instances and writing in a more functional programming style may be easier to think through, so lower performance might be a worthwhile penalty for gaining that clarity.

### Constants

Like immutable variables, *constants* are values that are bound to a name and are not allowed to change, but there are a few differences between constants and variables.

First, you aren’t allowed to use **mut** with constants. Constants aren’t just immutable by default, they’re always immutable. You declare constants using the **const** keyword instead of **the** let keyword, and the type of the value *must* be annotated.

Constants can be declared in any scope, including the global scope, which makes them useful for values that many parts of code need to know about.

The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.

Example given:

```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

Rust’s naming convention for constants is to use all uppercase with underscores between words. 

Constants are valid for the entire time a program runs, within the scope they were declared in. This property makes constants useful for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn or the speed of light.

### Shadowing

As seen in the guessing game before, you can declare a new variable with the same name as a previous variable. It's said that the first variable is shadowed by the second, which means that the second variable’s value is what the program sees when the variable is used. We can shadow a variable by using the same variable’s name and repeating the use of the **let** keyword as follows:

```rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);
}
```

This program first binds **x** to a value of **5**. Then it shadows **x** by repeating **let x =**, taking the original value and adding **1** so the value of **x** is then **6**. Then, within an inner scope, the third let statement also shadows x, multiplying the previous value by **2** to give **x** a value of **12**. When that scope is over, the inner shadowing ends and **x** returns to being **6**. When we run this program, it will output the following:

```sh
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

Shadowing is different from marking a variable as **mut**, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the **let** keyword. By using **let**, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed.

The other difference between **mut** and shadowing is that because we’re effectively creating a new variable when we use the **let** keyword again, we can change the type of the value but reuse the same name. For example, say our program asks a user to show how many spaces they want between some text by inputting space characters, and then we want to store that input as a number:

```rs
    let spaces = "   ";
    let spaces = spaces.len();
```

The first **spaces** variable is a string type and the second **spaces** variable is a number type. Shadowing thus spares us from having to come up with different names, such as **spaces_str** and **spaces_num**; instead, we can reuse the simpler **spaces** name. However, if we try to use **mut** for this, as shown here, we’ll get a compile-time error:

```rs
    let mut spaces = "   ";
    spaces = spaces.len();
```

The error says we’re not allowed to mutate a variable’s type:

```sh
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

## Data Types

Rust is a statically typed language, which means that it must know the types of all variables at compile time. The compiler can usually infer what type we want to use based on the value and how we use it.

If we recall our guessing game, we had to specify the type of **guess**:

```rs
let guess: u32 = "42".parse().expect("Not a number!");
```

If we don’t add the type annotation here, Rust will display the following error, which means the compiler needs more information from us to know which type we want to use:

```sh
$ cargo build
   Compiling no_type_annotations v0.1.0 (file:///projects/no_type_annotations)
error[E0282]: type annotations needed
 --> src/main.rs:2:9
  |
2 |     let guess = "42".parse().expect("Not a number!");
  |         ^^^^^ consider giving `guess` a type

For more information about this error, try `rustc --explain E0282`.
error: could not compile `no_type_annotations` due to previous error
```

### Scalar Types

A scalar type represents a single value. Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

* Integer Types

An *integer* is a number without a fractional component. We used one integer type before, the u32 type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with **i**, instead of **u**) that takes up 32 bits of space. The table below shows the built-in integer types in Rust. We can use any of these variants to declare the type of an integer value.

| Length | Signed | Unsigned
| --- | --- | --- |
| 8-bit | u8 | i8 |
| 16-bit | u16 | i16 |
| 32-bit | u32 | i32 |
| 64-bit | u64 | i64 |
| 128-bit | u128 | i128 |
| arch | isize | usize |

Each variant can be either signed or unsigned and has an explicit size. Signed and unsigned refer to whether it’s possible for the number to be negative—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned).

Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an **i8** can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a **u8** can store numbers from 0 to 28 - 1, which equals 0 to 255.

Additionally, the **isize** and **usize** types depend on the architecture of the computer your program is running on, which is denoted in the table as “arch”: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in the following table. Note that number literals that can be multiple numeric types allow a type suffix, such as **57u8**, to designate the type. Number literals can also use **_** as a visual separator to make the number easier to read, such as **1_000**, which will have the same value as if you had specified **1000**.

| Number Literals | Example |
| --- | --- |
| Decimal | 98_222 |
| Hex | 0xff |
| Octal | 0o77 |
| Binary | 0b1111_0000 |
| Byte (u8 only) | b'A' |

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good places to start: integer types default to **i32**. The primary situation in which you’d use **isize** or **usize** is when indexing some sort of collection.

** Integer Overflow: Let’s say you have a variable of type **u8** that can hold values between 0 and 255. If you try to change the variable to a value outside of that range, such as 256, *integer overflow* will occur, which can result in one of two behaviors. When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to *panic* at runtime if this behavior occurs. Rust uses the term panicking when a program exits with an error.

When you’re compiling in release mode with the **--release** flag, Rust does *not* include checks for integer overflow that cause panics. Instead, if overflow occurs, Rust performs *two’s complement wrapping*. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a **u8**, the value 256 becomes 0, the value 257 becomes 1, and so on. The program won’t panic, but the variable will have a value that probably isn’t what you were expecting it to have. Relying on integer overflow’s wrapping behavior is considered an error.

* Floating-Point Types

Rust also has two primitive types for floating-point numbers, which are numbers with decimal points. Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size, respectively. The default type is f64 because on modern CPUs it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

Here’s an example that shows floating-point numbers in action:

```rs
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```

Floating-point numbers are represented according to the [IEEE-754 standard](https://en.wikipedia.org/wiki/IEEE_754). The **f32** type is a single-precision float, and **f64** has double precision.

* Numeric Operations

Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder. Integer division rounds down to the nearest integer. The following code shows how you’d use each numeric operation in a **let** statement:

```rs
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;
}
```

* Booleans

As in most other programming languages, a Boolean type in Rust has two possible values: **true** and **false**. Booleans are one byte in size. The Boolean type in Rust is specified using **bool**. For example:

```rs
fn main() {
    let t = true;

    let f: bool = false; // with explicit type annotation
}
```

* The Character Type

Rust’s **char** type is the language’s most primitive alphabetic type. Here’s some examples of declaring **char** values:

```rs
fn main() {
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

Note that we specify **char** literals with single quotes, as opposed to string literals, which use double quotes. Rust’s **char** type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid **char** values in Rust. Unicode Scalar Values range from **U+0000** to **U+D7FF** and **U+E000** to **U+10FFFF** inclusive. However, a “character” isn’t really a concept in Unicode, so your human intuition for what a “character” is may not match up with what a **char** is in Rust.

### Compound Types

*Compound types* can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

* Tuples

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same. We’ve added optional type annotations in this example:

```rs
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
```

The variable **tup** binds to the entire tuple, because a tuple is considered a single compound element. To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rs
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
```

This program first creates a tuple and binds it to the variable **tup**. It then uses a pattern with **let** to take **tup** and turn it into three separate variables, **x**, **y**, and **z**. This is called *destructuring*, because it breaks the single tuple into three parts. Finally, the program prints the value of **y**, which is **6.4**.

We can also access a tuple element directly by using a period (**.**) followed by the index of the value we want to access. For example:

```rs
fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}
```

The tuple without any values, **()**, is a special type that has only one value, also written **()**, The type is called the *unit type* and the value is called the *unit value*. Expressions implicitly return the unit value if they don’t return any other value.

* Arrays

Another way to have a collection of multiple values is with an *array*. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length:

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];
}
```

Arrays are useful when you want your data [allocated on the stack rather than the heap](https://stackoverflow.com/questions/79923/what-and-where-are-the-stack-and-heap) or when you want to ensure you always have a fixed number of elements. An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size.

Arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

```rs
let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
```

You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

```rs
let a: [i32; 5] = [1, 2, 3, 4, 5];
```

You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets, as shown here:

```rs
let a = [3; 5];
```

The array named **a** will contain **5** elements that will all be set to the value **3** initially. This is the same as writing **let a = [3, 3, 3, 3, 3]** but in a more concise way.

An array can be accessed the same way as in other programming languages:

```rs
fn main() {
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
```

Let's suppose we have the following chunk of code:

```rs
use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}
```

If ran with user input being either number 0 through 4, code will run successfully. However, if a number exceeding such range is given, let's say 10, the code panics:

```sh
thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 10', src/main.rs:19:19
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The error is raised at runtime because the compiler does not know the input the user will provide. This is one of Rust's advantage over other low-level languages that won't raise an error, and just let the user access invalid memory.

## Functions

Functions are prevalent in Rust code. You’ve already seen one of the most important functions in the language: the **main** function, which is the entry point of many programs. You’ve also seen the **fn** keyword, which allows you to declare new functions.

Rust code uses *snake case* as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words. Here’s a program that contains an example function definition:

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

We define a function in Rust by entering **fn** followed by a function name and a set of parentheses. The curly brackets tell the compiler where the function body begins and ends.

We can call any function we’ve defined by entering its name followed by a set of parentheses. Because **another_function** is defined in the program, it can be called from inside the **main** function. Note that we defined **another_function** *after* the **main** function in the source code; we could have defined it before as well. Rust doesn’t care where you define your functions, only that they’re defined somewhere.

Functions will be explored further in the *functions* project.

### Parameters

We can define functions to have *parameters*, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called *arguments*, but in casual conversation, people tend to use the words *parameter* and *argument* interchangeably for either the variables in a function’s definition or the concrete values passed in when you call a function.

We then modify the *main.rs* file in functions as an example:

```rs
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

By running this program we get:

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/functions`
The value of x is: 5
```

In function signatures, you *must* declare the type of each parameter. This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean.

When defining multiple parameters, separate the parameter declarations with commas, like this:

```rs
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

This code outputs:

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/functions`
The measurement is: 5h
```

### Statements and Expressions

Function bodies are made up of a series of statements optionally ending in an expression. So far, the functions we’ve covered haven’t included an ending expression, but you have seen an expression as part of a statement. Because Rust is an expression-based language, this is an important distinction to understand. Other languages don’t have the same distinctions, so let’s look at what statements and expressions are and how their differences affect the bodies of functions.

*Statements* are instructions that perform some action and do not return a value. *Expressions* evaluate to a resulting value. Let’s look at some examples.

We’ve actually already used statements and expressions. Creating a variable and assigning a value to it with the *let* keyword is a statement.

```rs
fn main() {
    let y = 6;
}
```

Function definitions are also statements; the entire preceding example is a statement in itself.

Statements do not return values. Therefore, you can’t assign a let statement to another variable, as the following code tries to do; you’ll get an error:

```rs
fn main() {
    let x = (let y = 6);
}
```

When you run this program, the error you’ll get looks like this:

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are experimental
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information
  = help: you can write `matches!(<expr>, <pattern>)` instead of `let <pattern> = <expr>`

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  | 

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 2 previous errors; 1 warning emitted
```

The **let y = 6** statement does not return a value, so there isn’t anything for **x** to bind to. This is different from what happens in other languages, such as C and Ruby, where the assignment returns the value of the assignment. In those languages, you can write **x = y = 6** and have both **x** and **y** have the value **6**; that is not the case in Rust.

Expressions evaluate to a value and make up most of the rest of the code that you’ll write in Rust. Consider a math operation, such as **5 + 6**, which is an expression that evaluates to the value **11**. Expressions can be part of statements: as seen before, the **6** in the statement **let y = 6**; is an expression that evaluates to the value **6**. Calling a function is an expression. Calling a macro is an expression. A new scope block created with curly brackets is an expression, for example:

```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);
}
```

This expression:

```rs
{
    let x = 3;
    x + 1
}
```

is a block that, in this case, evaluates to **4**. That value gets bound to **y** as part of the **let** statement. Note that the **x + 1** line doesn’t have a semicolon at the end, unlike most of the lines you’ve seen so far. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value. Keep this in mind as you explore function return values and expressions next.

### Functions with Return Values

Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (**->** as in Python). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the **return** keyword and specifying a value, but most functions return the last expression implicitly. Here’s an example of a function that returns a value:

```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
```

There are no function calls, macros, or even **let** statements in the **five** function, just the number **5** by itself. That’s a perfectly valid function in Rust. Note that the function’s return type is specified too, as **-> i32**. Try running this code; the output should look like this:

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/functions`
The value of x is: 5
```

The five function has no parameters and defines the type of the return value, but the body of the function is a lonely 5 with no semicolon because it’s an expression whose value we want to return.

Let’s look at another example:

```rs
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

Running this code will print **The value of x is: 6**. But if we place a semicolon at the end of the line containing **x + 1**, changing it from an expression to a statement, we’ll get an error:

```sh
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: consider removing this semicolon

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

The main error message, “mismatched types, ” reveals the core issue with this code. The definition of the function **plus_one** says that it will return an **i32**, but statements don’t evaluate to a value, which is expressed by **()**, the unit type. Therefore, nothing is returned, which contradicts the function definition and results in an error. 

## Comments

Comments are written starting with double forward slashes (**//**) and following with the code's comment. There are no multi line comments, so every line must start with double forward slashes. Example given:

```rs
fn five() -> i32 {
    // this function returns 5
    5
}

fn main() {
    let f = five();
    println!("Ive just instantiated a number {}", f); // prints text to console
}
```

## Control Flow

The ability to run some code depending on if a condition is true, or run some code repeatedly while a condition is true, are basic building blocks in most programming languages. The most common constructs that let you control the flow of execution of Rust code are **if** expressions and loops.

* **if** Expressions

**if** conditions are nested with a syntax similar to that of defining functions:

```rs
fn main() {
    let b = greater_than_one(3);
    println!("Condition was {}", b);
}

fn greater_than_one(x: i32) -> bool {
    if x > 1{
        True
    } else {
        False
    }
}
```

Running the code outputs:

```sh
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
Condition was true
```

All **if** expressions start with the keyword **if**, followed by a condition. In this case, the condition checks whether or not the variable **x** has a value greater than **1**. We place block of code to execute if the condition is true immediately after the condition inside curly brackets.

Optionally, we can also include an **else** expression, which we chose to do here, to give the program an alternative block of code to execute should the condition evaluate to false. If you don’t provide an **else** expression and the condition is false, the program will just skip the **if** block and move on to the next bit of code.

Conditions *must* be boolean, one can't establish truth values to other types of data (such as numbers or strings, like Python supports).

* Multiple **if else** statements

You can use multiple conditions by combining **if** and **else** in an **else if** expression. For example:

```rs
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

When this program executes, it checks each **if** expression in turn and executes the first body for which the condition holds true. Note that even though 6 is divisible by 2, we don’t see the output **number is divisible by 2**, nor do we see the **number is not divisible by 4, 3, or 2** text from the **else** block. That’s because Rust only executes the block for the first true condition, and once it finds one, it doesn’t even check the rest.

* Using **if** inside a **let** statement

Because **if** is an expression, we can use it on the right side of a **let** statement to assign the outcome to a variable:

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
}
```

However, **if** and **else** values should be type compatible, running:

```rs
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {}", number);
}
```

Raises an error:

```sh
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if` and `else` have incompatible types
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ expected integer, found `&str`
  |                                 |
  |                                 expected because of this

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

* Repetition with Loops

Rust has three kinds of loops: **loop**, **while**, and **for**. Let’s try each one.

The **loop** keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop. Example given:

```rs
fn main() {
    loop {
        println!("again!");
    }
}
```

This chunk of code will indefinitely print "again!" to the console until the user explicitly cancels execution with **^C**.

Fortunately, Rust also provides a way to break out of a loop using code. You can place the **break** keyword within the loop to tell the program when to stop executing the loop (as we did in the guessing game example). We also used **continue** in the guessing game, which in a loop tells the program to skip over any remaining code in this iteration of the loop and go to the next iteration.

One can also nest loops inside loops:

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);
}
```

The outer loop has the label **'counting_up**, and it will count up from 0 to 2. The inner loop without a label counts down from 10 to 9. The first **break** that doesn’t specify a label will exit the inner loop only. The **break 'counting_up; ** statement will exit the outer loop. This code prints:

```sh
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.58s
     Running `target/debug/loops`
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
End count = 2
```
