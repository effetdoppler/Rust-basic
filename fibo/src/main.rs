fn main()
{
    let mut n: u64 = 0;
    while n < 11
    {
        println!("fibo({}) = {}", n, fibo(n));
        n = n + 1;
    }
    println!("...");
    n = 81;
    while n < 91
    {
        println!("fibo({}) = {}", n, fibo(n));
        n = n + 1;
    }
}

fn fibo(n: u64) -> u64
{
    if n < 2
	{
		n
	}
	else
	{
		let mut a = 0;
		let mut b = 1;
		let mut result:u64 = 1;
		let mut i = 2;
		while i < n+1
		{
			result = a + b;
			a = b;
			b = result;
			i = i + 1;
		}
		result
	}
	
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_fibo()
    {
        assert_eq!(fibo(0), 0);
        assert_eq!(fibo(1), 1);
        assert_eq!(fibo(2), 1);
        assert_eq!(fibo(3), 2);
        assert_eq!(fibo(10), 55);
		assert_eq!(fibo(81), 37889062373143906);
    }
}