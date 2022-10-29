# Getting Started

## Hello World

To get started we create a simple `main.rs` file which contians

```rust
fn main() {
  println!("Hello World!")
}
```

The `main` function is special, it is always the first code that runs in every executable Rust program.
The function holds a *Rust Macro* - fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming, which we will explore later in [Chapter 19](https://doc.rust-lang.org/book/ch19-06-macros.html). For now we just need to know that using a `!` means that we're calling a macro instead of a normal function and Macros don't always folow the same rules as functions.

As the name of the macro indicates we print the given argument `"Hello World"` in the terminal. 

To see our first programm in action, we need to *compile* and *run* our program. 

Before we can run a Rust program, we must *compile* it. To do this we will use the Rust compiler `rustc`. Entering `rustc main.rs` will compile our program and create a binary executeable `main` in our project directory. 

Now that our program is compiled, we can *run* it. To see "Hello World" in the terminal, we can check first if its available.

```shell
$ ls
main  main.rs
```

As we see the `main` executeable is available, now we can run it by entering `./main` in our terminal and we should see our message be printed.

```shell
$ ./main
Hello World!
```

Compiling simple programs with `rustc` is fine, but as our projects grows it would be tedious to always to do the above steps to see if our program runs, therefore Rust has a tool called *Cargo* to manage, install, test, compile and run our Rust programs.

## Cargo
To start a new Rust project all we need to do is run `cargo new hello_cargo`. 

```shell
./src
  main.rs
.gitignore
Cargo.toml
```

As we can see, a new project being created in our directory where we ran the command and see that there is a `src` directory which includes our `main.rs` file, as we created ourselves in the __hello_word__ project, a `.gitignore` file, since `cargo new` initializes a new Git repository (it won't do this, if we run `cargo new` within an existing Git repository) and a `Cargo.toml` which holds all the information about our Rust projects like the ``name``, ``version`` and a empty list of our `dependencies`.

### Building and Running a Cargo Project

To build a program with Cargo, we can simply type `cargo build` in our terminal and see 
```shell
cargo build
   Compiling hello_cargo v0.1.0 (~/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.40s
```

This command creates an executable file in __target/debug/hello_cargo__.

Running `cargo build` for the first time also causes Cargo to create a new file at the top level: `Cargo.lock`. This file keeps track of the exact versions of dependencies in your project. Cargo manages its contents for you, so you won't ever need to change that file manually.

Since we would need to make multiple steps to run the compiled executable, there is also a `cargo run` command which will do the compilation and running in one command. It's more convenient than having to run `cargo build`, wait for it and then use the whole path to the binary to run the program.

When runnining `cargo run`, we can see it didn't compile the program again, because it is smart enough to know that nothing has changed since the last compilation, therefore skips this step. 

```shell
cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_cargo`
Hello, world!
```

If we now change just a comma in our argument to the `println!` macro, cargo will recognize that something has changed, compiles the program and runs it again

```shell
cargo run
   Compiling hello_cargo v0.1.0 (~/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
     Running `target/debug/hello_cargo`
Hello world!
```

Cargo also provides a command called `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:

```shell
cargo check 
    Checking hello_cargo v0.1.0 (~/projects/hello_cargo)
    Finished dev [unoptimized + debuginfo] target(s) in 0.08s
```

This makes sense, if you just want to quickly check if your changes are working, since it is faster because it skips the step of producing a executable, which takes longer as the project grows.

*Recap:*
- We can create a project using cargo new.
- We can build a project using cargo build.
- We can build and run a project in one step using cargo run.
- We can build a project without producing a binary to check for errors using cargo check.
- Instead of saving the result of the build in the same directory as our code, Cargo stores it in the target/debug directory.

### Building for Release