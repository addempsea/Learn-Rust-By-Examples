#[derive(Debug)]
struct User {
    name: String,
    age: u8,
    is_admin: bool,
    email: String
}

enum Option<T> {
    Some(T),
    None,
}

enum IpAddr {
    V4(String),
    V6(String),
}

struct IpAddrt {
    kind: IpAddr,
    address: String,
}

impl User {
    fn user_details(&self) -> u8{
        self.age + 10
    }
}

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

fn main() {
    let user1 = User {
        name: String::from("Ademola"),
        age: 12,
        is_admin: false,
        email: String::from("demola@enyata.com")
    };
    let demo: i32 = 55;
    let _home = IpAddr::V4(String::from("demo")) ;
    
   let _age_in_ten = user1.user_details();

    // let some_five: Option<i8> = Some(5);
    // let non: Option<i32> = None;
    let das = Coin::Dime;
    let vad: u8 = value_in_cents(das);
    println!("{}", vad)
}


