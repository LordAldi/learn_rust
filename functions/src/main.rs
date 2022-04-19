//FUCTIONS
// the main function is the entry point of many programs.
// fn keyword allows you to declare new functions.
fn main() {
    println!("Hello, world!");
    another_function();
    another_function_with_params(6);
    print_labeled_measurement(5, 'h');
    statements();
    func_with_ret();
}
fn another_function() {
    println!("Another function.");
}

// PARAMETERS
// When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments,

fn another_function_with_params(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}


//Statements and Expressions
// Statements are instructions that perform some action and do not return a value. Expressions evaluate to a resulting value. Let’s look at some examples.

fn statements() {
    // Creating a variable and assigning a value to it with the let keyword is a statement.
    let _y = 6;

    // let x = (let y = 6); wrong
    // The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. 


    //Expressions can be part of statements
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of y is: {}", y);//y=4 (3+1)
}


// Functions with Return Values
// We don’t name return values, but we must declare their type after an arrow (->)
// In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function
// You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

// The 5 in five is the function’s return value, which is why the return type is i32
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
    // x + 1;
    //But if we place a semicolon at the end of the line containing x + 1, changing it from an expression to a statement, we’ll get an error.
}

fn func_with_ret() {
    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);
}