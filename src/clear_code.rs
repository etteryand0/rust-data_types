// const is global if declared outside of function
// you have to specify type (e.g. u32) for constants
const GLOBAL_CONSTANT: u32 = 10_000;

fn shadowing() {
    // note: x is immutable but you can shadow it with new variable x
    let x = 5;
    let x = x + 1;
    let x = x * 2;

    // also reassignment is useful when have to change variables data type
    let spaces = "    ";  // string
    let spaces = spaces.len()  // integer
}

fn reassignment() {
    // instead of shdowing you can reassign an existing mutable variable
    let mut x = 5;
    x = x + 1;
    x = x * 2;
}

fn main() {
    shadowing();
    reassignment();
}