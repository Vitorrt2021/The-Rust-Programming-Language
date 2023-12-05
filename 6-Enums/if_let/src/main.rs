enum OddEven {
    Odd,
    Even
}

fn check_odd_even(value: i32) -> OddEven {
    if value == 0 {
        OddEven::Even
    } else {
        OddEven::Odd 
    }
}

fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let num = 3;
    if let OddEven::Odd = check_odd_even(num) {
        println!("{} its Odd", num);
    } else {
        println!("{} its Even", num);
    }

}
