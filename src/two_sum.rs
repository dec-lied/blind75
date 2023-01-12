pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>
{
    use std::collections::HashMap;

    let mut diff_map: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len()
    {
        if diff_map.contains_key(&(target - nums[i]))
        {
            return vec![diff_map[&(target - nums[i])], i as i32];
        }
        else
        {
            diff_map.insert(nums[i], i as i32);
        }
    }

    return vec![-1, -1];
}

#[test]
pub fn test()
{
    assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    assert_eq!(two_sum(vec![3, 2, 4], 6), vec![1, 2]);
    assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
}
