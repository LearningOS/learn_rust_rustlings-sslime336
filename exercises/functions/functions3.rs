// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)


fn main() {
    call_me(123 as u32); // perhaps 123 is also ok, compiler while do the type inference
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
