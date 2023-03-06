    // Datatypes
    // i8 => signed integer.
    // i16
    // i32
    // i64
    // i128

    // u32 => unsigned number.
    // u64
    // u128

    // f32 => floating point data...
    // f64

    // bool: true or false

    // char: a single letter: 'd'

    // tuple and arrays
    // can be mutable by "mut" keyword.

    // Arrays:
    // fixed length and datatype, unlike tuple which has multiple datatypes
    // elements accessed with [<index_number>]
    // let <var_name>: [<datatype>; <num_of_elements>]; 
fn main() {
    let mut list = (34, false, 'g');
    // array and tuple elements are accessed like this: list.<index_number>
    println!("{:?}", list.2); // prints g
    list.2 = 't';
    println!("{:?}", list.2);

    // Array

    let mut arr: [i32; 23] = [0, 1, 2, 3, 4];
    println!("{:?}", arr[3]);
    arr[2] = 90;
    println!("{:?}", arr[2]);
}
