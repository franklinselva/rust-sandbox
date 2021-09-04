/**
 * @Author: Franklin Selva
 * @Date:   2021-09-04 10:36:06
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-04 10:53:51
 */
use std::mem::size_of_val;

pub fn run() {
    // VECTORS are fixed list with same data type elements
    let arr: Vec<i32> = vec![1, 2, 3, 4, 5];
    let mut num: Vec<i8> = vec![0, 0, 0, 0, 0];

    //Vector assignments
    num[0] = 1;
    num.push(6);
    num.pop();

    println!("VECTORS.rs - Array {:?}", arr);
    println!(
        "VECTORS.rs - Single value: {} \nArray Length: {}",
        num[0],
        num.len()
    );
    println!("Size of vectors: {} bytes", size_of_val(&num));

    // Slicing
    let sliced: &[i8] = &num[0..3];
    println!("Sliced Vectors: {:?}", sliced);

    //Loops
    println!("Looping through vectors: ");
    for i in num.iter() {
        print!("{}", i);
    }

    println!("\nLooping through mutated vectors: ");
    for i in num.iter_mut() {
        print!("{}", *i + 4);
    }
}
