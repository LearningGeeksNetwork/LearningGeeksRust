/*
These are different lints. dead_code refers to unused code at the item level, e.g. imports, functions and types. unused_variables refers to variables that are never accessed.
You can also cover both cases with #[allow(unused)].
*/

use std::env;


const PI: f32 = 3.14159;

#[allow(unused)]
fn main() {

    // this method needs to be inside main() method
    env::set_var("RUST_BACKTRACE", "full");

    let mon_x = 12; // by default this is i32
    let mon_a: u8 = 12;
    let a = 14u16;
    let uu: u128 = 345666;
    let ii: i128 = -3455466767;
    let b = 3.14; // by default this is f64
    let b = 3.14f64; 
    let c: f32 = 4.3;
    let bv = true;
    let tuple: (i32, bool)  = (13, false);
    let sentence: &str = "hello world! dans la Stack Memory";
    let mut strref: String = String::from("objet string Heap Memory- ");
    strref.push_str("toto");
    let array1 = ["toto", "tata", "titi"];
    println!(
        "{} {} {} {} {} {} {} {} {} {}  {}",
        mon_x, a, uu, ii, b, c, bv, tuple.0, tuple.1, sentence, strref
    );


    //------------------ Basic Type Conversion

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);

    //-----------------  Constants
    // Unlike variables, constants must always have explicit types.
    // Constant names are always in SCREAMING_SNAKE_CASE.
    println!(" Pi = {}",  PI  );

    //----------------- Arrays
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}