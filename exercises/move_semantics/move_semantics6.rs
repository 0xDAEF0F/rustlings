// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "rust is great!".to_string();

    let cha = get_char(&data);

    string_uppercase(data);

    println!("char is: {cha}");
}

// Should not take ownership
fn get_char(data: &str) -> char {
    data.chars().next().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
