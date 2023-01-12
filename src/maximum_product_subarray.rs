pub fn max_product(nums: Vec<i32>) -> i32
{
    let mut curr_product: i32 = nums[0];
    let mut max_product: i32 = nums[0];

    println!("curr: {}, max: {}", curr_product, max_product);
    for num in nums.iter()
    {
        curr_product *= num;

        if curr_product > max_product
        {
            max_product = curr_product
        }

        if curr_product == 0
        {
            curr_product = 1;
        }

        println!("num: {}, curr: {}, max: {}", num, curr_product, max_product);
    }

    return max_product;
}

#[test]
pub fn test()
{
    assert_eq!(max_product(vec![2, 3, -2, 4]), 6);
    println!("done");
    assert_eq!(max_product(vec![-2, 0, 1]), 0);
    println!("done");
    assert_eq!(max_product(vec![-3,-1,-1]), 3);
}
