pub fn max_profit(prices: Vec<i32>) -> i32
{
    let mut l: usize = 0;
    let mut r: usize = 1;

    let mut max_profit: i32 = 0;

    while r <= prices.len() - 1
    {
        if prices[r] < prices[l]
        {
            l = r;
            r += 1;

            continue;
        }

        if prices[r] - prices[l] > max_profit
        {
            max_profit = prices[r] - prices[l];
        }

        r += 1;
    }

    return max_profit;
}

#[test]
pub fn test()
{
    assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
    assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
}
