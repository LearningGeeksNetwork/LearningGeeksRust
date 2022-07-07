Primitive (Basic) Types
===========

Rust has a variety of familiar types:

-   booleans - `bool` for representing true/false
-   unsigned integers - `u8` `u16` `u32` `u64` `u128` for representing nonnegative whole numbers
-   signed integers - `i8` `i16` `i32` `i64` `i128` for representing whole numbers
-   pointer sized integers - `usize` `isize` for representing indexes and sizes of things in memory
-   floating point - `f32` `f64`
-   tuple - `(value, value, ...)` for passing fixed sequences of values on the stack
-   arrays - `[value, value, ...]` a collection of similar elements with fixed length known at compile time
-   slices - a collection of similar elements with length known at runtime
-   `str`(string slice) - text with a length known at runtime

Text might be more complex than you are used to in other languages; since Rust is a system programming language, it cares about memory issues you might not be used to. We will be going into this in detail later.

Numeric types can be explicitly specified by appending the type to the end of the number (e.g. `13u32`, `2u8`).


Basic Type Conversion
=====================

Rust requires explicitness when it comes to numeric types. One cannot use a `u8` for a `u32` casually without error.

Luckily Rust makes numeric type conversions very easy with the **as** keyword.


Arrays
======

An *array* is a **fixed length collection** of data elements all of the same type.

The data type for an *array* is `[T;N]` where T is the elements' type, and N is the fixed length known at compile-time.

Individual elements can be retrieved with the `[x]` operator where *x* is a *usize* index (starting at 0) of the element you want.

Collections with a dynamic length, often called dynamic or variable arrays, are introduced in a later chapter about **Vectors**.


Rust Cheat Sheet - Data Types
https://cheats.rs/#basic-types


https://euangoddard.github.io/clipboard2markdown/





