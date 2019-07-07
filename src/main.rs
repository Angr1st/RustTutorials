fn main() { 
    let list:Vec<usize> = vec![3,5,7,12,13,14,21,23,23,23,23,29,40,56];
    print_average(&list);
    print_median(&list);
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
    println!("The length is: {}; Is it even: {}; The middle is at: {}",length,even,middle);
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