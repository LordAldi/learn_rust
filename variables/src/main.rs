fn main() {

    // by default variables are immutable
    // However, you still have the option to make your variables mutable.
    

    //wrong because its immutable by default
    /*let x =5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);*/

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // you aren’t allowed to use mut with constants. Constants aren’t just immutable by default—they’re always immutable.
    // the type of the value must be annotated
    // constants may be set only to a constant expression, not the result of a value that could only be computed at runtime.
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("three hours is {}'s", THREE_HOURS_IN_SECONDS);



    //SHADOWING
    // you can declare a new variable with the same name as a previous variable
    let x = 5;

    let x = x + 1;//x=6

    {
        let x = x * 2;//x=12
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);//x=6

    // Shadowing is different from marking a variable as mut, 
    // because we’ll get a compile-time error if we accidentally try to reassign to this variable without using the let keyword. 
    // The other difference between mut and shadowing is that because we’re effectively creating a new variable when we use the let keyword again, we can change the type of the value but reuse the same name

    //not work because mut not shadowing
    /*let mut spaces = "   ";
    spaces = spaces.len();*/

    let spaces = "   ";
    let spaces = spaces.len();

    println!("spaces length is {}", spaces);


}
