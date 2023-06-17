use regex::Regex;

use ansi_term::ANSIGenericString;
use ansi_term::Colour;

pub(crate) trait PaintStriped {
    fn paint_striped(&self, input: &str) -> ANSIGenericString<'_, str>;
}

impl PaintStriped for Colour {
    fn paint_striped(&self, input: &str) -> ANSIGenericString<'_, str> {
        self.paint({
            let regex = Regex::new(r"^\d{2}m").unwrap();
            regex.replace(input, "").into_owned()
        })
    }
}
