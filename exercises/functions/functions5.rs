// functions5.rs
// Make me compile! Execute `rustlings hint functions5` for hints :)

// Notice no `return` keyword if there is no semi-colon as well

fn main() {
    let answer = square(3);
    println!("The answer is {}", answer);
}

fn square(num: i32) -> i32 {
    num * num;
}
