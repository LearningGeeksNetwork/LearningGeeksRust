Memory
======

Rust programs have 3 memory regions where data is stored:

-   **data memory** - For data that is fixed in size and **static** (i.e. always available through life of program). Consider the text in your program (e.g. "Hello World!"): This text's bytes are only ever read from one place and therefore can be stored in this region. Compilers make lots of optimizations with this kind of data, and they are generally considered very fast to use since locations are known and fixed.
-   **stack memory** - For data that is declared as variables within a function. The location of this memory never changes for the duration of a function call; because of this compilers can optimize code so stack data is very fast to access.
-   **heap memory** - For data that is created while the application is running. Data in this region may be added, moved, removed, resized, etc. Because of its dynamic nature it's generally considered slower to use, but it allows for much more creative usages of memory. When data is added to this region we call it an **allocation**. When data is removed from this section we call it a **deallocation**.


-------------------------------

Creating Data In Memory
=======================

When we **instantiate** a **struct** in our code our program creates the associated field data side by side in memory.

We instantiate by specifying all field values within

`StructName { ... }`.

Struct fields are accessed using a dot operator `.`.

Memory details of our example:

-   The text inside the quotes is read only data (e.g. "Ferris"), therefore it is placed in the *data memory region*.
-   The function call `String::from` creates a struct `String` that is placed side by side with the fields of SeaCreature in the *stack*. A String represents text that can be changed and does this by:

1.  Creating memory on the *heap* for the text where it can be modified
2.  Storing a reference to that memory location on the *heap* and storing it in `String` struct (More on this in future lessons)

-   Finally our two friends *Ferris* and *Sarah* have data structures that will always have fixed locations in our program, so they are placed on the *stack*.


-----------------------------------------------

Tuple-like Structs
==================

For conciseness, you can create structs that are used like a tuple.

--------------------------------

Unit-like Structs
=================

Structs do not have to have any fields at all.

As mentioned in Chapter 1 a *unit* is another word for an empty tuple `()`. This is why this kind of struct is called *Unit-like*.

This type of struct is rarely used.

--------------------------------------




