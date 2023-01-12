pub fn is_palindrome(x: i32) -> bool
{
    let int_string: String = x.to_string();
    let reverse: String = int_string.chars().rev().collect();

    return int_string == reverse;
}

#[test]
pub fn test()
{
    assert!(is_palindrome(121));
    assert!(is_palindrome(555));
    assert!(!is_palindrome(20));
    assert!(!is_palindrome(-121));
}
