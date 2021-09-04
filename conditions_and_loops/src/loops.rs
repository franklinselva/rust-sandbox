/**
 * @Author: Franklin Selva
 * @Date:   2021-09-04 11:14:48
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-04 11:22:27
 */
pub fn run() {
    let mut count = 0;

    // Unconditional loops
    loop {
        count += 1;
        print!("{}", count);
        if count >= 10 {
            break;
        }
    }
    println!("While Loop - Fizzbuzz Challenge");

    // While loop
    while count <= 100 {
        if count % 15 == 0 {
            println!("fizzbuzz");
        } else if count % 3 == 0 {
            println!("fizz");
        } else if count % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", count);
        }

        count += 1
    }

    println!("For Loop - Fizzbuzz Challenge");

    // For loop
    for x in 0..100 {
        if x % 15 == 0 {
            println!("fizzbuzz");
        } else if x % 3 == 0 {
            println!("fizz");
        } else if x % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", x);
        }

        count += 1
    }
}
