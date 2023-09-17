pub fn max_sub_array(nums: Vec<i32>) -> i32
{
    let mut curr_sum: i32 = if nums[0] > 0 { nums[0] } else { 0 };
    let mut max_sum: i32 = nums[0];

    println!("{:?}", nums);

    for num in nums.iter().skip(1)
    {
        println!("num: {}, curr_sum: {}, max_sum: {}", num, curr_sum, max_sum);

        curr_sum += num;

        if curr_sum > max_sum
        {
            max_sum = curr_sum
        }

        if curr_sum < 0
        {
            curr_sum = 0;
        }
    }

    return max_sum;
}

#[test]
pub fn test()
{
    assert_eq!(max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
}
