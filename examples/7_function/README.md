Functions
=========

A function has zero or more parameters.

In this example, the *add* function takes two arguments of type `i32` (signed integer of 32-bit length).

If you just want to return an expression, you can drop the `return` keyword and the semicolon at the end, as we did in the *subtract* function.

Function names are always in `snake_case`.

Hint: if you define a function, the data it accepts are called parameters. If you call that function and pass data to it, then it's called arguments.


Multiple Return Values
======================

Functions can return multiple values by returning a **tuple** of values.

Tuple elements can be referenced by their index number.

Rust supports various kinds of destructuring that we will see in many forms, allowing us to extract sub-pieces of data structures in ergonomic ways.


Returning Nothing
=================

If no return type is specified for a function, it returns an empty tuple, also known as a *unit*.

An empty tuple is represented by `()`.

Using `()` is uncommon, but will come up often enough that it's good to know whats happening.


------------------------------

Calling Methods
===============

Unlike functions, methods are functions associated with a specific data type.

**static methods** --- methods that belong to a type itself are called using the `::` operator.

**instance methods** --- methods that belong to an instance of a type are called using the `.` operator.

We will talk more on making your own methods in future


-------------------------------


