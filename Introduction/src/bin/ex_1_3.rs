fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn classify_temp(temp_f: f64) -> &'static str {
    if temp_f < 50.0 {
        "cold"
    } else if temp_f < 77.0 {
        "mild"
    } else {
        "hot"
    }
}

fn main() {
    for c in [0.0, 20.0, 35.0] {
        let f = celsius_to_fahrenheit(c);
        println!("{c:.1}.C = {f:.1}.F — {}", classify_temp(f));
    }
}
