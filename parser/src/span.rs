use serde::{
    Deserialize,
    Serialize,
};

use crate::tokenizer::types::TokenInfo;

/// A source file position defined by a byte offset.
pub type Offset = usize;

/// A source file position defined by a line number and corresponding byte
/// offset into the line indicated by that number.
pub type Position = (
    usize, // The 1-indexed line number
    usize, // The 0-indexed column byte offset
);

/// A span of positions in a source file.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct SourceSpan {
    pub start_pos: Position,
    pub start_off: Offset,
    pub end_pos: Position,
    pub end_off: Offset,
}

impl From<&TokenInfo<'_>> for SourceSpan {
    fn from(token_info: &TokenInfo) -> Self {
        Self {
            start_pos: token_info.start_pos,
            start_off: token_info.start_off,
            end_pos: token_info.end_pos,
            end_off: token_info.end_off,
        }
    }
}

impl From<(&TokenInfo<'_>, &TokenInfo<'_>)> for SourceSpan {
    fn from(tokens: (&TokenInfo, &TokenInfo)) -> Self {
        let (start, end) = tokens;

        Self {
            start_pos: start.start_pos,
            start_off: start.start_off,
            end_pos: end.end_pos,
            end_off: end.end_off,
        }
    }
}

impl From<(&SourceSpan, &SourceSpan)> for SourceSpan {
    fn from(spans: (&SourceSpan, &SourceSpan)) -> Self {
        let (start, end) = spans;

        Self {
            start_pos: start.start_pos,
            start_off: start.start_off,
            end_pos: end.end_pos,
            end_off: end.end_off,
        }
    }
}

pub trait GetSourceSpan {
    fn get_source_span(&self) -> &SourceSpan;
}
