use uuid::Uuid;

struct Transaction {
    id: Uuid,
    user_id: u64,
    amount: u64,
    capture_method: String,
}

fn main() {
    let transaction = Transaction {
        id: Uuid::new_v4() ,
        user_id: 1,
        amount: 100,
        capture_method: String::from("emv"),
    };

    let mut transaction_2 = Transaction {
        id: Uuid::new_v4(),
        user_id: 1,
        amount: 110,
        capture_method: String::from("emv"),
    };

    transaction_2.amount = 200;

    let transaction_3 = build_transaction(122, 3, "contactless");
}


fn build_transaction(amount: u64, user_id: u64, capture_method: &str) -> Transaction {
    Transaction {
        id: Uuid::new_v4(),
        user_id: user_id,
        amount: amount,
        capture_method: String::from(capture_method),
    }
}