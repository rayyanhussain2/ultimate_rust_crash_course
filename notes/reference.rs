let mut s1 = "Hi!";

do_stuff(&s1);
//the value of s is &s1 which is a refernece to s1. s1's value is not being moved
/*
Lifetimes
References must always be valid. Can't create a reference to a value that does not exist
Never can point to NULL 
*/

fn do_stuff(s: &mut String) { //s is a address to a mutable string, another layer of protection when modifying the value of s1 even though it may be mutable
    //s is mutable now
    s.insert_str(0, "Hi, ");//auto derefences to the value on the heap
    *s.insert_str(0, "Hi, "); // valid
    *s = String :: from("Replacement");
    //do stuff
}
/*
only one mutable reference, and many immutable references to any variable

C does not have the concept of ownership and moving, so it creates a cokpy and wherever it can not copy those large arrays, it uses references
sort of half baked story on the part of C

In a nutshell, the purpose of references is to access a variable indirectly so that you don't need to move the ownership of a value and rather
access a variable's value using the address of the variable. Additionally, the reference has mut and immutable properties, again, because
we are thinking of it as a indirect access to the variable. Since that pointer will be how we will be controlling the variable,
we have mutable and immutable
references.
*/
