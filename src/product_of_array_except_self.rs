pub fn product_except_self(nums: Vec<i32>) -> Vec<i32>
{
    let mut product_array: Vec<i32> = Vec::with_capacity(nums.len());
    for _ in 0..nums.len()
    {
        product_array.push(1);
    }

    let mut coefficient: i32 = 1;
    for i in 0..nums.len()
    {
        product_array[i] *= coefficient;
        coefficient *= nums[i];
    }

    coefficient = 1;
    for i in (0..nums.len()).rev()
    {
        product_array[i] *= coefficient;
        coefficient *= nums[i];
    }

    return product_array;
}

#[test]
pub fn test()
{
    assert_eq!(product_except_self(vec![1, 2, 3, 4]), vec![24, 12, 8, 6]);
    assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
}
