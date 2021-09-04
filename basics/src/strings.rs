/**
 * @Author: Franklin Selva
 * @Date:   2021-09-03 17:50:35
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-03 18:07:44
 */
pub fn run(){
    let mut hello = String::from("Hello");
    let mut msg = "Message"; //String functions are not possible here

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push_str("0123456789");

    hello.push_str(" is Extended");

    println!("[STRINGS.rs] {} is of length {} and of capacity {}", hello, hello.len(), hello.capacity());
    println!("[STRINGS.rs] {} is of length {}", msg, msg.len());

    // Check if empty
    println!("[STRINGS.rs] Is Empty: {}", hello.is_empty());

    //Check contains a char or a string
    println!("[STRINGS.rs] Contains (o?): {}", hello.contains('o'));

    //Replace
    println!("[STRINGS.rs] REPLACE: {}", hello.replace("is Extended", "World"));

    //Loop in string
    print!("[STRINGS.rs] Looping string: \n");
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

}
