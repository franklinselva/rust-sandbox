/**
 * @Author: Franklin Selva
 * @Date:   2021-09-03 18:09:04
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-04 10:35:57
 */
pub fn run() {
    let person: (&str, &str, i8) = ("Franklin Selva", "France", 24);
    println!(
        "{} is from {} and he is {} years old",
        person.0, person.1, person.2
    );
}
