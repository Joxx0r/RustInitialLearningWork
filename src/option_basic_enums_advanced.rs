

#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska
}

enum Coin {
    Penny,
    Nickel(UsState),
    Dime,
    Quarter,
}

fn add_value(value:Option<i32>) -> Option<i32> 
{
    match value {
        None => None,
        Some(value) => Some(value + 1),
    }
}

fn value_in_cents(coin:Coin) -> u32 {
    match coin {
        Coin::Nickel(state) =>
        {
            println!("State {:?}", state);
            1
        } 
        Coin::Penny => 
        {
            println!("Here is penny value");
            2
        },
        Coin::Dime => 3,
        Coin::Quarter => 25,
    }
}

fn main() {
    let home:IpAddrKind = IpAddrKind::V4(127, 0, 0, 1);
    let loopback:IpAddrKind = IpAddrKind::V6(String::from("#1"));
    
    println!("home {:?}", home);
    println!("loopback {:?}", loopback);


    let value_dime = value_in_cents(Coin::Dime);
    let value_penny = value_in_cents(Coin::Penny);
    let value_quarter = value_in_cents(Coin::Quarter);
    let value_nickel = value_in_cents(Coin::Nickel(UsState::Alabama));
    println!("Dime value: {} Penny value: {} Nickel value Alabama {} Quarter value: {}", value_dime, value_penny,value_nickel, value_quarter);


    let five = Some(6);
    let six = add_value(five);
    let none = add_value(None);

    if let Some(5) = five {
        println!("Here is five");
    }
    else if let Some(6) = five {
        println!("here is six");
    }
}