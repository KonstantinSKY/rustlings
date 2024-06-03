// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let a:[u8; 101] = [
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 32, 34, 92, 34, 23, 55, 12, 23, 23, 232,
        12, 23
        ];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
