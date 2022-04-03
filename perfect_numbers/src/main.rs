use divisor_sum::divisor_sum;

fn main()
{
    let mut i:u64 = 1;
	while i < 100000
	{
		if is_perfect_number(i) == true
		{
			println!("{}", i);
		}
		i = i + 1;
	}
}

fn is_perfect_number(n: u64) -> bool
{
    if divisor_sum(n) == n
	{
		true
	}
	else
	{
		false
	}
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_is_perfect_number()
    {
        assert_eq!(is_perfect_number(6), true);
        assert_eq!(is_perfect_number(12), false);
        assert_eq!(is_perfect_number(28), true);
        assert_eq!(is_perfect_number(165), false);
        assert_eq!(is_perfect_number(496), true);
    }
}