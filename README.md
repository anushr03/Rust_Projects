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
```
fn main() {
    let s = "hello";
    println!("{s}\n");
}
```
Here the scope of s is only valid between the two ```{}``` brackets. 

    