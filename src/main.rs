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

fn integer() {
    // int, float, bool, char
    // INTEGER TYPES
    // Length    Signed  Unsigned  Size
    // 8-bit     i8      u8        -128:127 || 0:255
    // 16-bit    i16     u16       -32_768:32_767 || 0:65_535
    // 32-bit    i32     u32       -2_147_483_648:2_147_483_647 || 0:4_294_967_295
    // 64-bit    i64     u64       -9_223_372_036_854_775_808:9_223_372_036_854_775_807 || 0:18_446_744_073_709_551_615
    // 128-bit   i128    u128      -170_141_183_460_469_231_731_687_303_715_884_105_728:170_141_183_460_469_231_731_687_303_715_884_105_727 || 0:340_282_366_920_938_463_463_374_607_431_768_211_455
    // arch      isize   usize     64-bit if on 64-bit architecture || 32-bit if on 32-bit architecture
    // signed can be negative and positive
    // unsigned always positive
    // formula for signed is -(2**(n-1)) to 2**(n-1)-1
    // and for unsigned is 0 to 2**(n)-1

    // All number literals except the byte literal allow a type suffix, such as 57u8, and _ as a visual separator, such as 1_000
    // Number_literals  Example
    // Decimal          98_222
    // Hex              0xff
    // Octal            0o77
    // Binary           0b1111_0000
    // Byte (u8)        b'A'
}

fn main() {
    println!("{}Hello, data types!", GREEN);
    constants();
    shadowing();
    // scalar types
    integer();
}