#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

mod stack_and_heap;
mod control_flow;
mod data_structures;

use std::mem;

const MY_GLOBAL_VAR: u8 = 5;
static mut MY_STATIC_VALUE: i32 = 555; // if made mutable, separate threads can be writing to this global var, deeming it 'unsafe'
fn operators() {
    // arithmetic
    println!("=========================");
    println!("========OPERATORS========");
    println!("=========================");
    let mut a = 2 + 3 * 4;
    println!("{}", a);
    a = a + 1;
    a += 1; // +=, -=, *=, /=
    println!("remainder of {} / {} = {}", a, 3, (a % 3));

    let a_cubed: i32 = i32::pow(a, 3);
    println!("a_cubed = {}", a_cubed);

    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi: f64 = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi);

    // bitwise
    let c = 1 | 2; // '|' = OR, '&' = AND, '^' = XOR, '!' = NOR
                        // 1 | 2 == 01 OR 10 == 11 == 3_10
    println!("1|2 = {}", c);
    let two_to_10 = 1 << 10;
    println!("2^10 = {}", two_to_10);

    // logical
    let pi_less_than_4 = std::f64::consts::PI < 4.0; // true
    println!("Is Pi less than 4?: {}", pi_less_than_4);
    let x = 5;
    let x_is_5 = x == 5;
    println!("{} is equal to 5: {}", x, x_is_5);


}

fn scope_and_shadowing() {
    println!("=========================");
    println!("====SCOPE & SHADOWING====");
    println!("=========================");
    let a = 123;
    println!("a = {}", a);

    { // can create a new scope with set of curly brackets
        let b = 456;
        println!("inside, b = {}", b);

        let a = 555;
        println!("{} is shadowing 'a'", a);
    }
    // println!("outside, b = {}", b); // throws an error, 'b' does not exist in this scope
}

fn constants() {
    println!("=========================");
    println!("========CONSTANTS========");
    println!("=========================");

    const MEANING_OF_LIFE: u8 = 42;
    println!("MEANING OF LIFE = {}", MEANING_OF_LIFE);

}

fn main() {
    println!("Hello, world!");
    // u = unsigned = 0 to 2^n-1
    // i = signed = -2^(n-1) .. 2^(n-1)-1

    // let == immutable
    // u8 == unsigned, 8 bits, 0-255
    let a: u8 = 123;

    // let mut == mutable
    // i8 = signed, -128 - 127
    let mut b: i8 = 0;
    println!("b = {}, before", b);
    b = 42;
    println!("b = {}, after", b);

    let mut c = 123456789; // i32 = 32 bits, 4 bytes
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));

    // u8, u16, u32, u64, i8, i16, i32, i64
    // usize, isize
    let x: isize = 123;
    let z: usize = 123;
    let size_of_x = mem::size_of_val(&x);
    let size_of_z = mem::size_of_val(&z);
    println!("Byte-size of x: {}", size_of_x);
    println!("Byte-size of z: {}", size_of_z);

    let d: char = 'x'; // char can represent '.', ';', letters, invisible chars, etc.
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // f32, f64 == floats, adhere to IEEE754 standard, always signed

    let e: f32 = 2.500;
    let f: f64 = 2.500;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));
    println!("{}, size = {} bytes", e, mem::size_of_val(&f));

    let g: bool = false;
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));

    println!("MY_GLOBAL_VAR: {}", MY_GLOBAL_VAR);

    unsafe { // mutable global vars are 'unsafe' and should be decalred as such
        println!("MY_STATIC_VALUE: {}", MY_STATIC_VALUE);
    }

    // operators();
    // scope_and_shadowing();
    // constants();
    // stack_and_heap::stack_and_heap();
    // control_flow::control_flow();
    data_structures::data_structures();
}
