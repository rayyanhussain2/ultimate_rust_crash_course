//-----------------------------------------------variables---------------------------------------------------------------------------
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

//-----------------------------------------------fn---------------------------------------------------------------------------
/*  snakecase, no variable length arguments, multiple types for a single arg; macro does like println!*/
//to make sense of arguments that dont have 
fn do_stuff(let qty: f64,let oz: f64) -> f64 
{
    return qty * oz; // same as 'qty * oz' 
}

//-----------------------------------------------modules---------------------------------------------------------------------------
use hello::greet; //fetch greet function that is public in the lib.rs file
use std::collections::HashMap; //go to the std project into the collections file and fetch hashmap
//crate is a package which you can install using cargo package like project sort of
//to import the standard package you need to put the package name and version under dependency 
fn main()
{
    //hello::greet(); // hello is the name of project in cargo.toml; :: will go to the lib.rs file and fetch greet() fn. but the fn in the lib needs to be public by pub
    greet(); // using it as a normal function
}
//-----------------------------------------------compound types---------------------------------------------------------------------------
//Tuple - can be of anytype, arity is 12
let info = (1, 3.3, 999);
let info1: (i32, u8, i64) = (1, 400, 299);

let jets = info.0;
let fuel = info.1;
let ammo = info.2;

let (jets1, fuel1, ammo1) = info;

//Array - multiple elements of the same type, lived on the stack, size of 32
let buf = [0, 0, 0];
let buf1 = [0; 3];
let buf2: [u8; 3] = [1, 2, 3];

let read1 = buf[0];
