fn main() {
    //SCALARS
    // Integers
    // signed integer types start with i, instead of u

    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16	    u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    let scalar:u32 = 768;
    println!("The value of scalar:u32 is: {}", scalar);
    let scalar:i16 = -243;
    println!("The value of scalar:i16 is: {}", scalar);


    // Note that number literals that can be multiple numeric types allow a type suffix, such as 57u8, to designate the type. Number literals can also use _ as a visual separator to make the number easier to read, such as 1_000, which will have the same value as if you had specified 1000.
    // Number literals	    Example
    // Decimal	            98_222
    // Hex	                0xff
    // Octal	            0o77
    // Binary	            0b1111_0000
    // Byte (u8 only)	    b'A'

    let literals = 0x432;
    println!("The value of literals hex is: {}", literals);
    let literals = 0o6737;
    println!("The value of literals octal is: {}", literals);


    // Floating points
    // Rust has two primitive types for floating-point numbers,
    // f32 and f64, which are 32 bits and 64 bits in size, respectively
    //  The default type is f64

    let x = 2.0; // f64
    println!("The value of floating:f64 is: {}", x);

    let y: f32 = 3.0; // f32
    println!("The value of floating:f32 is: {}", y);



    // NUMERIC OPERATION
    // Rust supports the basic mathematical operations you’d expect for all of the number types: addition, subtraction, multiplication, division, and remainder

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {}", sum);

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {}", difference);

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {}", product);

    // division
    let quotient = 56.7 / 32.2;
    println!("The value of division quotient is: {}", quotient);

    let floored = 2 / 3; // Results in 0
    println!("The value of division floored is: {}", floored);

    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {}", remainder);


    //BOOLEAN
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("The value of t is: {}, f is: {}", t,f );



    // The Character Type
    // we specify char literals with single quotes, as opposed to string literals, which use double quotes. 
    // Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII. 
    let c = 'z';
    println!("The value of c is: {}", c);
    let z = 'ℤ';
    println!("The value of z is: {}", z);
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);


    //COMPOUND TYPES
    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.
    
    //TUPLE
    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // This is called destructuring,
    let (_x, y, _z) = tup;
    // We can also access a tuple element directly by using a period (.) followed by the index 
    println!("The value of y is: {}, x is:{}", y, tup.0);

    // The tuple without any values, (), is a special type that has only one value, also written (). The type is called the unit type and the value is called the unit value. Expressions implicitly return the unit value if they don’t return any other value.

    //ARRAY TYPE
    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type.
    // arrays in Rust have a fixed length.

    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    
    //type i32, with len 5
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // You can also initialize an array to contain the same value for each element
    let a = [3; 5];//let a = [3, 3, 3, 3, 3];

    // You can access elements of an array using indexing
    let _first = a[0];
    let _second = a[1];



}
