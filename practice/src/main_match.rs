#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> i8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            return 25;
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    return match x {
        None => None,
        Some(i) => Some(i+1)
    }
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("1 plus one result was {:?}", plus_one(Some(1)));
    println!("5 plus one result was {:?}", plus_one(Some(5)));
    println!("None plus one result was {:?}", plus_one(None));
}