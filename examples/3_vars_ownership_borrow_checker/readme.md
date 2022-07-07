Ownership
=========

Instantiating a type and **binding** it to a variable name creates a memory resource that the Rust compiler will validate through its whole **lifetime**. The bound variable is called the resource's **owner**.

------------------

Scope-Based Resource Management
===============================

Rust uses the end of scope as the place to deconstruct and deallocate a resource.

The term for this deconstruction and deallocation is called a **drop**.

Memory detail:

-   Rust does not have garbage collection.
-   This is also called Resource Acquisition Is Initialization ( RAII ) in C++.



-----------------------------


Dropping is Hierarchical
========================

When a struct is dropped, the struct itself is dropped first, then its children are dropped individually, and so on.

Memory Details:

-   By automatically freeing memory Rust helps ensure that there are fewer memory leaks.
-   Memory resources can only be dropped once.


-------------------------------


Moving Ownership
================

When an owner is passed as an argument to a function, ownership is moved to the function parameter.

After a **move** the variable in the original function can no longer be used.

Memory details:

-   During a **move** the stack memory of the owners value is copied to the function call's parameter stack memory.

---------------------------------------


Returning Ownership
===================

Ownership can also be returned from a function.

----------------------------------


Borrowing Ownership with References
===================================

References allow us borrow access to a resource with the `&` operator.

References are also dropped like other resources.

-------------------------------------


Borrowing Mutable Ownership with References
===========================================

We can also borrow mutable access to a resource with the `&mut` operator.

A resource owner cannot be moved or modified while mutably borrowed.

Memory detail:

-   Rust prevents having two ways to mutate an owned value because it introduces the possibility of a data race.


---------------------------------

Dereferencing
=============

Using `&mut` references, you can set the owner's value using the `*` operator.

You can also get a copy of an owned value using the `*` operator (if the value can be copied - we will discuss copyable types in later chapters).

------------------------------------------

rustc --explain E0506

-----------------------------


Passing Around Borrowed Data
============================

Rust's rules for references might best be summarized by:

-   Rust only allows there to be one mutable reference **or** multiple non-mutable references **but not both**.
-   A reference must never **live longer** than its owner.

This doesn't tend to be a problem when passing around references to functions.

Memory details:

-   The first rule of references prevents data races. What's a data race? A data race when reading from data has the possibility of being out of sync due to the existence of a writer to the data at the same time. This happens often in multi-threaded programming.
-   The second rule of references prevents the misuse of references that refer to non-existent data (called dangling pointers in C).

-----------------------------------

References Of References
========================

References can even be used on pieces of references.


---------------------------------------

Explicit Lifetimes
==================

Even though Rust doesn't always show it in code, the compiler understands the lifetime of every variable and will attempt to validate that a reference never exists longer than its owner.

Functions can be explicit by parameterizing the function signature with symbols that help identify which parameters and return values share the same lifetime.

Lifetime specifiers always start with a `'` (e.g. `'a`, `'b`, `'c`)

--------------------------------------


Multiple Lifetimes
==================

Lifetime specifiers allow us to be explicit with certain scenarios the compiler cannot resolve itself by distinguishing all of a function signature component's lifetimes.

----------------------------------------

Static Lifetimes
================

A **static** variable is a memory resource created at compile-time that exists through a program start to finish. They must have their types explicitly specified.

A **static lifetime** is a memory resource that lasts indefinitely to the end of a program. Note that by this definition some static lifetime resources can be created at runtime.

Resources with static lifetimes have a special lifetime specifier `'static`.

`'static` resources will never **drop**.

If static lifetime resources contain references they must all be `'static` (anything less would not live long enough).

Memory detail:

-   Modifying static variables is inherently dangerous because they are globally accessable to be read from by anyone introducing the possibility of a data race. We'll talk about the challenges of global data later.
-   Rust allows the use of `unsafe { ... }` blocks to perform some operations that the compiler cannot make memory guarantees about.

https://doc.rust-lang.org/nomicon/

------------------------------------------

Lifetimes In Data Types
Similarly to functions, data types can be parameterized with lifetime specifiers of its members.

Rust validates that the containing data structure of the references never lasts longer than the owners its references point to.

We can't have structs running around with references pointing to nothingness!

----------------------------------------------


Rust as a language aims to solve many of these common challenges in systems programming:

-   Unintentional modification of resources
-   Forgetting to deconstruct resources
-   Resources accidentally being deconstructed twice
-   Using resources after they have been deconstructed
-   Data races caused by writing to resources while others are reading from resources
-   Seeing clearly areas of the code where the compiler can't make guarantees


-------------------------------





