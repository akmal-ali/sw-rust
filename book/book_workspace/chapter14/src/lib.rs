#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg =5;
///	let answer = chapter14::add_one(arg);
/// assert_eq!(6,answer);
/// ```
pub fn add_one(x: i32) -> i32 {
	x + 1
}

// @todo learn markdown syntax
