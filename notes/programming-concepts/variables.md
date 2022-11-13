# Variables

## Mutability

In Rust, variables are immutable by default. This means, once a value is bound to a variable name, we can't change that value.

To change that we can add the `mut` in front of the variable name to tell the Rust compiler it is okay that the value will change and its also indicates to future readers fo the code that other parts of the code will be changing this variables value. 

## Constants

Like immutable variables, __constants__ are a values that are bound to a name and are not allowed to change, as the name describes, they are constant. 

Other than that these variables are *always* immutable, we also need to annotate the type.

Constants can be defined in any scope, most likely we will find them in the global scope, which makes them useful for values that many parts of code need to know about. 

**The last difference is that constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.**

`const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

Rust’s naming convention for constants is to use all uppercase with underscores between words.

The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800. See the [Rust Reference’s](https://doc.rust-lang.org/reference/const_eval.html) section on constant evaluation for more information on what operations can be used when declaring constants.

## Shadowing

Shadowing can be best described with an example:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

Whats happening here:

1. We define the variable `x` and bind the value of `5` to it.
2. Then we create a new variable `x` and take the original value and add `1` to it. 
3. We create a new inner scope via the curly brackets `{}` and shadow the variable `x` a third time.
4. No we print out that the variable `x` holds the value `12`
5. After that we leave the inner scope and therefore the inner shadowing ends and `x` returns to being `6`.

When we run the code above we get following output: 

```shell
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/variables`
The value of x in the inner scope is: 12
The value of x is: 6
```

Shadowing is different from marking a variable as mut, because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. 

**By using let, we can perform a few transformations on a value but have the variable be immutable after those transformations have been completed** 

Like in the following code:

```rust
let mut vec = Vec::new();
vec.push(1);
vec.push(2);
let vec = vec;
```

Here we create a new `Vec` (Vector), mutate the vector and shadow it to make it immutable in the future usage of our program.

The other difference between `mut` and `shadowing` is that because we’re effectively creating a new variable when we use the `let` keyword again, we can change the type of the value but reuse the same name. 

```rust
let spaces = "   ";
let spaces = spaces.len();
```

The first `spaces` variable is a `string` type and the second `spaces` variable is a `number` type. Shadowing thus spares us from having to come up with different names, such as `spaces_str` and `spaces_num`; instead, we can reuse the simpler spaces name.

However, if we try to use mut for this, as shown here:

```rust
let mut spaces = "   ";
spaces = spaces.len();
```

We’ll get a compile-time error:which states that we’re not allowed to mutate a variable’s type

```shell
$ cargo run
   Compiling variables v0.1.0 (~/projects/variables)
error[E0308]: mismatched types
 --> src/main.rs:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

```