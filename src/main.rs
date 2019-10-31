mod collatz;
mod quicksort;

fn main()
{
    //test_collatz();
    test_quicksort();
}

#[allow(dead_code)]
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

#[allow(dead_code)]
fn test_quicksort()
{
    let mut x:Vec<u32> = vec![3,4,71,4,6,1,3,5,7,5,45,9,7,65,100,67,74,24,6];

    quicksort::quicksort(&mut x);
    println!("{:?}", x);

}
