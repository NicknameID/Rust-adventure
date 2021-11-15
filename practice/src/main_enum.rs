enum IpAddrKind {
    v4,
    v6,
}

enum IpAddrEnum {
    v4(String),
    v6(String),
}

enum IpAddrValueEnum {
    v4(u8, u8, u8, u8),
    v6(String),
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddrEnum {
    fn call(&self) {
    
    }
}

// fn route(ip_type: IpAddrKind) {
//     println!("IpAddrKing: {}", ip_type)
// }

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;
    
    let home = IpAddr { kind: IpAddrKind::v4, address: String::from("127.0.0.1") };
    let loopback = IpAddr { kind: IpAddrKind::v6, address: String::from("::1") };
    
    let home_enum = IpAddrEnum::v4(String::from("127.0.0.1"));
    let loopback_enum = IpAddrEnum::v6(String::from("::1"));

    let home_value_enum = IpAddrValueEnum::v4(127, 0, 0, 1);
    let loopback_value_enum = IpAddrValueEnum::v6(String::from("::1"));
}