use std::io;
// io library is the input output library 
use rand::Rng;

fn main() {
    println!("Guess the number!");
    //  remember println! is A macro 
    // anything with a ! in the end is a macro 
    println!("Please input your guess.");
    let a_secret_number=rand::thread_rng().gen_range(1,101); 
    println!("the secret number = {}", a_secret_number);
    let mut guess = String::new();
    // mut means mutable if you remember 
    // :: means that new is being impolemented on 'String'
    io::stdin() // is we wldnt have dec above then we wld have written std::io::stdin
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed : {}", guess); 

    // let just_another_random_variabl="234";
    // println!(" this is the variable {}", just_another_random_variabl);

}


 // !crate is just a collection of rust source files 
 //~ crate update will update all the crates to the latest version 