fn main(){
    let mut x = 5;
    println!("The value of x is {x}");
    x = 32;
    println!("The value of y is {x}");
    const _PI: u32 = 21;
    let x = 32;
    println!("The value of x is {x}");
    let x = 3 + x;
    println!("The value of x now is {x}");
    {
        let x = x*2;
        println!("The value of x inside is {x}");
    }

    println!("Now the value of x is {x}");

    // with mut we can change the value but not the datatype 
    // with let the prev. var. is shadowed by completly new var. as a result we can declare this new var with diffrent datatype
    // using mut and changing datatype will return error

    let spaces = "   ";
    let spaces = spaces.len();
    println!("the value of spaces is {spaces}");
}