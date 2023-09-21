struct Temperature {
    degrees: f64,
}

impl Temperature {
    fn new(degrees: f64) -> Self {
        Self { degrees }
    }

    fn as_fahrenheit(&self) -> f64 {
        self.degrees * 1.8 + 32.0
    }

    fn as_celsius(&self) -> f64 {
        self.degrees
    }

    fn freezing_celsius() -> Self {
        Self::new(0.0)
    }

    fn freezing_fahrenheit() -> Self {
        Self::new(32.0)
    }

    fn boiling_celsius() -> Self {
        Self::new(100.0)
    }

    fn boiling_fahrenheit() -> Self {
        Self::new(212.0)
    }

    fn show_temp(&self) {
        println!("Temperature: {:.2}°C", self.as_celsius());
        println!("Temperature: {:.2}°F", self.as_fahrenheit());
    }
}
fn main() {
    let temp = Temperature::new(37.0);
    temp.show_temp();
    Temperature::new(100.0).show_temp();

    let cold_celcius = Temperature::freezing_celsius();
    let cold_fahrenheit = Temperature::freezing_fahrenheit();

    println!("Freezing: {}°C is {}°F", cold_celcius.as_celsius(), cold_fahrenheit.as_fahrenheit());

    let hot_celcius = Temperature::boiling_celsius();
    let hot_fahrenheit = Temperature::boiling_fahrenheit();

    println!("Boiling: {}°C is {}°F", hot_celcius.as_celsius(), hot_fahrenheit.as_fahrenheit());
}

