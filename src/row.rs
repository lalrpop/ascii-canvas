use crate::style::{Style, StyleCursor};
use std::fmt::{Debug, Display, Error, Formatter};

pub struct Row {
    text: String,
    styles: Vec<Style>,
}

impl Row {
    pub fn new(chars: &[char], styles: &[Style]) -> Row {
        assert_eq!(chars.len(), styles.len());
        Row {
            text: chars.iter().cloned().collect(),
            styles: styles.to_vec(),
        }
    }

    pub fn write_to<T: std::io::Write + std::io::IsTerminal>(
        &self,
        term: &mut T,
    ) -> std::io::Result<()> {
        let mut cursor = StyleCursor::new(term)?;
        for (character, &style) in self.text.trim_end().chars().zip(&self.styles) {
            cursor.set_style(style)?;
            write!(cursor.term(), "{}", character)?;
        }
        Ok(())
    }
}

// Using display/debug just skips the styling.

impl Display for Row {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        Display::fmt(self.text.trim_end(), fmt)
    }
}

impl Debug for Row {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> Result<(), Error> {
        // NB: use Display, not Debug, just throw some quotes around it
        write!(fmt, "\"")?;
        Display::fmt(self.text.trim_end(), fmt)?;
        write!(fmt, "\"")
    }
}
