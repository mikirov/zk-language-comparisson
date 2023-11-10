#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);


fn fibonacci_184() -> u128 {
    let mut a: u128 = 0;
    let mut b: u128 = 1;

    for _ in 1..184 { // We need for loop range to be known at compile time
        let temp = a + b;
        a = b;
        b = temp;
    }

    b
}

pub fn main() {
    // TODO: Implement your guest code here

    // read the input
    let input: String = env::read();

    let output: String = fibonacci_184().to_string();

    if output != input {
        panic!("not correct 184th fibonacci element");
    }

    // write public output to the journal
    env::commit(&output);
}
