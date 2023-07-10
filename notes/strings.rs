/*a borrowed string slice is a string with capacity equal to its length
a string may have capacity greater than its length - like conventional c
both are utf 8
a literal string is a borrowed string because its capcacity is its length. let l = "hi!";
can't index a string in accordance to its character position
utf 8 is 8 bits but since the total number of characters are more than 256, it uses 1 to 4 bytes it uses bit 6 and 7 to indicate if there ar emore bytes
and that's the reason we cant really access string character as byte positions because a byte can be a continuation of the previous byte for the 
info for a full symbol
*/

let str1 = "hello"; //borrowed string
let str2 = "Hello".to_string();
str1.bytes() //iterates through the bytes of the string
str1.chars() // best way to iterate through chars of string
//cant modify the string characters
