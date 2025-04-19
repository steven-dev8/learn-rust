// Tipo de implementação 1
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Tipo de implementação 2: bem melhor
enum IpAddrKindBest {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // Implementação 1
    let ip1 = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("1.0.0.127")
    };

    // Implementação 2
    let ip2 = IpAddrKindBest::V4(1, 0, 0, 127);
    let ip3 = IpAddrKindBest::V6(String::from("::1"));
}
