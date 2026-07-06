use ch00_setup::{greeting, setup_status};

fn main() {
    let name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Rust".to_string());

    println!("{}", greeting(&name));
    println!("{}", setup_status());
}
