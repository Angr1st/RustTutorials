fn main() {
    let celsius = Celsius { temperature: 30 };
    println!("The Temperature is {} degree celsius", celsius.temperature);
    let fahrenheit = celsius.to_fahrenheit();
    println!("The Temperature is {} degree fahrenheit", fahrenheit.temperature);
    let celsius = fahrenheit.to_celsius();
    let enum_temp = Temperature::Celsius(celsius);
    let enum_temp = transform_temperature(enum_temp);
    println!("The Tempreture is {}!", enum_temp.temp_display())
}

fn transform_temperature(temp:Temperature) -> Temperature {
    match temp {
        Temperature::Fahrenheit(inner) => Temperature::Celsius(Celsius::from(inner)),
        Temperature::Celsius(inner) => Temperature::Fahrenheit(Fahrenheit::from(inner))
    }
} 

struct Celsius {
    temperature:i32
}

struct Fahrenheit {
    temperature:i32
}

enum Temperature {
    Celsius(Celsius),
    Fahrenheit(Fahrenheit)
}

impl Temperature {
    fn temp_display(self) -> i32 {
        match self {
            Temperature::Celsius(inner) => inner.temperature,
            Temperature::Fahrenheit(inner) => inner.temperature
        }
    }
}

impl Celsius {
    fn to_fahrenheit(self) -> Fahrenheit {
        from_celsius_to_fahrenheit(self)
    }

    fn from(fahrenheit:Fahrenheit) -> Celsius {
        from_fahrenheit_to_celsius(fahrenheit)
    }
}

impl Fahrenheit {
    fn to_celsius(self) -> Celsius {
        from_fahrenheit_to_celsius(self)
    }

    fn from(celsius:Celsius) -> Fahrenheit {
        from_celsius_to_fahrenheit(celsius)
    }
}

fn from_celsius_to_fahrenheit(celius:Celsius) -> Fahrenheit{
            Fahrenheit{ 
        temperature:(celius.temperature * 9/5) + 32
        } 
}

fn from_fahrenheit_to_celsius(fahrenheit:Fahrenheit) -> Celsius {
            Celsius{
            temperature:(fahrenheit.temperature - 32) * 5/9 
        }
}