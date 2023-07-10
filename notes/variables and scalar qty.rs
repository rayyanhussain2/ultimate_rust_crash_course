//scalar types
//booleans are not integers by default unless you set true as u8 or smn. underscores are ignored in integers and float 1_6 is same as 16
//char is 4 bytes or 32 bits; UTF - 32 encoding standard. 
//i32 and f64 is default types
//strings are utf-8 or 8 bit characters. so strings don't use char internally. source files are utf 8 look bottom right. so mostly we deal with 
//utf - 8 strings
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