use crate::enums::Glyph;

/// Represents sningle line in editor state.
/// Line is a simple vector of `Renderable` structs,
/// But also supports reporting length, inserting and removing elements at positio
pub struct Line(Vec<Glyph>);

impl Line {
    /// Get length of line
    ///
    ///# Examples:
    /// ```
    /// let line = Line(vec![String::from("hello"), " ".into(), String::from("world")]);
    /// assert_eq!(line.len(), 11)
    /// ```
    pub fn as_vec(&self) -> &Vec<Glyph> {
        &self.0
    }

    pub fn as_vec_mut(&mut self) -> &mut Vec<Glyph> {
        &mut self.0
    }

    pub fn new() -> Line {
        Line(vec![])
    }
}

impl From<String> for Line {
    fn from(value: String) -> Self {
        let vec: Vec<Glyph> = value.chars().map(|c| {
            Glyph::Char(c)
        }).collect();

        Line(vec)
    }
}
impl From<&str> for Line {
    fn from(value: &str) -> Self {
        let vec: Vec<Glyph> = value.chars().map(|c| {
            Glyph::Char(c)
        }).collect();

        Line(vec)
    }
}
impl std::fmt::Debug for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LINE<")?;
        for item in &self.0 {
            write!(f, "{:?}", item)?;
        };
        write!(f, ">")
    }
}
impl AsRef<Line> for Line {
    fn as_ref(&self) -> &Line {
        self
    }
}
