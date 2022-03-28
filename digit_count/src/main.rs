fn main()
{
	println!("digit_count(0) = {}", digit_count(0));
    let mut n: u64 = 1;
    while n < 9223372036854775808
    {
        println!("digit_count({}) = {}", n, digit_count(n));
        n = n * 2;
    }
    println!("digit_count({}) = {}", n, digit_count(n));
}

fn digit_count(mut n: u64) -> u8
{
    let mut result:u8 = 1;
	while n > 9
	{
		result = result + 1;
		n = n/10;
	}
	result
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_digit_count()
    {
        assert_eq!(digit_count(0), 1);
        assert_eq!(digit_count(8), 1);
        assert_eq!(digit_count(16), 2);
        assert_eq!(digit_count(32768), 5);
        assert_eq!(digit_count(9223372036854775808), 19);
    }
}