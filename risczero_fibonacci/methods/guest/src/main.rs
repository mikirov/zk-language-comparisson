// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]

use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let iterations: u32 = env::read();
    let answer_1 = fibonacci_1(iterations);
    env::commit(&answer_1);
}

/// Implementation #1 of the nth Fibonacci number calculation.
///
/// This implementation a straight-forward, closely following the standard description of the
/// algorithm. It provides the baseline for comparison with any optimizations.
///
/// NOTE: Marked with #[inline(never)] to make sure this function is easy to see in the profile.
#[inline(never)]
#[no_mangle]
pub fn fibonacci_1(n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);
    if n <= 1 {
        return n as u64;
    }
    let mut i = 2;
    while i <= n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}
