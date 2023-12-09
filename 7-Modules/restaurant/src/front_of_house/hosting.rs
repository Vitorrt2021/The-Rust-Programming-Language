pub fn add_to_waitlist() -> bool {
    true
}

pub fn seat_at_table() -> bool {
    super::serving::serve_order()
}