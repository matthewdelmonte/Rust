use std::collections::HashMap;

// print the elements of a vector in ascending order
// fn main() {
//     let v = vec![100, 32, 57];
//     for i in &v {
//         println!("{}", i);
//     }
// }

// find the median of the vector
// fn main() {
//     let mut v = vec![12, 8, 10, 6, 4];

//     v.sort();

//     let middle = v.len() / 2;

//     let median = if v.len() % 2 == 0 {
//         (v[middle -1] + v[middle]) /2       
//     } else {
//         v[middle]
//     };

//     println!("The median is {}", median);
// }

// find the mode of the vector
fn main() {
    let vec = vec![1, 2, 2, 3, 4, 4, 5, 5, 5, 6, 7, 7];

    let mut frequency: HashMap<i32, i32> = HashMap::new();

    for element in vec {
        *frequency.entry(element).or_insert(0) += 1;
    }

    let mut mode = None;
    let mut max_frequency = 0;
    for (element, count) in frequency {
        if count > max_frequency {
            mode = Some(element);
            max_frequency = count;
        }
    }
    match mode {
        Some(element) => println!("The mode is {:?}", element),
        None =>  println!("The mode is no mode"),
    }
}

