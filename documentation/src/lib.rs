/// Adds one to the number given
/// 
/// # Examples
/// ```
/// let five = 5;
/// 
/// assert_eq!(6, my_crate::add_one(5));
/// ```
/// 
/// # Panics
/// ```
/// let five = 'j';
/// my_crate::add_one(five)
/// ```
/// 
pub fn add_one(x: i32) -> i32 {
    x + 1
}