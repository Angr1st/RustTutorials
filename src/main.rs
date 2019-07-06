fn main() {
    for number in 0..12 {
        println!("{}",build_first_block(number))
    }
}

fn build_first_block(n:usize) -> String{
    let first = "On the ";
    let middle = get_times(n);
    let end = " day of Christmas,\nmy true love sent to me";
    format!("{}{}{}\n{}\n",first,middle,end,get_presents(n))
}

fn get_times(n:usize) -> &'static str{
    let numbers = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    numbers[n]
}

fn get_presents(n:usize) -> String{
    let presents = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];
    let mut result = presents[n].to_owned();
    if n == 0 {
        result
    }
    else {
        for number in (0..n).rev() {
            if number == 0 {
                result = format!("{},\nAnd {}.",result,presents[number]);
            }
            else {
                result = format!("{},\n{}",result,presents[number]);
            }           
        }
        result
    }
}