/**
 * @Author: Franklin Selva
 * @Date:   2021-09-04 11:09:13
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-04 11:14:41
 */
pub fn run() {
    let age = 18;
    let check_id: bool = true;

    // If elseif else
    if age >= 21 && check_id {
        println!("You are good to go! Have fun");
    } else if age <= 21 && check_id {
        println!("I am sorry. This section is not for you")
    } else {
        println!("Could you show me your ID?")
    }

    // Shorthand if
    let is_permitted: bool = if age >= 21 { true } else { false };
    println!(
        "Are you permitted: {}",
        if is_permitted { "Yes" } else { "No" }
    );
}
