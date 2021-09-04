/**
 * @Author: Franklin Selva
 * @Date:   2021-09-03 17:31:08
 * @Last Modified by:   Franklin Selva
 * @Last Modified time: 2021-09-03 17:43:17
 */

pub fn run()
{
    // Immutable variablese
    let name = "Franklin Selva";
    let age = "24";
    
    // Mutable variables
    let mut mut_age = 25;
    
    // Constant variables
    const ID:i8 = 124;

    // chars
    let character: char = 'a';
    
    mut_age = mut_age + 2;
    println!("VARS.rs - {:?}", (name, age, mut_age, ID, character));
}