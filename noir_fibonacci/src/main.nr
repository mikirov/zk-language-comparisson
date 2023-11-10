fn fibonacci_184() -> Field {
    let mut a: Field = 0;
    let mut b: Field = 1;

    // We need for loop range to be known at compile time
    for _ in 1..184 {
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}


fn main(result: pub Field) {
    let expected = fibonacci_184();
    assert (result == expected);   
}