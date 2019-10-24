mod collatz;

fn main()
{
    test_collatz()
}

fn test_collatz()
{
    for x in 1..=100
    {
        let c1 = collatz::collatz_it(x);
        println!("for n = {}, smallest i: {}, largest n: {}", x, c1.0, c1.1);
        let c2 = collatz::collatz_re(x, 0);
        println!("for n = {}, smallest i: {}", x, c2);
        assert!(c1.0 == c2);
    }
}
