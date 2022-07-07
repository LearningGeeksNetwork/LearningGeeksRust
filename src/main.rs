extern crate rust_learning_geeks;

use rust_learning_geeks::TaskList;
use std::io::prelude::*;
use std::process;

mod foomodule;
mod module0;

// Currently handles all errors and just calls process::exit(1)
// whenever an error occurs.
fn main() {

    let f = foomodule::Foo::new("hello");
    let res = module0::answer();
    println!("{:?} {}", f,res);
  
    println!("Taskers: The tacky task tracker for tiresome tasks");

    let mut task_list = TaskList::new();
    let mut stderr = std::io::stderr();

    if let Err(err) = task_list.load_from_file() {
        writeln!(&mut stderr, "File handling error: {}", err)
            .expect("Couldn't write to stderr");
        process::exit(1);
    } 

    if let Err(err) = rust_learning_geeks::run(&mut task_list) {
        writeln!(&mut stderr, "Application error: {}", err)
            .expect("Couldn't write to stderr");
        process::exit(1);
    }



}
