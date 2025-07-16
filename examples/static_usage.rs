//! Example: Using dynenum with static-types feature

use dynenum::{Information, ALL_INFORMATION};
use std::str::FromStr;

/// Example function that takes an Information enum
fn handle_information(info: Information) {
    match info {
        Information::Email => println!("Action: Handle email!"),
        Information::PhoneNumber => println!("Action: Handle phone!"),
        Information::Date => println!("Action: Handle date!"),
        Information::Credential => println!("Action: Handle credential!"),
}

fn main() {
    println!("All static types:");
    for t in ALL_INFORMATION {
        let info = Information::from_str(t).unwrap();
        println!("Type: {} (variant: {:?})", info, info);
        handle_information(info.clone());
    }

    // Parse from string and match
    let input = "phone_number";
    match Information::from_str(input) {
        Ok(info) => {
            println!("Parsed: {}", info);
            match info {
                Information::Email => println!("It's an email!"),
                Information::PhoneNumber => println!("It's a phone!"),
                Information::Date => println!("It's a date!"),
                Information::Credential => println!("It's a credential!"),
            }
        }
        Err(e) => println!("Unknown type: {}", e),
    }
} 
