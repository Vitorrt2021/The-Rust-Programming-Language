mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() -> bool {
            true
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_to_waitlist(){
        let result = front_of_house::hosting::add_to_waitlist();
        assert_eq!(result, true);
    }
}
