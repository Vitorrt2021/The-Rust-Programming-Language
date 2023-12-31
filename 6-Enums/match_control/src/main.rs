fn main() {
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }
    
    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    #[derive(Debug)] // so we can inspect the state in a minute
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum CoinV2 {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents_v2(coin: CoinV2) -> u8 {
        match coin {
            CoinV2::Penny => 1,
            CoinV2::Nickel => 5,
            CoinV2::Dime => 10,
            CoinV2::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);


    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    let dice_roll_v2 = 10;
    match dice_roll_v2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn reroll() {}
    fn add_fancy_hat() {}
    fn remove_fancy_hat() {}
    fn move_player(num_spaces: u8) {}
}
