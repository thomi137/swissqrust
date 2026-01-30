/// Helpers for String manipultion

/// Removes whitespace in-place
/// taken from
/// [Stackoverflow](https://stackoverflow.com/questions/57063777/remove-all-whitespace-from-a-string)
///
/// Using this because it is a little faster than a new alloc.
/// For validation purposes, that should suffice. Since I use it with a ref,
/// the performance should not increase that much, though.
///
/// ```
/// use swiss_qrust::utils::remove_whitespace;
/// let mut s = String::from("This has whitespace");
/// remove_whitespace(&mut s);
/// assert_eq!(s, "Thishaswhitespace");
/// ```
pub fn remove_whitespace(s: &mut String) {
    s.retain(|c| !c.is_whitespace());
}