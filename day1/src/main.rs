use std::io;
// Commands: 
// cargo build
// cargo run


fn main() {
    println!("Assalam o Alikum, buddy!");

    // Variable assignment
    // variables are immutable by default,
    // cannot change after assigning
    let walk = 47;
    println!("Nicely done: {}", walk); // <- immutable variable

    // to make variable mutable, use "mut"
    let mut far = 900; // <- mutable variable
    println!("{:?}", far);
    far = 8900;

    println!("{}", far);

    // OR
    // redefine variable with let
    // can change data type now i.e. u32 to string

    let y = "Mario";
    println!("{:?}", y);

    let y = "Giggle";

    println!("{:?}", y);

    // Name Shadowing...

    {
        // This is a scope.
        // Anything defined here is gonna be here ...
        // any changes made here are bound to be here..
        // But external values can be accessed here..
        // like:
        println!("Inside scope: {:?}", y);
        let y = "Bubbl";
        println!("Inside scope after changing value: {:?}", y); // Prints bubbl
    }

    println!("Exited from scope: {:?}", y); // Prints Giggle

    // Constants.
    // Must be all capital letters.
    // separated by _.
    // Must have a datatye defined.
    // And value decleared
    // cannot assign again like "let".
    const MY_FLIPPING_VARIABLE: u32 = 7600;

    println!("Constant variable: {:?}", MY_FLIPPING_VARIABLE);

    //  ############ USER INPUT #############

    // User Input
    // Have to use std:io crate (package) to take user input.
    // In RUST, . of Python is ::
    println!("Enter: ");
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read dummy..!!");

    println!("{}", user_input);

    // ######### Basic Maths #########

    // different datatypes do not "math up".
    // need to be same like i32 + i32
    // i64 and i32 do not work in maths..

    let q: i32 = 345;
    let w: i32 = 6696;

    let g: i32 = q + w;
    println!("{}", g);

    let g: i32 = q - w;
    println!("{}", g);

    let g: i32 = q / w;
    println!("{}", g);

    let g: i32 = q * w;
    println!("{}", g);

    // #$#$#$#$#$ TYPE CASTING #$#$#$#$#$

    
    let q = 345u32;
    let w = 6696u32;

    let q = 345 as i64;
    let w = 6696 as i64;

    let mut user_new_input = String::new();
    println!("Enter Input:");
    io::stdin().read_line(&mut user_new_input).expect("expected to read the line.");

    let string_to_number: i64 = user_new_input.trim().parse().unwrap();

    println!("{}", string_to_number + 45);

}
