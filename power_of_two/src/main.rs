fn main()
{
    let mut n: u8 = 0;
    while n < 64
    {
        println!("power_of_two({}) = {}", n, power_of_two(n));
        n = n + 1;
    }
}

fn power_of_two(n: u8) -> u64
{
    1 << n
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_power_of_two()
    {
        assert_eq!(power_of_two(0), 1);
        assert_eq!(power_of_two(5), 32);
        assert_eq!(power_of_two(10), 1024);
        assert_eq!(power_of_two(20), 1048576);
        assert_eq!(power_of_two(63), 9223372036854775808);
    }
}