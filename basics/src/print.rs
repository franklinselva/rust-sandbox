/**
 * @Author: Franklin Selva
 * @Date:   2021-09-03 17:15:46
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-03 17:52:04
 */

pub fn run()
{
    // Print
    print!("{}", "Hello");
    print!("{}", 3);
    print!("\n{}", 3+5);
    
    // Printlin
    println!("{content}", content = "Hello World!");
    println!("{0} is in {1} and {2} live in {0}", "France", "Europe", "I");
    println!("Number: {} + {} = {}",5, 6, 5+6);
    println!("Binary: {:b} Hex: {:x} Octo: {:o}", 5+6, 10, 10);
    println!("{:?}", (1, 23, "hello"));
}