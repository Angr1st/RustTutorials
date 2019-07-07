fn main() { 
    let list:Vec<usize> = vec![3,5,7,12,13,14,21,23,23,23,23,29,40,56];
    print_average(&list);
    print_median(&list);
    print_mode(&list);
}

fn print_mode(list:&Vec<usize>) {
    let mode = mode(&list);
    println!("The mode is: {}", mode);
}

use std::collections::HashMap;

fn mode(list:&Vec<usize>) -> usize {
    let mut map = HashMap::new();

    for number in list {
        let count = map.entry(number).or_insert(1);
        *count += 1
    }

    let mut max_val:usize = 0;
    let mut max_key:usize = 0;
    for (key,val) in map {
        if val > max_val {
            max_val = val;
            max_key = *key;
        }
    }

    max_key
}

fn print_median(list:&Vec<usize>) {
    let mut list = list.clone();
    list.sort();

    let median = median(&list);
    println!("The median is: {}", median);
}

fn median(list:&Vec<usize>) -> usize {
    let length = list.len();
    
    if length == 0 {
        return 0
    }

    let even = length % 2 == 0;
    let middle = length / 2;

    if even {
        let first = list[middle - 1];
        let second = list[middle];
        let list:Vec<usize> = vec![first,second];
        average(&list)
    }
    else
    {
        list[middle]
    }  
}

fn print_average(list:&Vec<usize>) {
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