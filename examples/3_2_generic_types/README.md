
Generic Types
==============

Generic types are incredibly important in Rust. They are used in the representation of nullable values (i.e. variables which might not have a value yet), error handling, collections, and more! In this section we will be learning about the foundational generic types you will likely be using all the time.

--------------------------


What Are Generic Types?
Generic types allow us to partially define a struct or enum, enabling a compiler to create a fully defined version at compile-time based off our code usage.

Rust generally can infer the final type by looking at our instantiation, but if it needs help you can always be explicit using the ::<T> operator, also known by the name turbofish.


--------------------------------------

Representing Nothing
====================

In other languages, the keyword `null` is used to represent an absence of a value. It creates difficulty in programming languages because it creates the possibility that our program might fail when interacting with a variable/field.

Rust does not have `null`, but it is not ignorant of the importance of representing nothing! Consider a naive representation using a tool we already know.

This pattern of providing a `None` alternative representation for one or many alternate values is so common in Rust because of its lack of a `null` value. Generic types help solve this challenge.

-----------------------------------

Option
======

Rust has a built in generic enum called `Option` that allows us to represent nullable values without using `null`.

```
enum Option<T> {
    None,
    Some(T),
}

```

This enum is so common, instances of the enum can be created anywhere with the enum variants `Some` and `None`.


----------------------------



Result
======

Rust has a built in generic enum called `Result` that allows us to return a value that has the possibility of failing. It is the idiomatic way in which the language does error handling.

```
enum Result<T, E> {
    Ok(T),
    Err(E),
}

```

Note that our generics type has multiple *parameterized types* separated by a comma.

This enum is so common, instances of the enum can be created anywhere with the enum variants `Ok` and `Err`.

-----------------------------------






