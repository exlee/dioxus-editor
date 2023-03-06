use crate::enums::Glyph;
use crate::line::Line;

#[derive(Debug)]
pub struct EditorData {
    pub lines: Vec<Line>,
}
impl EditorData {
    pub fn new() -> EditorData {
        EditorData { lines: Vec::<Line>::new() }
    }

    pub fn len(&self) -> usize {
        self.lines.len()
    }

    pub fn add_empty_line(&mut self, position: usize) {
        self.lines.insert(position,Line::new())
    }

    pub fn insert(&mut self, line: usize, cursor_pos: usize, glyph: Glyph) {
        self.lines[line-1].as_vec_mut().insert(cursor_pos-1, glyph);    }
}
impl From<&str> for EditorData {
    fn from(value: &str) -> EditorData {
        let mut st = EditorData::new();
        let mut cursor_line: Line = Line::new();
        cursor_line.as_vec_mut().push(Glyph::Cursor);
        st.lines.push(cursor_line);

        for line in value.split('\n') {
            st.lines.push(line.into());
        }

        st
    }
}
