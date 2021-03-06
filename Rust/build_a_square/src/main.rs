/*
---Build a Square---
I will give you an integer. Give me back a shape that is as long and wide as the integer. The
integer will be a whole number between 1 and 50.

Example
n = 3, so I expect a 3x3 square back just like below as a string:

+++
+++
+++
 */

fn main() {
    println!("{}", generate_shape(3));
}

fn generate_shape(n:i32) -> String {
    let mut square = String::new();

    for r in 0..n {
        for _ in 0..n {
            square.push('+');
        }
        if r < n - 1 {
            square.push('\n');
        }
    }

    return square;
}
