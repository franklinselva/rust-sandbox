/**
 * @Author: Franklin Selva
 * @Date:   2021-09-03 17:37:03
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-03 17:50:17
 */

pub fn run() {
    // Default is i32, f64
    let a: i32 = 789;
    let b: f64 = 8.99;
    let c: char = 'c';
    let d: bool = true;
    let e = "Hello";
    let f = (1, 2, 3);

    println!("TYPES.rs - {:?}", (a, b, c, d, e, f));
    println!("TYPES.rs - MAX: \n i32: {} \n f64: {}", std::i32::MAX, std::f64::MAX);
}