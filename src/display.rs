use owo_colors::OwoColorize;

// HELPER FUNCTIONS

pub fn print_i32_value(label: &str, value: i32) {
    println!("  {label}: {value}");
}

pub fn print_price(label: &str, value: f64) {
    println!("  {label}: ${value:.2}");
}

pub fn print_percent(label: &str, value: f64) {
    println!("  {label}: {value:.2}%");
}
