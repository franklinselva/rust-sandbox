/**
 * @Author: Franklin Selva
 * @Date:   2021-09-04 10:36:06
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-04 10:46:00
 */
use std::mem::size_of_val;

pub fn run() {
    // Arrays are fixed list with same data type elements
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let mut num: [i8; 5] = [0, 0, 0, 0, 0];

    num[0] = 1;

    println!("ARRAYS.rs - Array {:?}", arr);
    println!(
        "ARRAYS.rs - Single value: {} \nArray Length: {}",
        num[0],
        num.len()
    );
    println!("Size of arrays: {} bytes", size_of_val(&num));

    // Slicing
    let sliced: &[i8] = &num[0..3];
    println!("Sliced Array: {:?}", sliced);
}
