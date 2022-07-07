if/else
=======

Code branching in Rust is not surprising.

Conditions don't have parentheses! Did we ever really need them? Our logic now looks nice and clean.

All your usual relational and logical operators still work: `==`, `!=`, `<`, `>`, `<=`, `>=`, `!`, `||`, `&&`.


-----------------------------------------------

loop
====

Need an infinite loop?

Rust makes it easy.

`break` will escape a loop when you are ready.

`loop` has a secret we'll talk about soon.


-----------------------------------

Returning Values From loop
==========================

`loop` can break to return a value.

-----------------------------------

while
=====

`while` lets you easily add a condition to a loop.

If the condition evaluates to `false`, the loop will exit.


-----------------------------------------


for
===

Rust's `for` loop is a powerful upgrade. It iterates over values from any expression that evaluates into an iterator. What's an iterator? An iterator is an object that you can ask the question "What's the next item you have?" until there are no more items.

We'll explore this more in a future chapter. In the meantime, just know Rust makes it easy to create iterators that generate a sequence of integer numbers.

The `..` operator creates an iterator that generates numbers from a start number up to but not including an end number.

The `..=` operator creates an iterator that generates numbers from a start number up to and including an end number.

---------------------------------------


match
=====

Miss your switch statement? Rust has an incredibly useful keyword for matching all possible conditions of a value and executing a code path if the match is true. Let's see how this works for numbers. We will have more to say in future chapters on pattern matching more complex data. I promise you it will be worth the wait.

`match` is exhaustive so all cases must be handled.

Matching combined with destructuring is by far one of the most common patterns you will see in all of Rust.

-----------------------------------------


Returning Values From Block Expressions
=======================================

`if`, `match`, functions, and scope blocks all have a unique way of returning values in Rust.

If the last statement in an `if`, `match`, function, or scope block is an expression without a `;`, Rust will return it as a value from the block. This is a great way to create concise logic that returns a value that can be put into a new variable.

Notice that it also allows an `if` statement to operate like a concise ternary expression.

--------------------------------------------


