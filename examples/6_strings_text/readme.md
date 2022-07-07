String Literals
===============

String literals are always Unicode.

String literals type are `&'static str`:

-   `&` meaning that it's referring to a place in memory, and it lacks a `&mut` meaning that the compiler will not allow modification
-   `'static` meaning the string data will be available till the end of our program (it never drops)
-   `str` means that it points to a sequence of bytes that are always valid **utf-8**

Memory details:

-   The Rust compiler will likely put your string in the data segment of your program memory

---------------------------------------


What is utf-8
=============

As more languages were used on computers, the world needed to represent more text characters than ASCII allowed (1 byte only allowed 256 characters).

**utf-8** was introduced with a variable byte length of 1-4 bytes greatly increasing the range of possible characters.

An advantage of variable sized characters is text did not have unnecessary bytes for very common ASCII (only requiring 1 byte still in **utf-8**).

A downside of variable sized characters is that character lookup can no longer be done quickly (**O(1)** constant time) with a simple indexing (e.g. `my_text[3]` to get the 4th character). It's possible that the preceding characters could have variable widths, altering where the 4th character actually begins in the sequence of bytes.

Instead we must iterate through a **utf-8** byte sequence to understand where the Unicode characters actually begin (**O(n)** linear time).

Ferris: "I'm mostly just happy to have **utf-8** for representing emojis of my underwater friends."
🐠🐙🐟🐬🐋

------------------------------------------------

Escaping Characters
===================

It's challenging to visually represent certain characters, so **escape codes** allow us to put a symbol in their place.

Rust supports the common escape codes from C-based languages:

-   `\n` - newline
-   `\r` - carriage return
-   `\t` - tab
-   `\\` - backslash
-   `\0` - null
-   `\'` - single-quote

The complete list exists [here](https://doc.rust-lang.org/reference/tokens.html).

-----------------------------------------------

Multi-line String Literals
==========================

Rust strings are multiline by default.

Use a `\` at the end of a line if you don't want a line break.

--------------------------------------

Raw String Literals
===================

Raw strings allow us to write a sequence of characters verbatim by starting with `r#"` and ending with `"#`. It lets us insert characters that might otherwise confuse a normal string as literals (like double quotes and backslashes).

-----------------------------------------

String Literals From Files
==========================

If you have some very large text, consider using the macro `include_str!` to include text from local files in your program:

```
let hello_html = include_str!("hello.html");
```

-------------------------------------------

String Slice
============

A string slice is a reference to a sequence of bytes in memory that must always be valid utf-8.

A string slice (a sub-slice) of a `str` slice, must also be valid utf-8.

Common methods of `&str`:

-   `len` gets the length of the string literal in bytes (not number of characters).
-   `starts_with`/`ends_with` for basic testing.
-   `is_empty` returns true if zero length.
-   `find` returns an `Option<usize>` of the first position of some text.

------------------------------------------------

Chars

=====

With so much difficulty in working with Unicode, Rust offers a way to retrieve a sequence of utf-8 bytes as a vector of characters of type `char`.

A `char` is always 4 bytes long (allowing for efficient lookup of individual characters).

--------------------------------------

String
======

A **String** is a struct that owns a sequence of utf-8 bytes in heap memory.

Because its memory is on the heap, it can be extended, modified, etc. in ways string literals cannot.

Common methods:

-   `push_str` to add more utf-8 bytes to the end of a string.
-   `replace` to replace sequences of utf-8 bytes with others.
-   `to_lowercase`/`to_uppercase` for case changes.
-   `trim` for trimming space

When a String is dropped, its heap memory is also dropped.

`String` has a `+` operator that extends the string with a `&str` and returns itself, but it might not be as ergonomic as you hope for.

-----------------------------------

Text As Function Parameters
===========================

String literals and strings are generally passed around as a string slice to functions. This offers a lot of flexibility for most scenarios where you don't actually have to pass ownership.

--------------------------------

Building Strings
================

`concat` and `join` are two simple but powerful ways for building strings.

--------------------------------

Formatting Strings
==================

The `format!` macro allows us to create a string by defining a parameterized string with placeholders for where and how values should be placed (e.g. `{}`).

`format!` uses the same parameterized strings as `println!`

The capabilities of this function are too large of scope for *Tour of Rust*; check out the documentation [here](https://doc.rust-lang.org/std/fmt/).

------------------------------------------


Converting Strings
==================

Many types can be converted to a string using `to_string`.

The generic function `parse` can be used to convert strings or string literals into a typed value. This function returns a `Result` because it could fail.


-------------------------------










