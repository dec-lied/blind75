pub fn product_except_self(nums: Vec<i32>) -> Vec<i32>
{
    let mut product_array: Vec<i32> = Vec::new();

    for num in nums.iter()
    {
        nums.iter()
            .map
            (
                |n|
                {
                    if num != n
                    {

                    }
                    else
                    {
                        1
                    }
                }
            )
    }

    return product_array;
}
