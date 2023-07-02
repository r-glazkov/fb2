use std::fmt;

use xml::common::TextPosition;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Schema { pos: TextPosition, kind: ErrorKind },
    Xml(xml::reader::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;
        match self {
            Schema { pos, kind } => write!(f, "{pos} {kind}"),
            Xml(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use Error::*;
        use ErrorKind::*;
        match self {
            Schema { kind, .. } => match kind {
                InvalidDate(e) => Some(e),
                InvalidFloatAttributeValue(_, e) => Some(e),
                InvalidFloatElementValue(_, e) => Some(e),
                InvalidIntegerAttributeValue(_, e) => Some(e),
                InvalidIntegerElementValue(_, e) => Some(e),
                InvalidLanguage(e) => Some(e),
                MissingAttribute(_) => None,
                MissingContent => None,
                MissingElement(_) => None,
                MissingSectionContent => None,
                UnbalancedElement(_) => None,
                UnrecognizedAttributeValue(_, _) => None,
                UnrecognizedElement(_) => None,
                UnrecognizedElementValue(_, _) => None,
                UnrecognizedRootElement => None,
            },
            Xml(e) => Some(e),
        }
    }
}

impl From<xml::reader::Error> for Error {
    fn from(value: xml::reader::Error) -> Self {
        Error::Xml(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidDate(chrono::ParseError),
    InvalidFloatAttributeValue(String, std::num::ParseFloatError),
    InvalidFloatElementValue(String, std::num::ParseFloatError),
    InvalidIntegerAttributeValue(String, std::num::ParseIntError),
    InvalidIntegerElementValue(String, std::num::ParseIntError),
    InvalidLanguage(language_tags::ParseError),
    MissingAttribute(String),
    MissingContent,
    MissingElement(String),
    MissingSectionContent,
    UnbalancedElement(String),
    UnrecognizedAttributeValue(String, String),
    UnrecognizedElement(String),
    UnrecognizedElementValue(String, String),
    UnrecognizedRootElement,
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ErrorKind::*;
        match self {
            InvalidDate(e) => write!(f, "invalid date value: {e}"),
            InvalidFloatAttributeValue(name, e) => write!(f, "invalid float attribute={name} value: {e}"),
            InvalidFloatElementValue(name, e) => write!(f, "invalid float element={name} content: {e}"),
            InvalidIntegerAttributeValue(name, e) => write!(f, "invalid integer attribute={name} value: {e}"),
            InvalidIntegerElementValue(name, e) => write!(f, "invalid integer element={name} content: {e}"),
            InvalidLanguage(e) => write!(f, "invalid language value: {e}"),
            MissingAttribute(name) => write!(f, "expected attribute={name}"),
            MissingContent => write!(f, "expected text content"),
            MissingElement(name) => write!(f, "expected element={name}"),
            MissingSectionContent => write!(f, "required section content was omitted (either child sections or section parts) while specifying optional section elements"),
            UnbalancedElement(name) => write!(f, "unbalanced element={name}"),
            UnrecognizedAttributeValue(name, value) => write!(f, "unrecognized attribute={name} value={value}"),
            UnrecognizedElement(name) => write!(f, "unrecognized element={name}"),
            UnrecognizedElementValue(name, value) => write!(f, "unrecognized element={name} value={value}"),
            UnrecognizedRootElement => write!(f, "expected to process end of the document after fiction book parsing, received unrecognized element instead"),
        }
    }
}

impl Error {
    pub(crate) fn invalid_date(error: chrono::ParseError, pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::InvalidDate(error),
        }
    }

    pub(crate) fn invalid_float_attribute_value(
        name: &str,
        error: std::num::ParseFloatError,
        pos: TextPosition,
    ) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::InvalidFloatAttributeValue(name.into(), error),
        }
    }

    pub(crate) fn invalid_integer_element_value(
        name: &str,
        error: std::num::ParseIntError,
        pos: TextPosition,
    ) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::InvalidIntegerElementValue(name.into(), error),
        }
    }

    pub(crate) fn invalid_integer_attribute_value(
        name: &str,
        error: std::num::ParseIntError,
        pos: TextPosition,
    ) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::InvalidIntegerAttributeValue(name.into(), error),
        }
    }

    pub(crate) fn invalid_float_element_value(
        name: &str,
        error: std::num::ParseFloatError,
        pos: TextPosition,
    ) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::InvalidFloatElementValue(name.into(), error),
        }
    }

    pub(crate) fn invalid_language(error: language_tags::ParseError, pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::InvalidLanguage(error),
        }
    }

    pub(crate) fn missing_attribute(name: &str, pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::MissingAttribute(name.into()),
        }
    }

    pub(crate) fn missing_content(pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::MissingContent,
        }
    }

    pub(crate) fn missing_element(name: &str, pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::MissingElement(name.into()),
        }
    }

    pub(crate) fn missing_section_content(pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::MissingSectionContent,
        }
    }

    pub(crate) fn unbalanced_element(name: &str, pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::UnbalancedElement(name.into()),
        }
    }

    pub(crate) fn unrecognized_attribute_value(
        name: &str,
        value: String,
        pos: TextPosition,
    ) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::UnrecognizedAttributeValue(name.into(), value),
        }
    }

    pub(crate) fn unrecognized_element(name: &str, pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::UnrecognizedElement(name.into()),
        }
    }

    pub(crate) fn unrecognized_element_value(
        name: &str,
        value: String,
        pos: TextPosition,
    ) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::UnrecognizedElementValue(name.into(), value),
        }
    }

    pub(crate) fn unrecognized_root_element(pos: TextPosition) -> Error {
        Error::Schema {
            pos,
            kind: ErrorKind::UnrecognizedRootElement,
        }
    }
}

