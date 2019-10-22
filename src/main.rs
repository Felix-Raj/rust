fn main() {
    // types of IPAddrs
    enum IpAddKind {
        V4,  // CamelCase
        V6,
    };

    // create instances
    let four = IpAddKind::V4;
    let six = IpAddKind::V6;

    // can thus create some generic fun ( my terminology )
    fn route(ip_kind: IpAddKind) -> bool {
        // ip_kind can be V4 or V6
        true
    }
    route(four);

    // using as option
    struct IpAddr {
        kind: IpAddKind,
        address: String,
    }
    let ip1 = IpAddr{
        kind: IpAddKind::V4,
        address: "194.4.2.59".to_string(),
    };

    // more concise method would be, to have data inside enum variants
    enum IpAddr02 {
        V4(String),
        V6(String),
    }
    let ip2 = IpAddr02::V4(String::from("143.23.53.22"));

    // each variant can have different types and amount of associated data
    enum IpAddr03 {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let ip3 = IpAddr03::V4(198, 198, 34, 51);
    let ip4 = IpAddr03::V6("::1".to_string());

    // how does the standard library does this
    struct Ip4Addr {
        // some code
    };
    struct IP6Adr {
        // some code
    };
    enum IpAddr04 {
        V4(Ip4Addr),
        V6(IP6Adr),
    }

    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    };
    // is equivalent to creating separate structs for each of the variants
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32, y: i32
    }
    struct WriteMessage(String);  // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct
    // but then we cannot define a function that takes any of this message type

    // enums can also have methods
    impl Message {
        fn call(&self) {

        }
    }
    let m1 = Message::Write("Something".to_string());
    m1.call();

    // Rust does not as such have *Nulls*
    // the standard Option enum help to encode the concept of Null
    // Option as defined in standard library is
    /*enum Option<T> {
        Some(T),
        None,
    }*/
    // the variants of the Option is available w/o explicitly including it, also
    // the Option enum is also included
    let some_number = Some(5);
    let some_string = Some("a String");
    let some_null: Option<i32> = None;  // *should* specify the type also, if using None

    // if a value may become null, use Option<T>::None for it,
    // we cannot directly use this value, as Option is different from the type
    // for example
    let x: i8 = 5;
    let y: Option<i8> = None;
    // let sum = x+y // will fail
    // we have to convert Option<T> to T, we will require code that will handle each
    //    variant. Some code will run only when we have Some variant, some will run if
    //    we have None value
}