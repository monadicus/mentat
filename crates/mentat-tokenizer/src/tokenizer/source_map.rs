use std::{
    cell::RefCell,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
};

use super::span::{BytePos, CharPos, Span};
use crate::errors::{LexerError, Result};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct MultiByteChar {
    pub pos: BytePos,
    pub bytes: u8,
}

fn normalize_src(src: &mut String) {
    normalize_newlines(src);
}

fn normalize_newlines(src: &mut String) {
    if !src.as_bytes().contains(&b'\r') {
        return;
    }

    let mut buf = std::mem::take(src).into_bytes();
    let mut gap_len = 0;
    let mut tail = buf.as_mut_slice();
    loop {
        let idx = match find_crlf(&tail[gap_len..]) {
            None => tail.len(),
            Some(idx) => idx + gap_len,
        };
        tail.copy_within(gap_len..idx, 0);
        tail = &mut tail[idx - gap_len..];
        if tail.len() == gap_len {
            break;
        }
        gap_len += 1;
    }

    // Account for removed `\r`.
    // After `buf.truncate(..)`, `buf` is guaranteed to contain utf-8 again.
    let new_len = buf.len() - gap_len;
    buf.truncate(new_len);
    *src = String::from_utf8(buf).unwrap();

    fn find_crlf(src: &[u8]) -> Option<usize> {
        let mut search_idx = 0;
        while let Some(idx) = find_cr(&src[search_idx..]) {
            if src[search_idx..].get(idx + 1) != Some(&b'\n') {
                search_idx += idx + 1;
                continue;
            }
            return Some(search_idx + idx);
        }
        None
    }

    fn find_cr(src: &[u8]) -> Option<usize> {
        src.iter().position(|&b| b == b'\r')
    }
}

fn analyze_source_file(
    src: &str,
    source_file_start_pos: BytePos,
) -> (Vec<BytePos>, Vec<MultiByteChar>) {
    let mut lines = vec![source_file_start_pos];
    let mut multi_byte_chars = vec![];

    let mut i = 0;
    let src_bytes = src.as_bytes();

    while i < src.len() {
        let i_usize = i as usize;
        let byte = src_bytes[i_usize];
        let mut char_len = 1;

        let pos = BytePos::from(i) + source_file_start_pos;

        if let b'\n' = byte {
            lines.push(pos + BytePos(1));
        } else if byte >= 127 {
            let c = (src[i..]).chars().next().unwrap();
            char_len = c.len_utf8();

            if char_len > 1 {
                assert!((2..=4).contains(&char_len));
                let bytes = char_len as u8;
                let mbc = MultiByteChar { pos, bytes };
                multi_byte_chars.push(mbc);
            }
        }

        i += char_len;
    }

    if let Some(&last_line_start) = lines.last() {
        let source_file_end = source_file_start_pos + BytePos::from(src.len());
        assert!(source_file_end >= last_line_start);
        if last_line_start == source_file_end {
            lines.pop();
        }
    }

    (lines, multi_byte_chars)
}

#[derive(Debug)]
pub struct SourceFile {
    pub name: PathBuf,
    pub src: String,
    pub start_pos: BytePos,
    pub end_pos: BytePos,
    lines: Vec<BytePos>,
    multibyte_chars: Vec<MultiByteChar>,
}

impl SourceFile {
    fn new(name: PathBuf, mut src: String, start_pos: BytePos) -> Self {
        normalize_src(&mut src);
        let end_pos = start_pos + BytePos::from(src.len());
        let (lines, multibyte_chars) = analyze_source_file(&src, start_pos);
        Self {
            name,
            src,
            start_pos,
            end_pos,
            lines,
            multibyte_chars,
        }
    }

    fn bytepos_to_file_charpos(&self, bpos: BytePos) -> CharPos {
        let mut total_extra_bytes = 0;

        for mbc in self.multibyte_chars.iter() {
            if mbc.pos < bpos {
                total_extra_bytes += mbc.bytes as u32 - 1;
                assert!(u32::from(bpos) >= u32::from(mbc.pos) + mbc.bytes as u32);
            } else {
                break;
            }
        }

        assert!(u32::from(self.start_pos) + total_extra_bytes <= bpos.into());
        CharPos(usize::from(bpos) - usize::from(self.start_pos) - total_extra_bytes as usize)
    }

    fn lookup_line(&self, pos: BytePos) -> Option<usize> {
        match self.lines.binary_search(&pos) {
            Ok(idx) => Some(idx),
            Err(0) => None,
            Err(idx) => Some(idx - 1),
        }
    }

    fn lookup_file_pos(&self, pos: BytePos) -> (usize, CharPos) {
        let chpos = self.bytepos_to_file_charpos(pos);
        match self.lookup_line(pos) {
            Some(a) => {
                let line = a + 1; // Line numbers start at 1
                let linebpos = self.lines[a];
                let linechpos = self.bytepos_to_file_charpos(linebpos);
                let col = chpos - linechpos;
                assert!(chpos >= linechpos);
                (line, col)
            }
            None => (0, chpos),
        }
    }

    /// Returns contents of a `span` assumed to be within the given file.
    fn contents_of_span(&self, span: Span) -> String {
        let begin_pos = self.bytepos_to_file_charpos(span.start).into();
        let end_pos = self.bytepos_to_file_charpos(span.stop).into();
        String::from_utf8_lossy(&self.src.as_bytes()[begin_pos..end_pos]).into_owned()
    }
}

#[derive(Default)]
struct SourceMapInner {
    used_address_space: u32,
    source_files: Vec<Rc<SourceFile>>,
}

impl SourceMapInner {
    fn try_allocate_address_space(&mut self, size: u32) -> Option<BytePos> {
        let current = self.used_address_space;
        self.used_address_space = current.checked_add(size)?.checked_add(1)?;
        Some(BytePos(current))
    }
}

#[derive(Default)]
pub struct SourceMap {
    inner: RefCell<SourceMapInner>,
}

pub struct LineCol {
    pub source_file: Rc<SourceFile>,
    pub line: usize,
    pub col: CharPos,
}

pub struct SpanLocation {
    pub source_file: Rc<SourceFile>,
    pub line_start: usize,
    pub line_stop: usize,
    pub col_start: usize,
    pub col_stop: usize,
}

impl SpanLocation {
    pub fn dummy() -> Self {
        let dummy = "<dummy>".to_owned();
        let span = Span::dummy();
        Self {
            source_file: Rc::new(SourceFile {
                name: "".into(),
                src: dummy,
                start_pos: span.start,
                end_pos: span.stop,
                lines: Vec::new(),
                multibyte_chars: Vec::new(),
            }),
            line_start: 0,
            line_stop: 0,
            col_start: 0,
            col_stop: 0,
        }
    }
}

impl SourceMap {
    pub fn load_file(&self, path: &Path) -> Result<Rc<SourceFile>> {
        Ok(self.new_source(
            &LexerError::could_not_load_file(fs::read_to_string(path), path.display())?,
            path.to_owned(),
        ))
    }

    pub fn new_source(&self, source: &str, name: PathBuf) -> Rc<SourceFile> {
        let len = u32::try_from(source.len()).unwrap();
        let mut inner = self.inner.borrow_mut();
        let start_pos = inner.try_allocate_address_space(len).unwrap();
        let source_file = Rc::new(SourceFile::new(name, source.to_owned(), start_pos));
        inner.source_files.push(source_file.clone());
        source_file
    }

    fn find_source_file_index(&self, pos: BytePos) -> Option<usize> {
        self.inner
            .borrow()
            .source_files
            .binary_search_by_key(&pos, |file| file.start_pos)
            .map_or_else(|p| p.checked_sub(1), Some)
    }

    fn find_source_file(&self, pos: BytePos) -> Option<Rc<SourceFile>> {
        Some(self.inner.borrow().source_files[self.find_source_file_index(pos)?].clone())
    }

    fn find_line_col(&self, pos: BytePos) -> Option<LineCol> {
        let source_file = self.find_source_file(pos)?;
        let (line, col) = source_file.lookup_file_pos(pos);
        Some(LineCol {
            source_file,
            line,
            col,
        })
    }

    pub(super) fn span_to_location(&self, sp: Span) -> Option<SpanLocation> {
        let lo = self.find_line_col(sp.start)?;
        let hi = self.find_line_col(sp.stop)?;
        Some(SpanLocation {
            source_file: lo.source_file,
            line_start: lo.line,
            line_stop: hi.line,
            col_start: usize::from(lo.col) + 1,
            col_stop: usize::from(hi.col) + 1,
        })
    }

    pub fn span_to_string(&self, span: Span) -> String {
        let loc = match self.span_to_location(span) {
            None => return "no-location".to_string(),
            Some(l) => l,
        };

        if loc.line_start == loc.line_stop {
            format!("{}:{}-{}", loc.line_start, loc.col_start, loc.col_stop)
        } else {
            format!(
                "{}:{}-{}:{}",
                loc.line_start, loc.col_start, loc.line_stop, loc.col_stop
            )
        }
    }

    pub fn contents_of_span(&self, span: Span) -> Option<String> {
        let begin = self.find_source_file(span.start)?;
        let end = self.find_source_file(span.stop)?;
        assert_eq!(begin.start_pos, end.start_pos);
        Some(begin.contents_of_span(span))
    }

    pub(super) fn line_contents_of_span(&self, span: Span) -> Option<String> {
        let begin = self.find_source_file(span.start)?;
        let end = self.find_source_file(span.stop)?;
        assert_eq!(begin.start_pos, end.start_pos);

        let idx_lo = begin.lookup_line(span.start).unwrap_or(0);
        let idx_hi = begin.lookup_line(span.stop).unwrap_or(0) + 1;
        let lo_line_pos = begin.lines[idx_lo];
        let hi_line_pos = if idx_hi < begin.lines.len() {
            begin.lines[idx_hi]
        } else {
            begin.end_pos
        };
        Some(begin.contents_of_span(Span::new(lo_line_pos, hi_line_pos)))
    }
}

scoped_tls::scoped_thread_local!(pub static SOURCE_MAP: SourceMap);

#[inline]
pub fn set_source_map_if_not_set<R>(f: impl FnOnce(&SourceMap) -> R) -> R {
    if !SOURCE_MAP.is_set() {
        let sg = SourceMap::default();
        SOURCE_MAP.set(&sg, || SOURCE_MAP.with(f))
    } else {
        SOURCE_MAP.with(f)
    }
}

#[inline]
pub fn with_source_map<R>(f: impl FnOnce(&SourceMap) -> R) -> R {
    SOURCE_MAP.with(f)
}
