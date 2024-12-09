mod exemple1;
mod exemple2;
mod exemple3;
mod exemple4;
mod exemple5;

enum SorteAdresseIp {
    V4,
    V6,
}

fn main() {
    let quatre = SorteAdresseIp::V4;
    let six = SorteAdresseIp::V6;

    router(SorteAdresseIp::V4);
    router(SorteAdresseIp::V6);
}

fn router(sorte_ip: SorteAdresseIp) { }

