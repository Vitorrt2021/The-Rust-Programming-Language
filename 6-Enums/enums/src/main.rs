

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4;
    fn route(ip_kind: IpAddrKind) {}

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    enum IpAddrV2 {
        V4(String),
        V6(String),
    }

    let home_v2 = IpAddrV2::V4(String::from("127.0.0.1"));

    enum IpAddrV3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home_v3 = IpAddrV3::V4(127, 0, 0, 1);


    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
