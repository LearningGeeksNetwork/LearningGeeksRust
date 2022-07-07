

struct Location(i32, i32);

fn structs_like_tuple() {
    // This is still a struct on a stack
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
}



struct Marker;

fn main() {
    let _m = Marker;
}

