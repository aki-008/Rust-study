// 🏋️ Exercise: Temperature Conversion Library (click to expand)
// Challenge: Build a mini temperature conversion library:

// Define Celsius(f64), Fahrenheit(f64), and Kelvin(f64) structs
// Implement From<Celsius> for Fahrenheit and From<Celsius> for Kelvin
// Implement TryFrom<f64> for Kelvin that rejects values below absolute zero (-273.15°C = 0K)
// Implement Display for all three types (e.g., "100.00°C")

use std::fmt;

// --- Tuple structs wrapping a raw f64 value for each temperature scale ---
struct Celsius(f64);
struct Fahrenheit(f64);
struct Kelvin(f64);

// --- Conversion: Celsius -> Fahrenheit via the standard `From` trait ---
// Implementing `From` here automatically gives us `Into` for free,
// which is what lets `Celsius(100.0).into()` work later in main().
impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        // c.0 accesses the wrapped f64 inside the Celsius tuple struct
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

// --- Conversion: Celsius -> Kelvin via the standard `From` trait ---
impl From<Celsius> for Kelvin {
    fn from(c: Celsius) -> Self {
        Kelvin(c.0 + 273.15) // add the Celsius-to-Kelvin offset
    }
}

// --- Custom error type used when a Kelvin value would be physically impossible ---
#[derive(Debug)] // enables {:?} formatting for this error type
struct BelowAbsoluteZero;

// --- Human-readable formatting for the error, needed for {e} in println! ---
impl fmt::Display for BelowAbsoluteZero {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "temperature below absolute zero")
    }
}

// --- Fallible conversion: raw f64 -> Kelvin, since not every f64 is valid ---
// TryFrom is used (instead of From) because this conversion can fail.
impl TryFrom<f64> for Kelvin {
    type Error = BelowAbsoluteZero; // associate our custom error type

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        // Flow: check validity first, branch into Err or Ok
        if value < 0.0 {
            Err(BelowAbsoluteZero) // negative Kelvin is physically invalid
        } else {
            Ok(Kelvin(value)) // valid value, wrap and return
        }
    }
}

// --- Display impls: control how each type is printed with {} / {var} ---
impl fmt::Display for Celsius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}°C", self.0) // 2 decimal places + unit suffix
    }
}
impl fmt::Display for Fahrenheit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}°F", self.0)
    }
}
impl fmt::Display for Kelvin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}K", self.0)
    }
}

fn main() {
    // Step 1: create a Celsius value (kept for later printing)
    let boiling = Celsius(100.0);

    // Step 2: convert a fresh Celsius(100.0) into Fahrenheit
    // `.into()` works because `From<Celsius> for Fahrenheit` is implemented above;
    // the target type is inferred from the `f: Fahrenheit` annotation.
    let f: Fahrenheit = Celsius(100.0).into();

    // Step 3: convert another fresh Celsius(100.0) into Kelvin, same mechanism
    let k: Kelvin = Celsius(100.0).into();

    // Step 4: print all three, each using its own Display impl
    println!("{boiling} = {f} = {k}");

    // Step 5: attempt a fallible conversion from a raw f64 to Kelvin
    // -10.0 is below absolute zero, so this will take the Err branch
    match Kelvin::try_from(-10.0) {
        Ok(k) => println!("{k}"),         // not reached for this input
        Err(e) => println!("Error: {e}"), // this branch executes
    }
}
