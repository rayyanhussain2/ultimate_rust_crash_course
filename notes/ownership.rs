let mut s1 = "Hi!";
let s2 = s1;

s1 = do_stuff(s1);

fn do_stuff(s : String) {
    s;
}
/*
1. Every value has an owner,
2. Only one owner
3. Values get dropped once once user goes out of scope
s2 has moved the data from s1 to point to its data stored on the heap. s1 on the stack now points to nothing. this is called copying where 
the data on the stack is copied. When data on the heap is also copied, this is called cloning
after s2 moved the value from s1, since s1 points to nothing, its considered uninitialised, and because its immutable, its just garbage.
it still exists on the stack though. If s1 was mutable, we could assign it some new value and then use it again*/
