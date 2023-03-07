fn main() {
    hill();
    println!("I am main function...");
    hill();
    hill();

    let o = add_it(8, 90, 897.45);
    println!("{:?}", o);

    // new type of expression, I don't understand how it works.
    let snew = {
        let h = 67;
         h + 1
    };

    eprintln!("{:?}", snew);
}

fn hill() {
    println!("I am flippin' hill.. climb me...");
}

fn add_it(a: i32, b: i32, c: f32) -> f32 {
    return a as f32 + b as f32 + c;
    // or an expression at end of finction is returned without return keyword.
}
