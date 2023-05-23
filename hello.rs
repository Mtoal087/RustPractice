//////////////////////////
//                      //
//      Matt Toal       //
//      Rust Syntax     //
//                      //
//////////////////////////

// Main Function
fn main() {
    println!("Hello, World!");

    let company_string = "Matt Toal";                   // string
    let rating_float = 4.5;                             // float
    let growing_boolean = true;                         // bool
    let icon_char = 'P';                                // char

    println!("\n");
    println!("Company name is: {}", company_string);
    println!("The rating for the company out of 5 is: {}", rating_float);
    println!("Is the company growing? {}", growing_boolean);
    println!("Abbreviation for the company is: {}", icon_char);
    println!("\n");

    const PI:f32 = 3.141592;                                // constant
    println!("The value of pi will always be {}\n", PI);

    // Arithmetic
    let a = 5;
    let b = 22;
    let c = a+b;
    println!("a + b = {}", c);
    println!("a + b + a = {}\n", c + a);

    // If Statements
    if c > 5 {
        println!("a + b is greater than 5");
    } 
    else if c < 5 {
        println!("a + b is less than 5");
    }
    else {
        println!("a + b equals 5");
    }
    println!("\n");

    // Match Statements -> similar to switch cases in C++
    let code = "MO";
    let state = match code {
        "KS" => "Kansas",
        "MO" => "Missouri",
        "KY" => "Kentucky",
        "TX" => "Texas",
        "FL" => "Florida",
        _ => "Unknown"
    };
    println!("State name is {}\n", state);

    println!("For Loop");
    // For Loops
    for x in 1..11 {        // 11 is NOT included
        println!("x is: {}", x);
    }
    println!("\n");

    println!("While Loop");
    // While Loops                                              // 'break' and 'continue' keywords works the same way as C++
    let mut x = 0;
    while x < 10 {
        println!("Inside loop, x = {}", x);
        x += 1;
    }
    println!("Outside loop, x = {}\n", x);


    // Calling function
    function_hello();

    // Calling another function with parameters
    x = 4;
    another_hello(x);
    println!("\n");

    // Function with a return statement
    let x = get_value();
    println!("In the 'get_value' function, it returns the value {}", x);

    // Another function 
    println!("The value of pi is {}", get_pi());
    println!("\n");


    // Tuples
    let tuple:(i32, f64, u8) = (-325, 4.9, 22);
    println!("{:?}", tuple);
    println!("Integer is: {}", tuple.0);
    println!("Float is: {}", tuple.1);
    println!("Unsigned int is: {}", tuple.2);
    println!("\n");

    // Tuples inside functions
    let b: (i32, bool, f64) = (110, true, 10.9);
    print(b);
    println!("\n");


    // Arrays
    let array1: [i32;3] = [3,7,1];
    println!("Array is {:?}", array1);
    println!("Size of array is {}", array1.len());

    // Array in For Loop
    for i in 0..3 {
        println!("Index is: {} & value is: {}", i, array1[i]);
    }

    // Passing Arrays as Parameters for Functions
    let arr = [10, 20, 30];
    update(arr);
    println!("Inside main {:?}\n", arr);


    // Ownership
    let v = vec![1,2,3];
    let v2 = v;
    // println!("{:?}", v);                     Creates an error because 'v' has lost ownership of vector

    // Passing Value to a Function
    display(v2);                                // v2 is moved to display and becomes invalidated
    // println!("In main {:?}", v2);            v2 is no longer usable here

    // Returning value of a function
    let v3 = vec![2,4,6];
    let v2_return = display2(v3);
    println!("In main {:?}\n", v2_return);
}

//////////////////////////////      FUNCTIONS        ////////////////////////////////////////////////////////////////////////////////////////////////
/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn function_hello() {
    println!("Hello from a seperate function!\n");
}

fn another_hello(x: i32) {
    for i in 0..x {
        println!("Hi! {}", i);
    }
}

fn get_value() -> i64 {
    let integer = 26;
    return integer;
}

fn get_pi() -> f64 {
    let x = 22.0 / 7.0;
    return x;
}

fn print(x:(i32, bool, f64)) {              // Pass tuple as a parameter
    println!("Inside print function");
    println!("{:?}", x);

    let (age, is_male, cgpa) = x;
    println!("Age is {}, isMale? {}, cgpa is {}", age, is_male, cgpa);
}

fn update(mut arr:[i32;3]) {
    for i in 0..3 {
        arr[i] = 0;
    }
    println!("Inside update {:?}", arr);
}

fn display(v:Vec<i32>) {
    println!("Inside of display {:?}", v);
}

fn display2(v:Vec<i32>) -> Vec<i32> {
    // returning the same vector
    println!("Inside display2 {:?}", v);
    return v;
}