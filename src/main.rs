mod collatz;
mod quicksort;
mod polynoms;

use polynoms::Polynomial;


fn main()
{
    //test_collatz();
    // test_quicksort();
    test_polynoms();
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

#[allow(dead_code)]
fn test_polynoms()
{
    let x = Polynomial::new().add(1,3).add(-2,2).add(-11,1).add(12,0);
    x.print();
    println!("{}", x.newton(-4.));
    println!("{}", x.newton(0.));
    println!("{}", x.newton(2.35287527));

    let x = Polynomial::new().add(1,3).add(-2,2).add(-5,1).add(6,0);
    x.print();
    println!("{}", x.newton(-3.));
    println!("{}", x.newton(0.));
    println!("{}", x.newton(4.));

    let x = Polynomial::new().add(2,4).add(7,3).add(6,2).add(8,1).add(12,0);
    x.print();
    println!("{}", x.newton(0.));
    println!("{}", x.newton(10.));
}
