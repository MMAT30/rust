fn main() {


    // byte size        signed      unsigned
    // 8-bit            i8	        u8
    // 16-bit	        i16	        u16
    // 32-bit	        i32	        u32
    // 64-bit	        i64	        u64
    // 128-bit	        i128	    u128
    // arch	            isize	    usize
    // f32              f32	        -
    // f64              f64	        -


    // numeric literals
    // Decimal	            98_222
    // Hex	                0xff
    // Octal	            0o77
    // Binary	            0b1111_0000
    // Byte (u8 only)	    b'A'
    // Bool	                true
    // Char	                'a'
    // String	            "abc"
    // Tuple	            (1, true)
    // Array	            [1, 2, 3]


    // creating a constant
    const MAX_POINTS: u32 = 100_000;
    println!("The value of MAX_POINTS is: {MAX_POINTS}");

    // creating a immutable variable
    let x = 5;
    println!("The value of x is: {x}");

    // creating a mutable variable
    let mut y = 10;
    println!("The value of y is: {y}");

    // changing the value of a mutable variable
    y = 15;
    println!("The value of y is: {y}");


    // shadowing - allows us to change the type of the variable without being mutable
    let z = "Hello";
    println!("The value of z is: {z}");

    let z = z.len();
    println!("The value of z is: {z}");


    // creating a tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("The value of tup is: {tup}");

    // creating a array
    let arr = [1, 2, 3, 4, 5];
    println!("The value of arr is: {arr}");

    // creating a sized array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of arr is: {arr}");



}
