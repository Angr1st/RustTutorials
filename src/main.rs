fn main() {
    let x= 6;

    let y = {
 plus_five(10)
    };

    println!("The value of y is: {}", y);
}

fn plus_five(x: i32) -> i32 {
    x+5;
}