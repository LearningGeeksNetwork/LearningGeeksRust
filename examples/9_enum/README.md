Enumerations
============

Enumerations allow you to create a new type that can have a value of several tagged elements using the `enum` keyword.

`match` helps ensure exhaustive handling of all possible enum values making it a powerful tool in ensuring quality code.

--------------------------------



Enumerations With Data
======================

`enum` elements can also have one or more data types allowing them to behave like *union* from C.

When an `enum` is pattern matched using `match`, you can bind a variable name to each data value.

Memory details of `enum`:

-   An enum data value will have a memory size equal to its largest element. This allows for all potential values to fit in the same space of memory.
-   In addition to element data types (if any), each element also has a numeric value that represents which tag it is.

Other details:

-   Rust's `enum` is something also known as a *tagged union*.
-   The combining of types to make a new type is what people mean when they say Rust has *algebraic types*.



-----------------------------





