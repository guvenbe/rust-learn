struct Temperature {
    degrees: f64,
}

impl Temperature {
    fn show_temperature(&self) {
        println!("{:?} degrees F", self.degrees);
    }
    fn freezing() -> Self {
        Self { degrees: 32.0 }
    }
    fn boiling() -> Self {
        Self { degrees: 212.0 }
    }
}

fn main() {
    let hot = Temperature { degrees: 99.9 };
    hot.show_temperature();
    let cold = Temperature::freezing();
    cold.show_temperature();
    let boiling = Temperature::boiling();
    boiling.show_temperature();
}
