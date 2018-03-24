fn main() {
    let numbers = [11,22,33,44,55,66,77];
    let mut index = 0;

    while index < 7 {
        println!("{}!", numbers[index] );

        index = index + 1;
    }
}