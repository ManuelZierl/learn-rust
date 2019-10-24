// TASK 1: Collatz Conjecture

use std::cmp;

fn f(n:u32) -> u32
{
    if n%2 == 0 { return n/2 }
    return 3*n +1
}

pub fn collatz_it(mut n:u32) -> (u32,u32)
{
    let mut count = 0;
    let mut max = n;
    while n != 1
    {
        let temp = f(n);
        max = cmp::max(max, temp);
        n = temp;
        count += 1;
    }
    return (count, max)
}

pub fn collatz_re(n:u32, counter:u32) -> u32
{
    if n != 1 {collatz_re(f(n), counter+1)}
    else {counter}
}
