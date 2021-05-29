#[derive(Debug)]
pub enum USState {
    Alabama,
    Alaska,
    Arizona
}

#[derive(Debug)]
pub enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(USState)
}

pub fn get_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}