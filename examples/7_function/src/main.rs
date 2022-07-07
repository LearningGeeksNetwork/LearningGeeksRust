fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn subtract(x: i32, y: i32) -> i32 {
    x - y
}

fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn make_nothing() -> () {

    //  empty tuple, also known as a unit.
    return ();
    
}

// the return type is implied as ()
fn make_nothing2() {
    //  empty tuple, also known as a unit.
    // this function will return () if nothing is specified to return
}


fn main() {
    println!("42 + 13 = {}", add(42, 13));
    println!("42 - 13 = {}", subtract(42, 13));

    //---------------------------------

    // return a tuple of return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);


    //-------------------------------

    let a = make_nothing();
    let b = make_nothing2();

    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);

}

// Calling Methods :  static methods,  instance methods
fn test() {
    // Using a static method to create an instance of String
    let s = String::from("Hello world!");
    // Using a method on the instance
    println!("{} is {} characters long.", s, s.len());
}



