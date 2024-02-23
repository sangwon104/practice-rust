fn main() {
    enum_basic();
}

enum IpAddr {
    V4(String),
    V6(String)
}

fn enumerations() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
}
