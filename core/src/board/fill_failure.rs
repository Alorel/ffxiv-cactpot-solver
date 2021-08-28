use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FillFailure {
    ValueAlreadyContained,
    PositionAlreadyFilled,
}

impl Display for FillFailure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_ref())
    }
}

impl AsRef<str> for FillFailure {
    fn as_ref(&self) -> &str {
        match self {
            Self::PositionAlreadyFilled => "PositionAlreadyFilled",
            Self::ValueAlreadyContained => "ValueAlreadyContained",
        }
    }
}
