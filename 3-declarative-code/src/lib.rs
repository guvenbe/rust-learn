#[derive(Debug, Clone, Default)]
pub struct Metadata {
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Document {
    pub body_bytes: usize,
    pub meta: Metadata,
}

#[derive(Debug, Clone)]
pub enum ParseError {
    Internal,
}

#[derive(Debug)]
pub enum ParserState {
    ExpectingHeader { line_number: usize },
    ReadingBody { bytes_remaining: usize },
    ProcessingMetadata { metadata: Metadata },
    Complete { document: Document },
    Error { error: ParseError },
}

impl ParserState {
    pub fn new() -> Self {
        ParserState::ExpectingHeader { line_number: 1 }
    }

    pub fn process_line(&mut self, line: String) -> Result<(), ParseError> {
        use std::mem::replace;

        *self = match replace(
            self,
            ParserState::Error {
                error: ParseError::Internal,
            },
        ) {
            ParserState::ExpectingHeader { line_number } => {
                if line.starts_with("---") {
                    ParserState::ReadingBody { bytes_remaining: 1024 }
                } else {
                    ParserState::ExpectingHeader {
                        line_number: line_number + 1,
                    }
                }
            }
            ParserState::ReadingBody { bytes_remaining } => {
                if bytes_remaining == 0 {
                    ParserState::ProcessingMetadata {
                        metadata: Metadata::default(),
                    }
                } else {
                    let used = line.len();
                    let rem = bytes_remaining.saturating_sub(used);
                    // For demo: if we hit exactly 0, transition next call will finalize.
                    ParserState::ReadingBody { bytes_remaining: rem }
                }
            }
            ParserState::ProcessingMetadata { metadata } => {
                // For demo: finalize immediately
                ParserState::Complete {
                    document: Document {
                        body_bytes: 1024, // pretend full body
                        meta: metadata,
                    },
                }
            }
            other => other, // Complete or Error: keep as-is
        };

        Ok(())
    }
}
