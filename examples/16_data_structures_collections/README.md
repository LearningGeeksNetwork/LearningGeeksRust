Basic Data Structure Types
==========================

It's time we explore beyond basic types! In this chapter we will look at the most primitive data structures in Rust, paying close attention to their representations in memory. I think you will enjoy how little Rust hides from you how things work.

------------------------

Structures
==========

A `struct` is a collection of fields.

A *field* is simply a data value associated with a data structure. Its value can be of a primitive type or a data structure.

Its definition is like a blueprint for a compiler on how to layout the fields in memory nearby each other.

---------------------------------


Vectors
=======

Some of the most useful generic types are collection types. A vector is a variably sized list of items represented by the struct `Vec`.

The macro `vec!` lets us easily create a vector rather than manually constructing one.

`Vec` has the method `iter()` which creates an iterator from a vector, allowing us to easily put a vector into a `for` loop.

Memory Details:

-   `Vec` is a struct, but internally it contains a reference to a fixed list of its items on the heap.
-   A vector starts with a default capacity; when more items are added than it has capacity for, it reallocates its data on the heap to have a new fixed list with large capacity.

------------------------


