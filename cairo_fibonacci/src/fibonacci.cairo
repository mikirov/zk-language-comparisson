use debug::PrintTrait;

fn fibonacci() -> felt252 {
    let mut a: felt252 = 0;
    let mut b: felt252 = 1;

    // We need for loop range to be known at compile time

    let mut i: usize = 1;
    loop {
        i += 1;
        if i > 1000 {
            break;
        }
        let temp = a + b;
        a = b;
        b = temp;

    };

    b
}

fn main() {
    let fib = fibonacci();
    fib.print();
}
