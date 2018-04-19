use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

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
    value: HashMap<U,V>,
}

impl<T,U,V> Cacher<T,U,V>
   where T: Fn(U) -> V,
         U: Eq + Hash,
         V: Copy
{
    fn new(calculation: T) -> Cacher<T,U,V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        match self.value.get(&arg) {
            Some(x) => x,
            None => {
            let v = (self.calculation)(arg);
            self.value.entry(arg).or_insert_with(|| v);
            v
            },
        }       
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num : u32| -> u32 {
    println!("Calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    let newNum = num.clone();
    newNum
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
