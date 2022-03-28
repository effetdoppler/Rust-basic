pub fn divisor_sum(n: u64) -> u64
{
    let mut result: u64 = 0;
    if n > 1
    {
        let mut i: u64 = 2;
        let mut l;
        result = 1;
        while (i * i) <= n
        {
            l = n/i;
            if (l * i) == n
            {
                if l == i
                {
                    result = result + l
                }
                else
                {
                    result = result + l + i;
                }
            }
            i = i + 1;
        }
    }
    result
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_divisor_sum()
    {
        assert_eq!(divisor_sum(1), 0);
        assert_eq!(divisor_sum(3), 1);
        assert_eq!(divisor_sum(9), 4);
        assert_eq!(divisor_sum(28), 28);
        assert_eq!(divisor_sum(30), 42);
        assert_eq!(divisor_sum(137438691328), 137438691328);
    }
}