use std::{fmt, path::Path};

use super::model::Action;

pub fn needs_confirmation(action: &Action) -> bool {
    matches!(action, Action::Remove)
}

pub fn format_confirmation(w: &mut impl fmt::Write, link: &Path, action: &Action) -> fmt::Result {
    match action {
        Action::Remove => writeln!(
            w,
            "  remove {}? this cannot be undone. [y/N]",
            link.display()
        ),
        Action::Relink(target) => writeln!(
            w,
            "  relink {} -> {}? [y/N]",
            link.display(),
            target.display()
        ),
        Action::Skip => Ok(()),
    }
}

pub fn parse_confirmation(input: &str) -> bool {
    matches!(input.trim().to_ascii_lowercase().as_str(), "y" | "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_needs_confirmation() {
        assert!(needs_confirmation(&Action::Remove));
    }

    #[test]
    fn relink_does_not_need_confirmation() {
        assert!(!needs_confirmation(&Action::Relink("/target".into())));
    }

    #[test]
    fn skip_does_not_need_confirmation() {
        assert!(!needs_confirmation(&Action::Skip));
    }

    #[test]
    fn confirm_yes() {
        assert!(parse_confirmation("y"));
        assert!(parse_confirmation("Y"));
        assert!(parse_confirmation("yes"));
        assert!(parse_confirmation("YES"));
        assert!(parse_confirmation("  y  "));
    }

    #[test]
    fn confirm_no() {
        assert!(!parse_confirmation("n"));
        assert!(!parse_confirmation("no"));
        assert!(!parse_confirmation(""));
        assert!(!parse_confirmation("  "));
        assert!(!parse_confirmation("nah"));
        assert!(!parse_confirmation("yep"));
    }

    #[test]
    fn empty_defaults_to_no() {
        assert!(!parse_confirmation(""));
    }

    #[test]
    fn remove_confirmation_prompt() {
        let mut out = String::new();
        format_confirmation(&mut out, Path::new("/home/user/link"), &Action::Remove).unwrap();
        assert!(out.contains("remove /home/user/link"));
        assert!(out.contains("cannot be undone"));
        assert!(out.contains("[y/N]"));
    }

    #[test]
    fn relink_confirmation_prompt() {
        let mut out = String::new();
        format_confirmation(
            &mut out,
            Path::new("/home/user/link"),
            &Action::Relink("/new/target".into()),
        )
        .unwrap();
        assert!(out.contains("relink /home/user/link -> /new/target"));
        assert!(out.contains("[y/N]"));
    }

    #[test]
    fn skip_produces_no_output() {
        let mut out = String::new();
        format_confirmation(&mut out, Path::new("/link"), &Action::Skip).unwrap();
        assert!(out.is_empty());
    }
}
