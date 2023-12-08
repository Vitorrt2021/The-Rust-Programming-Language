mod back_of_house;
mod front_of_house;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use crate::front_of_house::hosting;
use crate::front_of_house::hosting::seat_at_table as seat;
use crate::front_of_house::{serving::serve_order, hosting::add_to_waitlist};
use crate::front_of_house::*;

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

    #[test]
    fn add_to_waitlist_with_absolute_path(){
        let result = crate::front_of_house::hosting::add_to_waitlist();
        assert_eq!(result, true);
    }

    #[test]
    fn use_super_keyword(){
        let result = super::front_of_house::hosting::seat_at_table();
        assert_eq!(result, true);
    }

    #[test]
    fn use_use_keyword() {
        let result = hosting::seat_at_table();
        assert_eq!(result, true);  
    }

    #[test]
    fn use_as_keyword() {
        let result = seat();
        assert_eq!(result, true);  
    }

}
