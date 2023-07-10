use hello::greet; //fetch greet function that is public in the lib.rs file
use std::collections::HashMap; //go to the std project into the collections file and fetch hashmap
//crate is a package which you can install using cargo package like project sort of
//to import the standard package you need to put the package name and version under dependency 
fn main()
{
    //hello::greet(); // hello is the name of project in cargo.toml; :: will go to the lib.rs file and fetch greet() fn. but the fn in the lib needs to be public by pub
    greet(); // using it as a normal function
}