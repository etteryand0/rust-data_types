const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const CLEAR: &str = "\x1b[0m";

fn constants() {
    println!("{}Let`s learn constants{}", YELLOW, CLEAR);

    const MAX_POINTS: u32 = 100_000;

    println!("const MAX_POINTS: u32 = 100_000; // is constant which containts u32 (integer)");
    println!("MAX_POINTS value is {}", MAX_POINTS);
    println!("It is useful for global values that other components also need to know");
    println!("{}YOU CAN`T USE mut WITH CONSTANTS!!!\n{}", RED, CLEAR);
}

fn shadowing() {
    println!("{}Let`s learn shadowing{}", YELLOW, CLEAR);
    println!("You can shadow mutable/immutable variables by using {}let{}", GREEN, CLEAR);
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("{}x{} = {}", GREEN, CLEAR, x);

    println!("Or you can reassign");
    let mut x = 5;
    x = x + 4;
    x = x * 2;
    println!("{}mut x{} = {}", GREEN, CLEAR, x);

    println!("Shadowing is useful when you want to change the data type but use the same variable name");
    let spaces = "     ";
    let spaces = spaces.len();
    println!("{}spaces{} = {}", GREEN, CLEAR, spaces);
    println!("{}spaces{} was string before. Now it is integer", GREEN, CLEAR);
}

fn main() {
    println!("{}Hello, data types!", GREEN);
    constants();
    shadowing();
}