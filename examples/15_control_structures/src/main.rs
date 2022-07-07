
fn conditionl_test() {

    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }
}



fn loop_test() {
    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }
    println!("{}", x);
}

fn loop_test2() {
   
        for  x in 2..4 {
            println!("{}", x);
        }
    
        for x in 4..=5 {
            println!("{}", x);
        }
}

fn loop_test3() {
   
        let mut x = 0;
        let v = loop {
            x += 1;
            if x == 13 {
                break "found the 13";
            }
        };
        println!("from loop: {}", v);
    
}


fn match_test() {
    let x = 42;

    match x {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }
}


fn example() -> i32 {
    let x = 42;
    // Rust's ternary expression  (boolexpress) ? true : false
    let v = if x < 42 { -1 } else { 1 };
    println!("from if: {}", v);

    let food = "hamburger";
    let result = match food {
        "hotdog" => "is hotdog",
        // notice the braces are optional when its just a single return expression
        _ => "is not hotdog",
    };
    println!("identifying food: {}", result);

    let v = {
        // This scope block lets us get a result without polluting function scope
        let a = 1;
        let b = 2;
        a + b
    };
    println!("from block: {}", v);

    // The idiomatic way to return a value in rust from a function at the end
    v + 4
}

fn main() {
    println!("from function: {}", example());
}



