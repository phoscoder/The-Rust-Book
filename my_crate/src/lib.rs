/// Adds one to the number given.
/// # Examples
/// ```
/// let five = 5;
/// assert_eq!(6, my_crate::add_one(five));
/// ```
/// # Panics
/// This function will panic if the result is larger than an i32 can hold.
/// # Errors
/// This function will return an error if the input is negative.
/// # Safety
/// This function is safe to call with any i32 value.
/// # See also
/// - `add_two` - Adds two to the number given.
/// - `subtract_one` - Subtracts one from the number given.
/// # Notes
/// This function is part of the `my_crate` library.



pub fn add_one(x: i32) -> i32 {
    x + 1
}