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