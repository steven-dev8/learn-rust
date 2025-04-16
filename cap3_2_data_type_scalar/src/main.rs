fn main() {
    //scalar types: represents a single value.
    //four scalar types: int, floar, bool and char

    // Integer types
    // lenght | signed | unsignes
    // 8bits  |   i8   |    u8
    // 16bits |   i16  |    u16
    // 32bits |   i32  |    u32
    // 64bits |   i64  |    u64
    // 128bits|  i128  |    u128
    // arch   |  isize |    usi
    // 2^n - 1 unsigned, - 2^n até 2^n -1

    let x: i8 = -13;
    println!("{x}");

    // Error: let x: u8 = -13;
    // Error: const X = 13;
    // Error: let x: i8 = 1000; x > 2^8 = 128

    // Rust suport number literal
    // Decimal : 98_222 = 98222
    // Hex : 0xff
    // Binary : 0b1111_0000
    // Byte : b'A'

    // Float types : f32 and f64
    let y = 2.0; // f64
    let z: f32 = 3.0;
    
    // Bool types: true and false
    let t = true;

    let f: bool = false;

    // Char is a alphabetic type, for exemple:
    let c = 'z';
    let d = 'Z';
    let e = '😻';

    // note specify literal char with single quotes,
    // diferente as string literal

    
}
