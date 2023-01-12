pub fn find_min(nums: Vec<i32>) -> i32
{
    println!("vec: {:?}", nums);
    let mut l: usize = 0;
    let mut r: usize = nums.len() - 1;

    while l < r
    {
        let m = l + ((r - l) / 2);

        if nums[l] < nums[r]
        {
            return nums[l];
        }

        if l == r
        {
            return nums[l];
        }

        if m > l && nums[m - 1] > nums[m]
        {
            return nums[m];
        }
        else if m < r && nums[m] > nums[m + 1]
        {
            return nums[m + 1];
        }
        else
        {
            if nums[m] < nums[r]
            {
                r = m - 1;
            }
            else
            {
                l = m + 1;
            }
        }
    }

    return nums[l];
}

#[test]
pub fn test()
{
    assert_eq!(find_min(vec![1, 2]), 1);
    assert_eq!(find_min(vec![2, 1]), 1);
    assert_eq!(find_min(vec![3, 4, 5, 1, 2]), 1);
    assert_eq!(find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
    assert_eq!(find_min(vec![11, 13, 15, 17]), 11);
}
