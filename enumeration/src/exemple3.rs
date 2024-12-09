fn main() {
    enum AdresseIp {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let local = AdresseIp::V4(127, 0, 0, 1);

    let rebouclage = AdresseIp::V6(String::from("::1"));
}
