fn main() -> pub Field{
    let mut a: Field = 0;
    let mut b: Field = 1;

    // We need for loop range to be known at compile time
    for _ in 1..10000 {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}