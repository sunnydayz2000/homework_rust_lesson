mod mod_outter;
mod mod_inner;

use mod_outter::print_char_from_a_to_b;
use mod_inner::fun::inner_print_char_from_a_to_b;
fn main() {
    //println!("Hello, world!");

    let string_from_a_to_Z = print_char_from_a_to_b('a', 'Z');
    println!("{}", string_from_a_to_Z);

    let string_from_A_to_z = inner_print_char_from_a_to_b('A', 'z');
    println!("{}", string_from_A_to_z);
}
