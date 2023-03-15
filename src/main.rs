// fn main(){
//     let mut x = 5;
//     println!("The value of x is {x}");
//     x = 32;
//     println!("The value of y is {x}");
//     const _PI: u32 = 21;
//     let x = 32;
//     println!("The value of x is {x}");
//     let x = 3 + x;
//     println!("The value of x now is {x}");
//     {
//         let x = x*2;
//         println!("The value of x inside is {x}");
//     }

//     println!("Now the value of x is {x}");

//     // with mut we can change the value but not the datatype 
//     // with let the prev. var. is shadowed by completly new var. as a result we can declare this new var with diffrent datatype
//     // using mut and changing datatype will return error

//     let spaces = "   ";
//     let spaces = spaces.len();
//     println!("the value of spaces is {spaces}");

//     // printing the value of tuple
//     let tup = (1, 2, 3);
//     // destructuring
//     let (x, y, z) = tup;

    
    
//     println!("({x}, {y}, {z})");

//     let x: (i32, f64, u8) = (500, 6.4, 1);
//     let a = x.0;
//     let _b = x.1;
//     println!("a = {a}");

//     let _arr: [i32; 4] = [1, 2, 3, 4];
//     let _first = _arr[0];
//     let _b = [3; 4];
    
// }

// // u32 means unsigned integer and has a explicit size

// // integer overflow
// /* if you assign the value outside the integers range integer overflow will occure
// when compiling with --release flag rust performs integer warpping hence the program 
// dosent panic but relying on integer warpping in rust is not considered as a good practice

// To explicitly handle the problem of overflow use following funcs provided by std lib

// 1. warping_* like warpping_add
// 2. return None for checked_* methodes
// 3. Return the boolean value for overflowing_*
// 4. Saturate the maximum and minimum value using saturating_* funcs 

// Compound Types
// 1, Tuple 
// A tuple is immutable */

use std::io;

fn main(){
    let a = [1, 2, 3];
    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index 
        .trim()
        .parse()
        .expect("Index entered is not a number");

    let element = a[index];

    println!("The value of the element at index {index} is {element}");
}