use std::thread;
use std::time::Duration;
use std::collections::HashMap;

fn main() {
    let simulated_user_specified_value = 26;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}

struct Cacher<T,U,V>
    where T: Fn(U) -> V
{
    calculation: T,
    value: Option<HashMap<U,V>>,
}

impl<T,U,V> Cacher<T,U,V>
   where T: Fn(U) -> V
{
    fn new(calculation: T) -> Cacher<T,U,V> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
    });
    if intensity < 25 {
        println!("Today, do {} pushups!",
        expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
           expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!",
            expensive_result.value(intensity)
            );
        }
    }
}
