// use std::io; 
use std::cmp::Ordering;
use rand::Rng;
fn main(){

    let random_number_two= rand::thread_rng().gen_range(0,110);

println!("  now we are going to compare both these numbers" );
loop{ 
    let random_number_one= rand::thread_rng().gen_range(0,110); 
    match random_number_one.cmp(&random_number_two){ 
        Ordering::Less=>println!(" low "),
        Ordering::Equal=>{
            println!(" equal ");
            break; 
        }
            Ordering::Greater=>println!("high "),
    }
}}