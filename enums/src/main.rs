
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}
fn main() {
    let home = IpAddrKind::V4(String::from("1278...."));
}
