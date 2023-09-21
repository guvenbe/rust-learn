enum  Light {
    Bright,
    Dull,
}

fn display_light(light: &Light) {
    match light {
        Light::Bright => println!("it's bright"),
        Light::Dull => println!("it's dull"),
    }
}

fn main() {
    let light = Light::Bright;
    display_light(&light);  // it is borrowed by passing reference
    display_light(&light);  // Without & this would not possible
}