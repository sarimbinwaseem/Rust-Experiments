
// Commands: 
// cargo build
// cargo run


fn main() {
    println!("Assalam o Alikum, buddy!");

    // Variable assignment
    // variables are immutable by default,
    // cannot change after assigning
    let walk = 45;
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
        // any changes made here is bound to be here..
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

}
