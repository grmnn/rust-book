# Data Types

Rust ist a *statically typed language*, which means that it must know the types of all variables at compile time. The Rust compiler can usually infer what type we want to use based on the value and how we use it. 

In case when many types are possible, such as in our [Guessing Game](../guessing-game.md), where we converted a `String` to a numeric value to compare the user input with the random generated number, we must add a type annotation.

```rust
let guess: u32 = "42".parse().expect("Not a number!");
```

If we wouldn't annotate the `guess` variable with the `unsigned 32-bit integer type`, the compiler wouldn't know what kind of type we want to have.


## Scalar Types
A __scalar type__ represents a single value. Rust has four primary scalar types: `integers`, `floating-point numbers`, `Booleans`, and `characters`

### Integer

Integers are numbers without a fraction. `Integer` types can have a length of **8, 16, 32, 64, 128 bits** and can be signed or unsigned, which is just a way to refer to the possibility if the number can be negative (`signed`) or not (`unsigned`).

Each signed variant can store numbers from -(2n - 1) to 2n - 1 - 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from -(27) to 27 - 1, which equals -128 to 127. Unsigned variants can store numbers from 0 to 2n - 1, so a u8 can store numbers from 0 to 28 - 1, which equals 0 to 255.

In Rust we also can easily get the maximum or minimum number like so: 

```rust
let min_unsigned_8bit_num = u8::MIN;
let max_unsigned_8bit_num = u8::MAX;

let min_signed_8bit_num = i8::MIN;
let max_signed_8bit_num = i8::MAX;

println!("Minimum unsigned 8 bit number: {min_unsigned_8bit_num}");
println!("Maximum unsigned 8 bit number: {max_unsigned_8bit_num}");

println!("Minimum signed 8 bit number: {min_signed_8bit_num}");
println!("Maximum signed 8 bit number: {max_signed_8bit_num}");
```

Which would produce following output:

```shell
cargo run
   Compiling variables v0.1.0 (~/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/variables`
Minimum unsigned 8 bit number: 0
Maximum unsigned 8 bit number: 255
Minimum signed 8 bit number: -128
Maximum signed 8 bit number: 127
```

Additionally we also have the `isize` and `usize` types, which are dependent on the architecture of the computer your program is running on.

So for example if you have a 64-bit machine the `u64::MAX` and `usize::MAX` is the same. Both would be the number 18446744073709551615

```rust
let max_unsigned_64bit_num = u64::MAX;
let max_unsigned_arch_num = usize::MAX;

println!("Minimum unsigned 64 number: {max_unsigned_64bit_num}");
println!("Maximum unsigned architectural number: {max_unsigned_arch_num}");
```

```shell
$ cargo run
   Compiling variables v0.1.0 (~/projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/variables`
Minimum unsigned 64 number: 18446744073709551615
Maximum unsigned architectural number: 18446744073709551615

```

