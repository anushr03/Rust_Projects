This Repo keeps track of all the projects I did to learn the Rust language using the Rust official programming language pdf([book](https://doc.rust-lang.org/book/title-page.html))

# 1. Hello_world and Hello_cargo

Intial hello wrold code to introduce myself to the Rust Programming language

# 2. guessing_game

A user input guessing game, run via the terminal, where the user keeps on guessing the number until it matches the random number seelcted by the code. I added another functionality to this where the user can input q and exit the game at any given time.

# 3. variables

An introduction to variables in Rust. Learned about <ins>shadowing</ins>, <ins>floating points</ins> and difference between <ins>tuples and arrays</ins> in Rust

# 4. Functions

Learned about functions. 

### a. Statements and Expressions
Firstly learned about the difference between statements and expression. Statements do not return a value and expressions return a value. This is why expressions do not have a semicolon. If we add the semicolon, it becomes a statement and therefore will not return a value.

### b. Functions with return values
Functions can return values to the code that calls them. We don’t name return values, but we must declare their type after an arrow (->)

# 5. Control Flow
 Learned about if expressions and loops

### a. if statements
An if expression allows you to branch your code depending on conditions. You provide a condition and then state, “If this condition is met, run this block of code. If the condition is not met, do not run this block of code.”

### b. Loops
It’s often useful to execute a block of code more than once. For this task, Rust provides several loops, which will run through the code inside the loop body to the end and then start immediately back at the beginning. To experiment with loops, let’s make a new project called loops.

Rust has three kinds of loops: **loop**, **while**, and **for**. 

# 6. Understanding Ownership
Ownership is a set of rules that govern how a Rust program manages memory. In Rust, memory is managed through a system of ownership with a set of rules that the compiler checks.

### a. Stacks and Heaps
The memory can find data faster when its on a stack as compared to the heap. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

### b. Ownership Rules
1. Each value in the rust has an _owner_

2. There can only be one owner at a time

3. When the owner goes out of scope, the value will be dropped

### c. Variable scope
```rust
fn main() {
    let s = "hello";
    println!("{s}\n");
}
```
Here the scope of s is only valid between the two ```{}``` brackets. 

In other words, there are two important points in time here:

- When s comes into scope, it is valid.

- It remains valid until it goes out of scope.

At this point, the relationship between scopes and when variables are valid is similar to that in other programming languages.

### d. The ```string``` type
Unlike other data types that we have seen, ```string``` is a complex data type, which is stored on the heap. 

I can also create a ```string``` from the string literal by using the ```from``` keyword. For eg.
```rust
let s = String::from("hello");
```

### e. Memory and Allocation
In the case of a string literal, we know the contents at compile time, so the text is hardcoded directly into the final executable. This is why string literals are fast and efficient. But these properties only come from the string literal’s immutability. Unfortunately, we can’t put a blob of memory into the binary for each piece of text whose size is unknown at compile time and whose size might change while running the program.

With the ```String``` type, in order to support a mutable, growable piece of text, we need to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. This means:

- The memory must be requested from the memory allocator at runtime.

- We need a way of returning this memory to the allocator when we’re done with our ```String```.

That first part is done by us: when we call ```String::from```, its implementation requests the memory it needs. This is pretty much universal in programming languages.

In Rust, the memory is instantly returned, once the variable goes out of scope. For eg, the code below but with ```String```, instead of the string literal

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s

}   // this scope is now over, and s is no longer valid
```