let bunnie: i32 = 16;
let bunnies = 16;
let (a, b) = (6, 5);
//variables are immutable by default
let mut c = 16;
c = 12;
//constant - can be global, inlined at compile time, fast
const WARP_FACTOR: i32 = 12;

//out of scope variables are detected at compile time, variables are shadowed;
let mut x = 5;
let x = x ; //x is now immutable because we dropped the mutation. old value of mut x is new value of immutable x
let x = "Rayyan";
{
    let x = 99;
    println!("{}", x);
}
println!("{}", x);
//if a variable is  used before being initialised compiler will throw error; safety