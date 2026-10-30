/// The text for the bar: the `h:mm` a title starts with, or none, which leaves the icon alone.
pub fn from_title(title: &str) -> Option<&str> {
    let first = title.split_whitespace().next()?;
    let (hours, minutes) = first.split_once(':')?;
    let digits = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
    (!hours.is_empty() && minutes.len() == 2 && digits(hours) && digits(minutes)).then_some(first)
}

#[cfg(test)]
mod tests {
    use super::from_title;

    #[test]
    fn takes_the_time_a_title_starts_with() {
        assert_eq!(from_title("1:05 tick=3"), Some("1:05"));
        assert_eq!(from_title("12:40"), Some("12:40"));
    }

    #[test]
    fn a_title_with_no_time_at_its_start_gives_no_text() {
        assert_eq!(from_title("trygga"), None);
        assert_eq!(from_title(""), None);
        assert_eq!(from_title("Website 1:05"), None);
        assert_eq!(from_title("1:5 tick=3"), None);
        assert_eq!(from_title(":05"), None);
    }
}
