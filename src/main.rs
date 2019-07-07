fn main() { 
    let list:Vec<usize> = vec![12,812,1,0,22,6132,13];
    let mean = average(&list);
    println!("The mean is: {}",mean);
}

fn average(list:&Vec<usize>) -> usize {
    sum(&list) / list.len()
}

fn sum(list:&Vec<usize>) -> usize {
    let mut result:usize = 0;
    for number in list {
        result = result + number;
    }
    result
}