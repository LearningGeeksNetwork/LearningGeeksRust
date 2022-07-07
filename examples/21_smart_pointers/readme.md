Smart Pointers
==============

In this chapter we will demystify smart pointers. Let's explore into these data structures that let us interact with the lowest level of memory.

Ferris says: "Don't feel overwhelmed by this chapter if you don't feel you can write your own low level memory management code in one short read. This chapter is mostly to introduce you to some useful tools and give a glimpse at how they work!"

-------------------------


References Revisited
====================

A reference is fundamentally just a number that is the start position of some bytes in memory. Its only purpose is to represent the concept of where data of a specific type exists. What makes a reference different from just a number is that Rust will validate the lifetime of references doesn't last longer than what it refers to (otherwise we'd get an error when we used it!).

--------------------------


Raw Pointers
============

References can be converted into a more primitive type called a *raw pointer*. Much like a number, it can be copied and moved around with little restriction. Rust makes no assurances of the validity of the memory location it points to.

Two kinds of raw pointers exist:

-   `*const T` - A raw pointer to data of type T that should never change.
-   `*mut T` - A raw pointer to data of type T that can change.

Raw pointers can be converted to and from numbers (e.g. `usize`).

Raw pointers can access data with *unsafe* code (more on this later).

Memory Details:

-   A reference in Rust is very similar to a pointer in C in terms of usage, but with much more compile time restrictions on how it can be stored and moved around to other functions.
-   A raw pointer in Rust is similar to a pointer in C that it represents a number that can be copied or passed around, and even turned into numerical types where it can be modifed as a number to do pointer math.

-----------------------------------

Dereferencing
=============

The process of accessing/manipulating data that is being referred to by a *reference* (i.e. `&i32`) is called *dereferencing*.

References are used to access/manipulate data in two ways:

-   Access to the referred data during assignment of variables.
-   Access to fields or methods of the referred data.

Rust has some powerful operators that allow us to do this.

-------------------------


The * Operator
==============

The `*` operator is an explicit way to dereference a reference.

```
let a: i32 = 42;
let ref_ref_ref_a: &&&i32 = &&&a;
let ref_a: &i32 = **ref_ref_ref_a;
let b: i32 = *ref_a;

```

Memory detail:

-   Because i32 is a primitive type that implements the `Copy` trait, the bytes of variable `a` on stack are copied into the bytes of variable `b`.

------------------------------------------------

The . Operator
==============

The `.` operator is used in accessing fields and methods of a reference. It works a bit more subtly.

```
let f = Foo { value: 42 };
let ref_ref_ref_f = &&&f;
println!("{}", ref_ref_ref_f.value);

```

Whoa, why didn't we need to add `***` before `ref_ref_ref_f`? This is because the `.` operator automatically dereferences a sequence of references. That last line is turned into the following by the compiler automatically.

```
println!("{}", (***ref_ref_ref_f).value);

```

-------------------------------------------

Smart Pointers
==============

In addition to the ability to create references to existing typed data using the `&` operator, Rust gives us the ability to create *reference-like* structs called **smart pointers**.

We can think of references at a high level as a type that give us access to another type. Smart pointers are different in their behavior from normal references in that they operate based on internal logic that a programmer writes. You --- the programmer --- are the *smart* part.

Typically smart pointers implement `Deref`, `DerefMut`, and `Drop` traits to specify the logic of what should happen when the structure is dereferenced with `*` and `.` operators.

-------------------------------------------

Smart Unsafe Code
=================

Smart pointers tend to use *unsafe* code fairly often. As mentioned earlier, they are common tools for interacting with the lowest levels of memory in Rust.

What is unsafe code? Unsafe code behaves exactly like normal Rust with the exception of a few abilities that the Rust compiler is unable to make guarantees about.

A primary ability of unsafe code is *dereferencing a raw pointer*. That means taking a *raw pointer* to a position in memory and declaring "a data structure exists here!" and turning it into a representation of data you can use (i.e. `*const u8` into `u8`). Rust has no way to keep track of the meaning of every byte that gets written to memory. Because Rust can't make guarantees about what exists at an arbitrary number used as a *raw pointer*, it puts the dereference in an `unsafe { ... }` block.

Smart pointers *dereference raw pointers* extensively, but they are well proven in what they do.

-------------------------------------------

Familiar Friends
================

Consider some smart pointers we've already seen like `Vec<T>` and `String`.

`Vec<T>` is a smart pointer that just owns some memory region of bytes. The Rust compiler has no idea what exists in these bytes. The smart pointer interprets what it means to grab items from the region of memory it manages, keeps track of where data structures within those bytes begin and end, and then finally dereferences a raw pointer into data structures into a nice clean ergonomic interface for us to use (e.g. `my_vec[3]`).

Similarly, `String` keeps track of a memory region of bytes, and programmatically restricts content written to it to always be valid `utf-8` and helps dereference that memory region into a type `&str`.

Both these datastructures use unsafe dereferencing of raw pointers to do their job.

Memory details:

-   Rust has an equivalent of C's `malloc` using [alloc](https://doc.rust-lang.org/std/alloc/fn.alloc.html) and [Layout](https://doc.rust-lang.org/std/alloc/struct.Layout.html) for getting ahold of your own memory regions to manage.

-------------------------------------------


Heap Allocated Memory
=====================

`Box` is a smart pointer that lets us move data from the stack to the heap.

Dereferencing it lets us use the heap allocated data ergonomically as if it were the original type.

----------------------------------------

Failable Main Revisited
=======================

Rust code may have a plethora of representations of errors, but the standard library has a universal trait `std::error::Error` for describing errors.

Using a smart pointer `Box` we can use the type `Box<dyn std::error::Error>` as a common type for returning errors because it allows us to propagate up an error on the heap and interact with it at a high level without having to know a specific type.

Early in Tour of Rust we learned that `main` can return an error. We can now return a type capable of describing almost any kind of error that might occur in our program so long as the error's data structure implements Rust's common `Error` trait.

```
fn main() -> Result<(), Box<dyn std::error:Error>>

```

-----------------------------------------------

Referencing Counting
====================

`Rc` is a smart pointer that moves data from the stack onto the heap. It allows us to clone other `Rc` smart pointers that all have the ability to immutably borrow the data that was put on the heap.

Only when the last smart pointer is dropped does the data on the heap become deallocated.

--------------------------------------------


Sharing Access
==============

`RefCell` is a container data structure commonly held by smart pointers that takes in data and lets us borrow mutable and immutable references to what's inside. It prevents borrowing from being abused by enforcing Rust's memory safety rules at runtime when you ask to borrow the data within:

**Only one mutable reference OR multiple immutable references, but not both!**

If you violate these rules `RefCell` will panic.


------------------------------------------------------------


Sharing Across Threads
======================

`Mutex` is a container data structure commonly held by smart pointers that takes in data and lets us borrow mutable and immutable references to the data within. This prevents borrowing from being abused by having the operating system restrict only one CPU thread at time to have access to the data, blocking other threads until that original thread is done with its locked borrow.

Multithreading is beyond the scope of Tour of Rust, but `Mutex` is a fundamental part of orchestrating multiple CPU threads accessing the same data.

There is a special smart pointer `Arc` which is identical to `Rc` except uses thread-safe incrementing of reference counts. It's often used to have many references to the same `Mutex`.

-----------------------------------------------------------

Combining Smart Pointers
========================

Smart pointers might seem limited, but they can make some very powerful combinations.

`Rc<Vec<Foo>>` - Allow the cloning of multiple smart pointers that can borrow the same vector of immutable data structures on the heap.

`Rc<RefCell<Foo>>` - Allow multiple smart pointers the ability to borrow mutably/immutably the same struct `Foo`

`Arc<Mutex<Foo>>` - Allow multiple smart pointers the ability to lock temporary mutable/immutable borrows in a CPU thread exclusive manner.

Memory detail:

-   You'll notice a theme with many of these combinations. The use of a immutable data type (possibly owned by multiple smart pointers) to modify internal data. This is referred to as the "interior mutability" pattern in Rust. It is a pattern that lets us bend the rules of memory usage at runtime with the same level of safety as Rust's compile-time checks.

--------------------------------------



