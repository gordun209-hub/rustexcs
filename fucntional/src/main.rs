use std::thread;
use std::time::Duration;

fn main() {
    // closures can capture values from the enclosing scope
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // can infer return type bec idk wtf ??? g
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    if intensity < 25 {
        println!("Today, do {} pushup", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today");
    } else {
        println!(
            "Today run for {} minutes",
            expensive_result.value(intensity)
        );
    }
}
// holy f
struct Cacher<T>
where
    // that holds closure
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // first time we put closure func to struct initial value None
    // and calculation is the function we put (closure func)
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }
// with that we get the value from struct, if some value "Some(v)" 
    // return value, if no value, calculate value with that "let v = (self.calculation)(arg)"
    // and put value inside struct so we can use it again,
    // finally return the calculated value, after that we dont have to calculate again because 
    // we are storing it in struct that s some beautiful shit right here
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            // thats not that good bec we cant change this value once we store
            // thus we never come this block of code again we got value once.
            // TODO try  to fix this with that we check hashmap first if the valie exists
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
