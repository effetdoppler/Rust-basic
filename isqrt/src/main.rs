fn main()
{
    let mut n: u64 = 0;
    while n < 201
    {
        println!("isqrt({}) = {}", n, isqrt(n));
        n = n + 8;
    }
}

fn isqrt(n: u64) -> u64
{
    let mut r:u64 = n;
	while r*r > n
	{
		r = r + n/r;
		r = r/2;
	}
	r
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_isqrt()
    {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(8), 2);
        assert_eq!(isqrt(72), 8);
        assert_eq!(isqrt(136), 11);
        assert_eq!(isqrt(200), 14);
    }
}