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
