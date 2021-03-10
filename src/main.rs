fn main() {
    /* Annotations */
    let user_age_from_input: i8 = "27".parse().expect("Not a number!");
    println!("{}", user_age_from_input);

    // This variable need annotations
    // let user_age_from_input = "27".parse().expect("Not a number!");

    /* Scalar Types */

    /* Scalar types represent only one value. Currently, we have four primary scalar types in Rust. 
    These are integers, floating-point numbers, booleans and characters. */
    
    // 1. Integer Types
    
    // cannot apply unary operator `-` to type `u8`
    // let user_age_from_input: u8 = -8;
    let user_age_from_input: i8 = -8;
    println!("My age is {}", user_age_from_input);

    let my_age = 27; // i32 by default
    println!("My age is {}", my_age);

    // 2. Floating-Point types
    
    let amount = 35.50; // f64
    println!("Amount f64 {}", amount);
    
    let total: f32 = 400.45; //f32 
    println!("Amount f32 {}", total);

    /* Numeric Operations */
    
    let sum_example = 19 + 1; // addition
    println!("addition {}", sum_example);
    
    let get_difference = 2020 - 1993; // subtraction
    println!("get_difference {}", get_difference);
    
    let multiply_it = 10 * 2; // multiplication
    println!("multiply_it {}", multiply_it);
    
    let divide_it = 30 / 2; // division
    println!("divide_it {}", divide_it);
    
    let get_mod = 10 % 2; // remainder
    println!("get_mod {}", get_mod);

    /* 3. Boolean Type */
    
    let is_completed = false;
    println!("is_completed {}", is_completed);

    // explicit type annotation
    let is_send: bool = true;
    println!("is_send {}", is_send);

    /* 4. Character Type */
    let symbol = '₺';
    println!("Symbol is {}", symbol); // Symbol is ₺
    
    /* Compound Types */
    
    /* Compound types can group multiple values into one type. 
    Rust has two primitive compound types. These are tuple and arrays. */

    // 1. Tuple type
    let user_tuple: (i32, f64, bool) = (2019, 1500.76, true);
    println!("User tuple is {:?}", user_tuple);
    // or
    let user_tuple = (2019, 1500.76, true);
    println!("User tuple is {:?}", user_tuple);

    // Destructing
    let user_tuple = (2019, 1500.76, true);
    let (register_year, balance, is_customer) = user_tuple;
    // let (register_year, balance, is_customer) = (2019, 1500.76, true);
    println!("User's balance is: {}", balance);
    println!("User's register_year is: {}", register_year);
    println!("User's is_customer is: {}", is_customer);

    // Accessing Tuple Indexes
    let user_tuple = (2019, 1500.76, true);
    println!("User's balance is: {}", user_tuple.1);

    /* Array Type */
    let users = ["Ali", "Ben", "Burak", "Enes"];
    println!("Users: {:?}", users);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months: {:?}", months);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    /*------type: ^^^--size--*/
    println!("Numbers: {:?}", numbers);
    // or
    let numbers = [1; 5];
    /*------value: ^--size--*/
    println!("Numbers: {:?}", numbers);

    // Accessing Array Elements
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("First element is: {}", numbers[0]);

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    // println!("Number is: {}", numbers[5]);
    // index out of bounds: the len is 5 but the index is 5
    println!("Number is: {}", numbers[4]);
}
