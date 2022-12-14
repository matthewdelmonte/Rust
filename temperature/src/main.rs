fn main() {
    fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
        (fahrenheit - 32.0) * ( 5.0 / 9.0 )
    }

    let fahrenheit = 100.0;
    let celsius = format!("{:.2}", fahrenheit_to_celcius(fahrenheit));
    
    println!("{} degrees Fahrenheit is {} degrees Celsius", fahrenheit, celsius);
}
