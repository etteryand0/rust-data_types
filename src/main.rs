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


fn main() {
    println!("{}Hello, data types!", GREEN);
    constants();
}