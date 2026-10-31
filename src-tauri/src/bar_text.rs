const PAUSE_MARK: &str = "⏸ ";

/// The text for the bar: what stands before the title's first spaced hyphen when that is a time,
/// with the pause mark or without, or none, which leaves the icon alone.
///
/// The format is a contract with the page, and an installed app cannot follow a change to it. The
/// titles in the tests are the ones in trygga's `tests/app/compact-timer-title.test.ts`.
pub fn from_title(title: &str) -> Option<&str> {
    let (text, _) = title.split_once(" - ")?;
    is_time(text.strip_prefix(PAUSE_MARK).unwrap_or(text)).then_some(text)
}

fn is_time(text: &str) -> bool {
    let digits = |part: &str| part.bytes().all(|byte| byte.is_ascii_digit());
    text.split_once(':').is_some_and(|(hours, minutes)| {
        !hours.is_empty() && minutes.len() == 2 && digits(hours) && digits(minutes)
    })
}

#[cfg(test)]
mod tests {
    use super::from_title;

    #[test]
    fn a_running_time_stands_in_the_bar() {
        assert_eq!(from_title("0:00 - Timer - trygga"), Some("0:00"));
        assert_eq!(from_title("0:42 - Timer - trygga"), Some("0:42"));
        assert_eq!(from_title("10:05 - Timer - trygga"), Some("10:05"));
        assert_eq!(from_title("26:00 - Timer - trygga"), Some("26:00"));
    }

    #[test]
    fn a_paused_time_stands_in_the_bar_with_its_mark() {
        assert_eq!(from_title("⏸ 1:35 - Timer - trygga"), Some("⏸ 1:35"));
        assert_eq!(from_title("⏸ 0:00 - Timer - trygga"), Some("⏸ 0:00"));
    }

    #[test]
    fn a_title_with_no_time_leaves_the_icon_alone() {
        assert_eq!(from_title("Timer - trygga"), None);
        assert_eq!(from_title("Sign in - trygga"), None);
        assert_eq!(from_title("trygga"), None);
        assert_eq!(from_title(""), None);
    }

    #[test]
    fn a_hyphen_in_the_page_name_changes_nothing() {
        assert_eq!(from_title("0:42 - Sign-in - trygga"), Some("0:42"));
        assert_eq!(from_title("Sign-in - trygga"), None);
        assert_eq!(from_title("Follow-up 1:05 - trygga"), None);
    }

    #[test]
    fn only_a_whole_time_counts() {
        assert_eq!(from_title("1:5 - Timer - trygga"), None);
        assert_eq!(from_title(":05 - Timer - trygga"), None);
        assert_eq!(from_title("1:05"), None);
        assert_eq!(from_title("⏸ - Timer - trygga"), None);
    }
}
