pub fn contains_duplicate(nums: Vec<i32>) -> bool
{
    let mut seen: Vec<i32> = vec![nums[0]];

    for i in 1..nums.len()
    {
        if seen.contains(&nums[i])
        {
            return true;
        }
        else
        {
            seen.push(nums[i]);
        }
    }

    return false;
}

#[test]
pub fn test()
{
    assert_eq!(contains_duplicate(vec![1, 2, 3, 1]), true);
    assert_eq!(contains_duplicate(vec![1, 2, 3, 4]), false);
    assert_eq!(contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]), true);
}
