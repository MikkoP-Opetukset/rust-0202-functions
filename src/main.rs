#![allow(dead_code, unused_variables, unused_mut)]

// If a line starts with two forward slashes ( // ) it is a comment. If a line
// starts with three forward slashes ( /// ) it means it is a doc comment. They
// can be used to generate documentation automatically. Inside doc comments you
// can use markdown formatting.

/*
Block comments start with (see above) and end with (see below).
*/

/// Main function is the entry point to the application. This function is
/// required for a project that is executable.

fn main() {
    first_function(); // Calling a function
    println!("Instead of running the application, you probably want to read the source code.");
}

/// # A function that takes no parameters and returns nothing.
/// - `fn` is the keyword for defining a function.
/// - `{}` denote the function body.
/// - The naming convention is snake_case.

fn first_function() {
    println!("This is a simple function.");
}

/// # Function with parameters and a return value
/// - Functions require you to use type annotations in function parameters and
/// return values.
/// - After the `->` in the function signature comes the return type.

fn subtraction(a: i32, b: i32) -> i32 {
    let result = a - b;

    // Returning a value from a function is done by ending the function body in
    // an expression.
    // Rust does have the `return` keyword, but it is only used when it is
    // needed, e.g. early returns from guard clauses, or when it makes the code
    // easier to read, e.g. when exiting a function from inside a loop.

    result // Expression. Note that there is no semicolon at the end.
}

/// # Statements and expressions example
/// Functions are made from statements and expressions. Statements do
/// something, but return nothing. Expressions evaluate to a value.

fn statements_expressions() -> i32 {
    //
    // `let` is a statement. "Bind the value 'Apple' to the name fruit." It
    // does not return a value. Another example of an expression is the
    // function definition itself "Bind this block of code to the name
    // statements_expressions".
    let fruit = "Apple";

    // Most of the rest of Rust is expressions that evaluate into some value.
    // An expression can be a part of a statement.

    // Expression. Function call evaluates to the return value, or if it does
    // not have a return value, it evaluates to `()`. `()` is technically an
    // empty tuple, but it has a special name "unit." All expressions that
    // don't for one reason or another return any value evaluate to a unit.
    first_function();

    // `let` is a statement, as mentioned. "YYZ" is an expression that
    // evaluates into the string literal "YYZ".
    let rush = "YYZ";

    // Here we create an inner code block `{}` to demonstrate expressions. From
    // inside the code block we return something that then gets
    let meaning_of_life = {
        let x = 10;
        let y = 32;

        // Expression. x + y evaluates to a value. This is the value returns
        // from the code block.
        x + y
    };

    // Calling a macro is an expression. We won't talk a lot about macros
    // during the course, but they pop up every now and then. You can identify
    // a macro from the fact that it looks very similar to a function call, but
    // there is an exclamation mark between the name and the parenthesis.
    println!("I suppose the meaning of life is {meaning_of_life}");

    // Expression. This line does no operations, it just evaluates to the value
    // of 100.
    100
}
