//expressions return value, statements don't like if vs let num = 5;
let mut msg;
if num == 5{
    msg = "five";
}
else if num == 4 {
    msg = "four";
} else {
    msg = "other";
}
//can be used as like this as well. Similar to ternary operator. Can't use return explicitly. All return types must be same. Semicolon at the end
msg = if num == 5 {
    "five"
} else if num == 4 {
    "four"
} else {
    "other"
};

//unconditional loop
'bob: loop { //label
    if !dizzy()
    {
        break;
    }
    loop {
        loop{
            continue bob;
        }
        break bob;
    }
}

while dizzy(){
    //do stuff
}

//iter
for num in [7, 8, 9].iter()
{
    //do stuff with num
}

let array = [(1,2), (3, 4)]; //array of tuples of the same type
for(x, y) in array.iter(){
    //do stuff with x and y
}

//ranges
for num in 0..5{//exlcusive of 5
    //do stuff with num
}
for num in 0..=5{//inclusive of 5
    //do stuff with num
}