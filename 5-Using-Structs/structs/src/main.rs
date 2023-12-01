use uuid::Uuid;

struct Transaction {
    id: Uuid,
    user_id: u64,
    amount: u64,
    capture_method: String,
}
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;


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

    let transaction_3 = build_transaction(122, 3, String::from("contactless"));

    let transaction_pix = Transaction {
        capture_method: String::from("pix"),
        ..transaction_3
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

}


fn build_transaction(amount: u64, user_id: u64, capture_method: String) -> Transaction {
    Transaction {
        id: Uuid::new_v4(),
        user_id: user_id,
        amount: amount,
        capture_method,
    }
}

fn build_transaction_shorthand(amount: u64, user_id: u64, capture_method: String) -> Transaction {
    Transaction {
        id: Uuid::new_v4(),
        user_id,
        amount,
        capture_method,
    }
}