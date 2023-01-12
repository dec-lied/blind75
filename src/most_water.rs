pub fn max_area(height: Vec<i32>) -> i32
{
    let mut max_area: i32 = 0;

    for i in 0..height.len()
    {
        for j in (i + 1)..height.len()
        {
            max_area = std::cmp::max
            (
                max_area,
                (j - i) as i32 * std::cmp::min(height[i], height[j])
            );
        }
    }

    return max_area;
}

#[test]
pub fn test()
{
    assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
}
