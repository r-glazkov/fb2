use std::io::Read;

use chrono::NaiveDate;
use xml::attribute::OwnedAttribute;
use xml::common::{Position, TextPosition};
use xml::reader::XmlEvent;
use xml::{EventReader, ParserConfig};

use crate::error::Error;
use crate::*;

pub fn parse<T: Read>(content: T) -> Result<FictionBook, Error> {
    let reader = ParserConfig::new()
        .trim_whitespace(true)
        .whitespace_to_characters(true)
        .cdata_to_characters(true)
        .ignore_comments(true)
        .create_reader(content);
    let mut parser = Parser { reader };
    // skip StartDocument declaration
    parser.seed()?;
    let seed = parser.seed()?;
    let (fiction_book, seed) = parser.parse_element(el::FICTION_BOOK, seed)?;
    if let XmlEvent::EndDocument = seed {
        Ok(fiction_book)
    } else {
        Err(Error::unrecognized_root_element(parser.position()))
    }
}

trait Parse<T: Read> {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error>
    where
        Self: Sized;
}

struct Parser<T: Read> {
    reader: EventReader<T>,
}

impl<T: Read> Parser<T> {
    fn seed(&mut self) -> Result<XmlEvent, Error> {
        Ok(self.reader.next()?)
    }

    fn position(&self) -> TextPosition {
        self.reader.position()
    }

    fn parse_element<U: Parse<T>>(
        &mut self,
        element: &str,
        seed: XmlEvent,
    ) -> Result<(U, XmlEvent), Error> {
        if let XmlEvent::StartElement {
            name, attributes, ..
        } = seed
        {
            if name.local_name == element {
                Ok((U::parse(element, attributes, self)?, self.seed()?))
            } else {
                Err(Error::missing_element(element, self.position()))
            }
        } else {
            Err(Error::missing_element(element, self.position()))
        }
    }

    fn parse_element_or_none<U: Parse<T>>(
        &mut self,
        element: &str,
        seed: XmlEvent,
    ) -> Result<(Option<U>, XmlEvent), Error> {
        if let XmlEvent::StartElement { name, .. } = &seed {
            if name.local_name == element {
                if let XmlEvent::StartElement { attributes, .. } = seed {
                    Ok((Some(U::parse(element, attributes, self)?), self.seed()?))
                } else {
                    unreachable!()
                }
            } else {
                Ok((None, seed))
            }
        } else {
            Ok((None, seed))
        }
    }

    fn parse_elements<U: Parse<T>>(
        &mut self,
        element: &str,
        seed: XmlEvent,
    ) -> Result<(Vec<U>, XmlEvent), Error> {
        let mut event = seed;
        let mut elements = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &event {
            if name.local_name != element {
                break;
            }
            if let XmlEvent::StartElement { attributes, .. } = event {
                elements.push(U::parse(element, attributes, self)?);
            } else {
                unreachable!();
            }
            event = self.seed()?;
        }
        Ok((elements, event))
    }

    fn consume_end_element(&self, element: &str, event: XmlEvent) -> Result<(), Error> {
        if let XmlEvent::EndElement { name } = event {
            if name.local_name == element {
                Ok(())
            } else {
                Err(Error::unbalanced_element(element, self.position()))
            }
        } else {
            Err(Error::unbalanced_element(element, self.position()))
        }
    }

    fn parse_empty_line(&mut self, _: XmlEvent) -> Result<XmlEvent, Error> {
        let seed = self.seed()?;
        self.consume_end_element(el::EMPTY_LINE, seed)?;
        Ok(self.seed()?)
    }

    fn parse_style_link(&mut self) -> Result<(Vec<StyleLinkElement>, XmlEvent), Error> {
        let mut seed = self.seed()?;
        let mut elements = Vec::new();
        loop {
            use StyleLinkElement::*;
            let (element, event) = match &seed {
                XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                    el::STRONG => {
                        let (el, seed) = self.parse_style_link()?;
                        self.consume_end_element(el::STRONG, seed)?;
                        (Strong(el), self.seed()?)
                    }
                    el::EMPHASIS => {
                        let (el, seed) = self.parse_style_link()?;
                        self.consume_end_element(el::EMPHASIS, seed)?;
                        (Emphasis(el), self.seed()?)
                    }
                    el::STYLE => {
                        let (el, seed) = self.parse_style_link()?;
                        self.consume_end_element(el::STYLE, seed)?;
                        (Style(el), self.seed()?)
                    }
                    el::STRIKETHROUGH => {
                        let (el, seed) = self.parse_style_link()?;
                        self.consume_end_element(el::STRIKETHROUGH, seed)?;
                        (Strikethrough(el), self.seed()?)
                    }
                    el::SUBSCRIPT => {
                        let (el, seed) = self.parse_style_link()?;
                        self.consume_end_element(el::SUBSCRIPT, seed)?;
                        (Subscript(el), self.seed()?)
                    }
                    el::SUPERSCRIPT => {
                        let (el, seed) = self.parse_style_link()?;
                        self.consume_end_element(el::SUPERSCRIPT, seed)?;
                        (Superscript(el), self.seed()?)
                    }
                    el::CODE => {
                        let (el, seed) = self.parse_style_link()?;
                        self.consume_end_element(el::CODE, seed)?;
                        (Code(el), self.seed()?)
                    }
                    el::IMAGE => {
                        let (el, seed) = self.parse_element(el::IMAGE, seed)?;
                        (Image(el), seed)
                    }
                    el => return Err(Error::unrecognized_element(el, self.position())),
                },
                XmlEvent::Characters(_) => {
                    if let XmlEvent::Characters(c) = seed {
                        (Text(c), self.seed()?)
                    } else {
                        unreachable!();
                    }
                }
                _ => break,
            };
            elements.push(element);
            seed = event;
        }
        Ok((elements, seed))
    }

    fn parse_style_content(&mut self) -> Result<(Vec<StyleElement>, XmlEvent), Error> {
        let mut seed = self.seed()?;
        let mut elements = Vec::new();
        loop {
            use StyleElement::*;
            let (element, event) = match &seed {
                XmlEvent::StartElement { name, .. } => match name.local_name.as_str() {
                    el::STRONG => {
                        let (el, seed) = self.parse_element(el::STRONG, seed)?;
                        (Strong(el), seed)
                    }
                    el::EMPHASIS => {
                        let (el, seed) = self.parse_element(el::EMPHASIS, seed)?;
                        (Emphasis(el), seed)
                    }
                    el::STYLE => {
                        let (el, seed) = self.parse_element(el::STYLE, seed)?;
                        (Style(el), seed)
                    }
                    el::LINK => {
                        let (el, seed) = self.parse_element(el::LINK, seed)?;
                        (Link(el), seed)
                    }
                    el::STRIKETHROUGH => {
                        let (el, seed) = self.parse_element(el::STRIKETHROUGH, seed)?;
                        (Strikethrough(el), seed)
                    }
                    el::SUBSCRIPT => {
                        let (el, seed) = self.parse_element(el::SUBSCRIPT, seed)?;
                        (Subscript(el), seed)
                    }
                    el::SUPERSCRIPT => {
                        let (el, seed) = self.parse_element(el::SUPERSCRIPT, seed)?;
                        (Superscript(el), seed)
                    }
                    el::CODE => {
                        let (el, seed) = self.parse_element(el::CODE, seed)?;
                        (Code(el), seed)
                    }
                    el::IMAGE => {
                        let (el, seed) = self.parse_element(el::IMAGE, seed)?;
                        (Image(el), seed)
                    }
                    el => return Err(Error::unrecognized_element(el, self.position())),
                },
                XmlEvent::Characters(_) => {
                    if let XmlEvent::Characters(c) = seed {
                        (Text(c), self.seed()?)
                    } else {
                        unreachable!();
                    }
                }
                _ => break,
            };
            elements.push(element);
            seed = event;
        }
        Ok((elements, seed))
    }

    fn take_single_attribute(
        &self,
        expected_name: &str,
        attributes: Vec<OwnedAttribute>,
    ) -> Result<String, Error> {
        for OwnedAttribute { name, value } in attributes {
            if name.local_name == expected_name {
                return Ok(value);
            }
        }
        Err(Error::missing_attribute(expected_name, self.position()))
    }

    fn take_single_attribute_or_none(
        &self,
        expected_name: &str,
        attributes: Vec<OwnedAttribute>,
    ) -> Option<String> {
        for OwnedAttribute { name, value } in attributes {
            if name.local_name == expected_name {
                return Some(value);
            }
        }
        None
    }

    fn require_attribute(&self, name: &str, attribute: Option<String>) -> Result<String, Error> {
        match attribute {
            None => Err(Error::missing_attribute(name, self.position())),
            Some(it) => Ok(it),
        }
    }

    fn parse_enum_element<U: FromStrOpt>(&self, name: &str, value: String) -> Result<U, Error> {
        U::from_str_or_none(&value)
            .ok_or_else(|| Error::unrecognized_element_value(name, value, self.position()))
    }

    fn parse_enum_attribute<U: FromStrOpt>(&self, name: &str, value: String) -> Result<U, Error> {
        U::from_str_or_none(&value)
            .ok_or_else(|| Error::unrecognized_attribute_value(name, value, self.position()))
    }

    fn parse_integer_attribute(&self, name: &str, value: String) -> Result<i32, Error> {
        value
            .parse()
            .map_err(|e| Error::invalid_integer_attribute_value(name, e, self.position()))
    }

    fn parse_float_element(&mut self, name: &str, content: &str) -> Result<f64, Error> {
        content
            .parse()
            .map_err(|e| Error::invalid_float_element_value(name, e, self.position()))
    }

    fn parse_float_attribute(&self, name: &str, value: String) -> Result<f64, Error> {
        value
            .parse()
            .map_err(|e| Error::invalid_float_attribute_value(name, e, self.position()))
    }

    fn parse_link_type(&self, value: Option<String>) -> Result<&'static str, Error> {
        if let Some(value) = value {
            if value != LINK_TYPE {
                return Err(Error::unrecognized_attribute_value(
                    attr::TYPE,
                    value,
                    self.position(),
                ));
            }
        }
        Ok(LINK_TYPE)
    }

    fn parse_lang(&self, value: &str) -> Result<LanguageTag, Error> {
        match value.parse() {
            Ok(it) => Ok(it),
            Err(e) => Err(Error::invalid_language(e, self.position())),
        }
    }

    /// Timezone/offset ignoring gYear parsing.
    fn parse_year(&self, mut content: String) -> Result<i32, Error> {
        let mut len = 0;
        let mut chars = content.chars();
        if let Some('0'..='9' | '+' | '-') = chars.next() {
            len += 1;
        }
        while let Some('0'..='9') = chars.next() {
            len += 1;
        }
        content.drain(len..);
        content
            .parse()
            .map_err(|e| Error::invalid_integer_element_value(el::YEAR, e, self.position()))
    }

    fn parse_date(&self, value: &str) -> Result<NaiveDate, Error> {
        let err = match NaiveDate::parse_from_str(value, "%Y-%m-%d") {
            Ok(it) => return Ok(it),
            Err(e) => e,
        };
        if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%d%z") {
            return Ok(date);
        }
        if let Ok(date) = NaiveDate::parse_from_str(value, "%Y-%m-%dZ") {
            return Ok(date);
        }
        Err(Error::invalid_date(err, self.position()))
    }
}

impl<T: Read> Parse<T> for FictionBook {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        let seed = parser.seed()?;
        let (stylesheets, seed) = parser.parse_elements(el::STYLESHEET, seed)?;
        let (description, seed) = parser.parse_element(el::DESCRIPTION, seed)?;
        let (body, seed) = parser.parse_element(el::BODY, seed)?;
        let (notes_body, seed) = parser.parse_element_or_none(el::BODY, seed)?;
        let (binaries, seed) = parser.parse_elements(el::BINARY, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(FictionBook {
            stylesheets,
            description,
            body,
            notes_body,
            binaries,
        })
    }
}

impl<T: Read> Parse<T> for Stylesheet {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let kind = parser.take_single_attribute(attr::TYPE, attributes)?;
        let content = String::parse(element, Vec::new(), parser)?;
        Ok(Stylesheet { kind, content })
    }
}

impl<T: Read> Parse<T> for Description {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        let seed = parser.seed()?;
        let (title_info, seed) = parser.parse_element(el::TITLE_INFO, seed)?;
        let (src_title_info, seed) = parser.parse_element_or_none(el::SRC_TITLE_INFO, seed)?;
        let (document_info, seed) = parser.parse_element(el::DOCUMENT_INFO, seed)?;
        let (publish_info, seed) = parser.parse_element_or_none(el::PUBLISH_INFO, seed)?;
        let (custom_info, seed) = parser.parse_elements(el::CUSTOM_INFO, seed)?;
        let (output, seed) = parser.parse_elements(el::OUTPUT, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(Description {
            title_info,
            src_title_info,
            document_info,
            publish_info,
            custom_info,
            output,
        })
    }
}

impl<T: Read> Parse<T> for TitleInfo {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        let seed = parser.seed()?;
        let (genres, seed) = parser.parse_elements(el::GENRE, seed)?;
        let (authors, seed) = parser.parse_elements(el::AUTHOR, seed)?;
        let (book_title, seed) = parser.parse_element(el::BOOK_TITLE, seed)?;
        let (annotation, seed) = parser.parse_element_or_none(el::ANNOTATION, seed)?;
        let (keywords, seed) = parser.parse_element_or_none(el::KEYWORDS, seed)?;
        let (date, seed) = parser.parse_element_or_none(el::DATE, seed)?;
        let (cover_page, seed) = parser.parse_element_or_none(el::COVER_PAGE, seed)?;
        let (lang, seed) = parser.parse_element(el::LANG, seed)?;
        let (src_lang, seed) = parser.parse_element_or_none(el::SRC_LANG, seed)?;
        let (translators, seed) = parser.parse_elements(el::TRANSLATOR, seed)?;
        let (sequences, seed) = parser.parse_elements(el::SEQUENCE, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(TitleInfo {
            genres,
            authors,
            book_title,
            annotation,
            keywords,
            date,
            cover_page,
            lang,
            src_lang,
            translators,
            sequences,
        })
    }
}

impl<T: Read> Parse<T> for DocumentInfo {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        let seed = parser.seed()?;
        let (authors, seed) = parser.parse_elements(el::AUTHOR, seed)?;
        let (program_used, seed) = parser.parse_element_or_none(el::PROGRAM_USED, seed)?;
        let (date, seed) = parser.parse_element(el::DATE, seed)?;
        let (src_urls, seed) = parser.parse_elements(el::SRC_URL, seed)?;
        let (src_ocr, seed) = parser.parse_element_or_none(el::SRC_OCR, seed)?;
        let (id, seed) = parser.parse_element(el::ID, seed)?;
        let (version, seed) = parser.parse_element::<String>(el::VERSION, seed)?;
        let version = parser.parse_float_element(el::VERSION, &version)?;
        let (history, seed) = parser.parse_element_or_none(el::HISTORY, seed)?;
        let (publishers, seed) = parser.parse_elements(el::PUBLISHER, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(DocumentInfo {
            authors,
            program_used,
            date,
            src_urls,
            src_ocr,
            id,
            version,
            history,
            publishers,
        })
    }
}

impl<T: Read> Parse<T> for PublishInfo {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        let seed = parser.seed()?;
        let (book_name, seed) = parser.parse_element_or_none(el::BOOK_NAME, seed)?;
        let (publisher, seed) = parser.parse_element_or_none(el::PUBLISHER, seed)?;
        let (city, seed) = parser.parse_element_or_none(el::CITY, seed)?;
        let (year, seed) = parser.parse_element_or_none(el::YEAR, seed)?;
        let year = if let Some(content) = year {
            Some(parser.parse_year(content)?)
        } else {
            None
        };
        let (isbn, seed) = parser.parse_element_or_none(el::ISBN, seed)?;
        let (sequences, seed) = parser.parse_elements(el::SEQUENCE, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(PublishInfo {
            book_name,
            publisher,
            city,
            year,
            isbn,
            sequences,
        })
    }
}

impl<T: Read> Parse<T> for CustomInfo {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut info_type = None;
        let mut lang = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::INFO_TYPE => info_type = info_type.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let info_type = parser.require_attribute(attr::INFO_TYPE, info_type)?;
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };
        let content = String::parse(element, Vec::new(), parser)?;
        let content = LocalizedText { lang, content };
        Ok(CustomInfo { info_type, content })
    }
}

impl<T: Read> Parse<T> for ShareInstruction {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut mode = None;
        let mut include_all = None;
        let mut price = None;
        let mut currency = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::MODE => mode = mode.or(Some(value)),
                attr::INCLUDE_ALL => include_all = include_all.or(Some(value)),
                attr::PRICE => price = price.or(Some(value)),
                attr::CURRENCY => currency = currency.or(Some(value)),
                _ => {}
            }
        }
        let mode = parser.require_attribute(attr::MODE, mode)?;
        let mode = parser.parse_enum_attribute(attr::MODE, mode)?;
        let include_all = parser.require_attribute(attr::INCLUDE_ALL, include_all)?;
        let include_all = parser.parse_enum_attribute(attr::INCLUDE_ALL, include_all)?;
        let price = if let Some(value) = price {
            Some(parser.parse_float_attribute(attr::PRICE, value)?)
        } else {
            None
        };

        let mut seed = parser.seed()?;
        let mut elements = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &seed {
            use ShareInstructionElement::*;
            let (element, event) = match name.local_name.as_str() {
                el::PART => {
                    let (part, event) = parser.parse_element(el::PART, seed)?;
                    (Part(part), event)
                }
                el::OUTPUT_DOCUMENT_CLASS => {
                    let (class, event) = parser.parse_element(el::OUTPUT_DOCUMENT_CLASS, seed)?;
                    (OutputDocumentClass(class), event)
                }
                name => return Err(Error::unrecognized_element(name, parser.position())),
            };
            elements.push(element);
            seed = event;
        }
        parser.consume_end_element(element, seed)?;

        Ok(ShareInstruction {
            mode,
            include_all,
            price,
            currency,
            elements,
        })
    }
}

impl<T: Read> Parse<T> for OutputDocumentClass {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut name = None;
        let mut create = None;
        let mut price = None;
        for OwnedAttribute { name: n, value } in attributes {
            match n.local_name.as_str() {
                attr::NAME => name = name.or(Some(value)),
                attr::CREATE => create = create.or(Some(value)),
                attr::PRICE => price = price.or(Some(value)),
                _ => {}
            }
        }
        let name = parser.require_attribute(attr::NAME, name)?;
        let create = if let Some(value) = create {
            Some(parser.parse_enum_attribute(attr::CREATE, value)?)
        } else {
            None
        };
        let price = if let Some(value) = price {
            Some(parser.parse_float_attribute(attr::PRICE, value)?)
        } else {
            None
        };

        let seed = parser.seed()?;
        let (parts, seed) = parser.parse_elements(el::PART, seed)?;
        parser.consume_end_element(element, seed)?;

        Ok(OutputDocumentClass {
            name,
            create,
            price,
            parts,
        })
    }
}

impl<T: Read> Parse<T> for PartShareInstruction {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut kind = None;
        let mut href = None;
        let mut include = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::TYPE => kind = kind.or(Some(value)),
                attr::HREF => href = href.or(Some(value)),
                attr::INCLUDE => include = include.or(Some(value)),
                _ => {}
            }
        }
        let kind = parser.parse_link_type(kind)?;
        let href = parser.require_attribute(attr::HREF, href)?;
        let include = parser.require_attribute(attr::INCLUDE, include)?;
        let include = parser.parse_enum_attribute(attr::INCLUDE, include)?;

        let seed = parser.seed()?;
        parser.consume_end_element(element, seed)?;

        Ok(PartShareInstruction {
            kind,
            href,
            include,
        })
    }
}

impl<T: Read> Parse<T> for Body {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut name = None;
        let mut lang = None;
        for OwnedAttribute { name: n, value } in attributes {
            match n.local_name.as_str() {
                attr::NAME => name = name.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };
        let seed = parser.seed()?;
        let (image, seed) = parser.parse_element_or_none(el::IMAGE, seed)?;
        let (title, seed) = parser.parse_element_or_none(el::TITLE, seed)?;
        let (epigraphs, seed) = parser.parse_elements(el::EPIGRAPH, seed)?;
        let (sections, seed) = parser.parse_elements(el::SECTION, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(Body {
            name,
            image,
            title,
            epigraphs,
            sections,
            lang,
        })
    }
}

impl<T: Read> Parse<T> for Sequence {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut name = None;
        let mut number = None;
        let mut lang = None;
        for OwnedAttribute { name: n, value } in attributes {
            match n.local_name.as_str() {
                attr::NAME => name = name.or(Some(value)),
                attr::NUMBER => number = number.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let name = parser.require_attribute(attr::NAME, name)?;
        let number = if let Some(value) = number {
            Some(parser.parse_integer_attribute(attr::NUMBER, value)?)
        } else {
            None
        };
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };
        let seed = parser.seed()?;
        let (sequences, seed) = parser.parse_elements(el::SEQUENCE, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(Sequence {
            name,
            number,
            lang,
            sequences,
        })
    }
}

impl<T: Read> Parse<T> for CoverPage {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        let seed = parser.seed()?;
        let (images, seed) = parser.parse_elements(el::IMAGE, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(CoverPage(images))
    }
}

impl<T: Read> Parse<T> for GenreWithMatch {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let match_percentage =
            if let Some(value) = parser.take_single_attribute_or_none(attr::MATCH, attributes) {
                parser.parse_integer_attribute(attr::MATCH, value)?
            } else {
                100
            };
        let value = String::parse(element, Vec::new(), parser)?;
        let value = parser.parse_enum_element(element, value)?;
        Ok(GenreWithMatch {
            match_percentage,
            value,
        })
    }
}

impl<T: Read> Parse<T> for Author {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        let seed = parser.seed()?;
        let verbose = if let XmlEvent::StartElement { name, .. } = &seed {
            match name.local_name.as_str() {
                el::FIRST_NAME => true,
                el::NICKNAME => false,
                el => return Err(Error::unrecognized_element(el, parser.position())),
            }
        } else {
            return Err(Error::missing_element(el::FIRST_NAME, parser.position()));
        };
        use Author::*;
        let (author, seed) = if verbose {
            let (first_name, seed) = parser.parse_element(el::FIRST_NAME, seed)?;
            let (middle_name, seed) = parser.parse_element_or_none(el::MIDDLE_NAME, seed)?;
            let (last_name, seed) = parser.parse_element(el::LAST_NAME, seed)?;
            let (nickname, seed) = parser.parse_element_or_none(el::NICKNAME, seed)?;
            let (home_pages, seed) = parser.parse_elements(el::HOME_PAGE, seed)?;
            let (emails, seed) = parser.parse_elements(el::EMAIL, seed)?;
            let (id, seed) = parser.parse_element_or_none(el::ID, seed)?;
            (
                Verbose(VerboseAuthorDetails {
                    first_name,
                    middle_name,
                    last_name,
                    nickname,
                    home_pages,
                    emails,
                    id,
                }),
                seed,
            )
        } else {
            let (nickname, seed) = parser.parse_element(el::NICKNAME, seed)?;
            let (home_pages, seed) = parser.parse_elements(el::HOME_PAGE, seed)?;
            let (emails, seed) = parser.parse_elements(el::EMAIL, seed)?;
            let (id, seed) = parser.parse_element_or_none(el::ID, seed)?;
            (
                Anonymous(AnonymousAuthorDetails {
                    nickname,
                    home_pages,
                    emails,
                    id,
                }),
                seed,
            )
        };
        parser.consume_end_element(element, seed)?;
        Ok(author)
    }
}

impl<T: Read> Parse<T> for Binary {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut content_type = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::ID => id = id.or(Some(value)),
                attr::CONTENT_TYPE => content_type = content_type.or(Some(value)),
                _ => {}
            }
        }
        let id = parser.require_attribute(attr::ID, id)?;
        let content_type = parser.require_attribute(attr::CONTENT_TYPE, content_type)?;
        let content = String::parse(element, Vec::new(), parser)?;
        Ok(Binary {
            id,
            content_type,
            content,
        })
    }
}

impl<T: Read> Parse<T> for Section {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut lang = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::ID => id = id.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let seed = parser.seed()?;
        let (title, seed) = parser.parse_element_or_none(el::TITLE, seed)?;
        let (epigraphs, seed) = parser.parse_elements(el::EPIGRAPH, seed)?;
        let (image, seed) = parser.parse_element_or_none(el::IMAGE, seed)?;
        let (annotation, seed) = parser.parse_element_or_none(el::ANNOTATION, seed)?;
        let (child_sections, seed) = parser.parse_elements(el::SECTION, seed)?;

        let mut seed = seed;
        let content;
        if !child_sections.is_empty() {
            let value = SectionContentValue::ChildSections(child_sections);
            content = Some(SectionContent {
                title,
                epigraphs,
                image,
                annotation,
                value,
            });
        } else if let XmlEvent::StartElement { name, .. } = &seed {
            let (first, event) = match name.local_name.as_str() {
                el::PARAGRAPH => {
                    let (el, seed) = parser.parse_element(el::PARAGRAPH, seed)?;
                    (FirstSectionPart::Paragraph(el), seed)
                }
                el::POEM => {
                    let (el, seed) = parser.parse_element(el::POEM, seed)?;
                    (FirstSectionPart::Poem(el), seed)
                }
                el::SUBTITLE => {
                    let (el, seed) = parser.parse_element(el::SUBTITLE, seed)?;
                    (FirstSectionPart::Subtitle(el), seed)
                }
                el::CITE => {
                    let (el, seed) = parser.parse_element(el::CITE, seed)?;
                    (FirstSectionPart::Cite(el), seed)
                }
                el::EMPTY_LINE => {
                    let seed = parser.parse_empty_line(seed)?;
                    (FirstSectionPart::EmptyLine, seed)
                }
                el::TABLE => {
                    let (el, seed) = parser.parse_element(el::TABLE, seed)?;
                    (FirstSectionPart::Table(el), seed)
                }
                el => return Err(Error::unrecognized_element(el, parser.position())),
            };
            seed = event;
            let mut rest = Vec::new();
            while let XmlEvent::StartElement { name, .. } = &seed {
                use RestSectionPart::*;
                let (element, event) = match name.local_name.as_str() {
                    el::PARAGRAPH => {
                        let (el, seed) = parser.parse_element(el::PARAGRAPH, seed)?;
                        (Paragraph(el), seed)
                    }
                    el::IMAGE => {
                        let (el, seed) = parser.parse_element(el::IMAGE, seed)?;
                        (Image(el), seed)
                    }
                    el::POEM => {
                        let (el, seed) = parser.parse_element(el::POEM, seed)?;
                        (Poem(el), seed)
                    }
                    el::SUBTITLE => {
                        let (el, seed) = parser.parse_element(el::SUBTITLE, seed)?;
                        (Subtitle(el), seed)
                    }
                    el::CITE => {
                        let (el, seed) = parser.parse_element(el::CITE, seed)?;
                        (Cite(el), seed)
                    }
                    el::EMPTY_LINE => {
                        let seed = parser.parse_empty_line(seed)?;
                        (EmptyLine, seed)
                    }
                    el::TABLE => {
                        let (el, seed) = parser.parse_element(el::TABLE, seed)?;
                        (Table(el), seed)
                    }
                    el => return Err(Error::unrecognized_element(el, parser.position())),
                };
                rest.push(element);
                seed = event;
            }
            let value = SectionContentValue::SectionParts(SectionParts { first, rest });
            content = Some(SectionContent {
                title,
                epigraphs,
                image,
                annotation,
                value,
            });
        } else {
            if title.is_some() || !epigraphs.is_empty() || image.is_some() || annotation.is_some() {
                return Err(Error::missing_section_content(parser.position()));
            }
            content = None;
        }
        parser.consume_end_element(element, seed)?;

        Ok(Section { id, lang, content })
    }
}

impl<T: Read> Parse<T> for Annotation {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut lang = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::ID => id = id.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };
        let mut seed = parser.seed()?;
        let mut elements = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &seed {
            use AnnotationElement::*;
            let (element, event) = match name.local_name.as_str() {
                el::PARAGRAPH => {
                    let (paragraph, seed) = parser.parse_element(el::PARAGRAPH, seed)?;
                    (Paragraph(paragraph), seed)
                }
                el::POEM => {
                    let (poem, seed) = parser.parse_element(el::POEM, seed)?;
                    (Poem(poem), seed)
                }
                el::CITE => {
                    let (cite, seed) = parser.parse_element(el::CITE, seed)?;
                    (Cite(cite), seed)
                }
                el::SUBTITLE => {
                    let (subtitle, seed) = parser.parse_element(el::SUBTITLE, seed)?;
                    (Subtitle(subtitle), seed)
                }
                el::TABLE => {
                    let (table, seed) = parser.parse_element(el::TABLE, seed)?;
                    (Table(table), seed)
                }
                el::EMPTY_LINE => {
                    let seed = parser.parse_empty_line(seed)?;
                    (EmptyLine, seed)
                }
                el => return Err(Error::unrecognized_element(el, parser.position())),
            };
            elements.push(element);
            seed = event;
        }
        parser.consume_end_element(element, seed)?;

        Ok(Annotation { id, lang, elements })
    }
}

impl<T: Read> Parse<T> for Epigraph {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let id = parser.take_single_attribute_or_none(attr::ID, attributes);

        let mut seed = parser.seed()?;
        let mut elements = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &seed {
            use EpigraphElement::*;
            let (el, event) = match name.local_name.as_str() {
                el::PARAGRAPH => {
                    let (el, seed) = parser.parse_element(el::PARAGRAPH, seed)?;
                    (Paragraph(el), seed)
                }
                el::POEM => {
                    let (el, seed) = parser.parse_element(el::POEM, seed)?;
                    (Poem(el), seed)
                }
                el::CITE => {
                    let (el, seed) = parser.parse_element(el::CITE, seed)?;
                    (Cite(el), seed)
                }
                el::EMPTY_LINE => {
                    let seed = parser.parse_empty_line(seed)?;
                    (EmptyLine, seed)
                }
                _ => break,
            };
            elements.push(el);
            seed = event;
        }
        let (text_authors, event) = parser.parse_elements(el::TEXT_AUTHOR, seed)?;
        parser.consume_end_element(element, event)?;

        Ok(Epigraph {
            id,
            elements,
            text_authors,
        })
    }
}

impl<T: Read> Parse<T> for Cite {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut lang = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                el::ID => id = id.or(Some(value)),
                el::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let mut seed = parser.seed()?;
        let mut elements = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &seed {
            use CiteElement::*;
            let (el, event) = match name.local_name.as_str() {
                el::PARAGRAPH => {
                    let (el, seed) = parser.parse_element(el::PARAGRAPH, seed)?;
                    (Paragraph(el), seed)
                }
                el::POEM => {
                    let (el, seed) = parser.parse_element(el::POEM, seed)?;
                    (Poem(el), seed)
                }
                el::EMPTY_LINE => {
                    let seed = parser.parse_empty_line(seed)?;
                    (EmptyLine, seed)
                }
                el::SUBTITLE => {
                    let (el, seed) = parser.parse_element(el::SUBTITLE, seed)?;
                    (Subtitle(el), seed)
                }
                el::TABLE => {
                    let (el, seed) = parser.parse_element(el::TABLE, seed)?;
                    (Table(el), seed)
                }
                _ => break,
            };
            elements.push(el);
            seed = event;
        }
        let (text_authors, seed) = parser.parse_elements(el::TEXT_AUTHOR, seed)?;
        parser.consume_end_element(element, seed)?;

        Ok(Cite {
            id,
            lang,
            elements,
            text_authors,
        })
    }
}

impl<T: Read> Parse<T> for Poem {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut lang = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                el::ID => id = id.or(Some(value)),
                el::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let seed = parser.seed()?;
        let (title, seed) = parser.parse_element_or_none(el::TITLE, seed)?;
        let (epigraphs, seed) = parser.parse_elements(el::EPIGRAPH, seed)?;

        let mut seed = seed;
        let mut stanzas = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &seed {
            use PoemStanza::*;
            let (stanza, event) = match name.local_name.as_str() {
                el::SUBTITLE => {
                    let (el, seed) = parser.parse_element(el::SUBTITLE, seed)?;
                    (Subtitle(el), seed)
                }
                el::STANZA => {
                    let (el, seed) = parser.parse_element(el::STANZA, seed)?;
                    (Stanza(el), seed)
                }
                _ => break,
            };
            stanzas.push(stanza);
            seed = event;
        }

        let (text_authors, seed) = parser.parse_elements(el::TEXT_AUTHOR, seed)?;
        let (date, seed) = parser.parse_element_or_none(el::DATE, seed)?;
        parser.consume_end_element(element, seed)?;

        Ok(Poem {
            id,
            lang,
            title,
            epigraphs,
            stanzas,
            text_authors,
            date,
        })
    }
}

impl<T: Read> Parse<T> for Stanza {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let lang = if let Some(value) = parser.take_single_attribute_or_none(attr::LANG, attributes)
        {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let seed = parser.seed()?;
        let (title, seed) = parser.parse_element_or_none(el::TITLE, seed)?;
        let (subtitle, seed) = parser.parse_element_or_none(el::SUBTITLE, seed)?;
        let (lines, seed) = parser.parse_elements(el::STANZA_LINE, seed)?;
        parser.consume_end_element(element, seed)?;

        Ok(Stanza {
            lang,
            title,
            subtitle,
            lines,
        })
    }
}

impl<T: Read> Parse<T> for Title {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let lang = if let Some(value) = parser.take_single_attribute_or_none(attr::LANG, attributes)
        {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let mut seed = parser.seed()?;
        let mut elements = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &seed {
            use TitleElement::*;
            let (el, event) = match name.local_name.as_str() {
                el::PARAGRAPH => {
                    let (el, seed) = parser.parse_element(el::PARAGRAPH, seed)?;
                    (Paragraph(el), seed)
                }
                el::EMPTY_LINE => {
                    let seed = parser.parse_empty_line(seed)?;
                    (EmptyLine, seed)
                }
                el => return Err(Error::unrecognized_element(el, parser.position())),
            };
            elements.push(el);
            seed = event;
        }
        parser.consume_end_element(element, seed)?;

        Ok(Title { lang, elements })
    }
}

impl<T: Read> Parse<T> for Paragraph {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut lang = None;
        let mut style = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::ID => id = id.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                attr::STYLE => style = style.or(Some(value)),
                _ => {}
            }
        }
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let (elements, seed) = parser.parse_style_content()?;
        parser.consume_end_element(element, seed)?;
        let content = Style { lang, elements };

        Ok(Paragraph { id, style, content })
    }
}

impl<T: Read> Parse<T> for Table {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut style = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::ID => id = id.or(Some(value)),
                attr::STYLE => style = style.or(Some(value)),
                _ => {}
            }
        }
        let seed = parser.seed()?;
        let (rows, seed) = parser.parse_elements(el::TABLE_ROW, seed)?;
        parser.consume_end_element(element, seed)?;
        Ok(Table { id, style, rows })
    }
}

impl<T: Read> Parse<T> for TableRow {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let align =
            if let Some(value) = parser.take_single_attribute_or_none(attr::ALIGN, attributes) {
                parser.parse_enum_attribute(attr::ALIGN, value)?
            } else {
                HorizontalAlign::default()
            };
        let mut seed = parser.seed()?;
        let mut cells = Vec::new();
        while let XmlEvent::StartElement { name, .. } = &seed {
            use TableCellElement::*;
            let (cell, event) = match name.local_name.as_str() {
                el::TABLE_HEAD => {
                    let (head, seed) = parser.parse_element(el::TABLE_HEAD, seed)?;
                    (Head(head), seed)
                }
                el::TABLE_DATA => {
                    let (data, seed) = parser.parse_element(el::TABLE_DATA, seed)?;
                    (Data(data), seed)
                }
                el => return Err(Error::unrecognized_element(el, parser.position())),
            };
            cells.push(cell);
            seed = event;
        }
        parser.consume_end_element(element, seed)?;
        Ok(TableRow { align, cells })
    }
}

impl<T: Read> Parse<T> for TableCell {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut id = None;
        let mut lang = None;
        let mut style = None;
        let mut column_span = None;
        let mut row_span = None;
        let mut horizontal_align = None;
        let mut vertical_align = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::ID => id = id.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                attr::STYLE => style = style.or(Some(value)),
                attr::COLUMN_SPAN => column_span = column_span.or(Some(value)),
                attr::ROW_SPAN => row_span = row_span.or(Some(value)),
                attr::ALIGN => horizontal_align = horizontal_align.or(Some(value)),
                attr::VERTICAL_ALIGN => vertical_align = vertical_align.or(Some(value)),
                _ => {}
            }
        }
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };
        let column_span = if let Some(value) = column_span {
            Some(parser.parse_integer_attribute(attr::COLUMN_SPAN, value)?)
        } else {
            None
        };
        let row_span = if let Some(value) = row_span {
            Some(parser.parse_integer_attribute(attr::ROW_SPAN, value)?)
        } else {
            None
        };
        let horizontal_align = if let Some(value) = horizontal_align {
            parser.parse_enum_attribute(attr::ALIGN, value)?
        } else {
            HorizontalAlign::default()
        };
        let vertical_align = if let Some(value) = vertical_align {
            parser.parse_enum_attribute(attr::VERTICAL_ALIGN, value)?
        } else {
            VerticalAlign::default()
        };

        let (elements, seed) = parser.parse_style_content()?;
        parser.consume_end_element(element, seed)?;
        let content = Style { lang, elements };

        Ok(TableCell {
            id,
            style,
            column_span,
            row_span,
            horizontal_align,
            vertical_align,
            content,
        })
    }
}

impl<T: Read> Parse<T> for NamedStyle {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut name = None;
        let mut lang = None;
        for OwnedAttribute { name: n, value } in attributes {
            match n.local_name.as_str() {
                attr::NAME => name = name.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let name = parser.require_attribute(attr::NAME, name)?;
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let (elements, seed) = parser.parse_style_content()?;
        parser.consume_end_element(element, seed)?;

        Ok(NamedStyle {
            name,
            lang,
            elements,
        })
    }
}

impl<T: Read> Parse<T> for Style {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let lang = if let Some(value) = parser.take_single_attribute_or_none(attr::LANG, attributes)
        {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };
        let (elements, seed) = parser.parse_style_content()?;
        parser.consume_end_element(element, seed)?;
        Ok(Style { lang, elements })
    }
}

impl<T: Read> Parse<T> for Link {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut href = None;
        let mut kind = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::HREF => href = href.or(Some(value)),
                attr::TYPE => kind = kind.or(Some(value)),
                _ => {}
            }
        }
        let href = parser.require_attribute(attr::HREF, href)?;

        let (elements, seed) = parser.parse_style_link()?;
        parser.consume_end_element(element, seed)?;

        Ok(Link {
            href,
            kind,
            elements,
        })
    }
}

impl<T: Read> Parse<T> for Date {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut date = None;
        let mut lang = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::VALUE => date = date.or(Some(value)),
                attr::LANG => lang = lang.or(Some(value)),
                _ => {}
            }
        }
        let date = if let Some(value) = date {
            Some(parser.parse_date(&value)?)
        } else {
            None
        };
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };
        let display_date = String::parse(element, Vec::new(), parser)?;
        Ok(Date {
            date,
            lang,
            display_date,
        })
    }
}

impl<T: Read> Parse<T> for Image {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut kind = None;
        let mut href = None;
        let mut alt = None;
        let mut title = None;
        let mut id = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::TYPE => kind = kind.or(Some(value)),
                attr::HREF => href = href.or(Some(value)),
                attr::ALT => alt = alt.or(Some(value)),
                attr::TITLE => title = title.or(Some(value)),
                attr::ID => id = id.or(Some(value)),
                _ => {}
            }
        }
        let kind = parser.parse_link_type(kind)?;

        let seed = parser.seed()?;
        parser.consume_end_element(element, seed)?;

        Ok(Image {
            kind,
            href,
            alt,
            title,
            id,
        })
    }
}

impl<T: Read> Parse<T> for InlineImage {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let mut kind = None;
        let mut href = None;
        let mut alt = None;
        for OwnedAttribute { name, value } in attributes {
            match name.local_name.as_str() {
                attr::TYPE => kind = kind.or(Some(value)),
                attr::HREF => href = href.or(Some(value)),
                attr::ALT => alt = alt.or(Some(value)),
                _ => {}
            }
        }
        let kind = parser.parse_link_type(kind)?;

        let seed = parser.seed()?;
        parser.consume_end_element(element, seed)?;

        Ok(InlineImage { kind, href, alt })
    }
}

impl<T: Read> Parse<T> for LocalizedText {
    fn parse(
        element: &str,
        attributes: Vec<OwnedAttribute>,
        parser: &mut Parser<T>,
    ) -> Result<Self, Error> {
        let lang = parser.take_single_attribute_or_none(attr::LANG, attributes);
        let lang = if let Some(value) = lang {
            Some(parser.parse_lang(&value)?)
        } else {
            None
        };

        let content = String::parse(element, Vec::new(), parser)?;
        Ok(LocalizedText { lang, content })
    }
}

impl<T: Read> Parse<T> for String {
    fn parse(element: &str, _: Vec<OwnedAttribute>, parser: &mut Parser<T>) -> Result<Self, Error> {
        if let XmlEvent::Characters(content) = parser.seed()? {
            let seed = parser.seed()?;
            parser.consume_end_element(element, seed)?;
            Ok(content)
        } else {
            Err(Error::missing_content(parser.position()))
        }
    }
}

mod el {
    pub(super) const ANNOTATION: &str = "annotation";
    pub(super) const AUTHOR: &str = "author";
    pub(super) const BINARY: &str = "binary";
    pub(super) const BODY: &str = "body";
    pub(super) const BOOK_NAME: &str = "book-name";
    pub(super) const BOOK_TITLE: &str = "book-title";
    pub(super) const CITE: &str = "cite";
    pub(super) const CITY: &str = "city";
    pub(super) const CODE: &str = "code";
    pub(super) const COVER_PAGE: &str = "coverpage";
    pub(super) const CUSTOM_INFO: &str = "custom-info";
    pub(super) const DATE: &str = "date";
    pub(super) const DESCRIPTION: &str = "description";
    pub(super) const DOCUMENT_INFO: &str = "document-info";
    pub(super) const EMAIL: &str = "email";
    pub(super) const EMPHASIS: &str = "emphasis";
    pub(super) const EMPTY_LINE: &str = "empty-line";
    pub(super) const EPIGRAPH: &str = "epigraph";
    pub(super) const FICTION_BOOK: &str = "FictionBook";
    pub(super) const FIRST_NAME: &str = "first-name";
    pub(super) const GENRE: &str = "genre";
    pub(super) const HISTORY: &str = "history";
    pub(super) const HOME_PAGE: &str = "home-page";
    pub(super) const ID: &str = "id";
    pub(super) const IMAGE: &str = "image";
    pub(super) const ISBN: &str = "isbn";
    pub(super) const KEYWORDS: &str = "keywords";
    pub(super) const LANG: &str = "lang";
    pub(super) const LAST_NAME: &str = "last-name";
    pub(super) const LINK: &str = "a";
    pub(super) const MIDDLE_NAME: &str = "middle-name";
    pub(super) const NICKNAME: &str = "nickname";
    pub(super) const OUTPUT: &str = "output";
    pub(super) const OUTPUT_DOCUMENT_CLASS: &str = "output-document-class";
    pub(super) const PARAGRAPH: &str = "p";
    pub(super) const PART: &str = "part";
    pub(super) const POEM: &str = "poem";
    pub(super) const PROGRAM_USED: &str = "program-used";
    pub(super) const PUBLISHER: &str = "publisher";
    pub(super) const PUBLISH_INFO: &str = "publish-info";
    pub(super) const SECTION: &str = "section";
    pub(super) const SEQUENCE: &str = "sequence";
    pub(super) const SRC_LANG: &str = "src-lang";
    pub(super) const SRC_OCR: &str = "src-ocr";
    pub(super) const SRC_TITLE_INFO: &str = "src-title-info";
    pub(super) const SRC_URL: &str = "src-url";
    pub(super) const STANZA: &str = "stanza";
    pub(super) const STANZA_LINE: &str = "v";
    pub(super) const STRIKETHROUGH: &str = "strikethrough";
    pub(super) const STRONG: &str = "strong";
    pub(super) const STYLE: &str = "style";
    pub(super) const STYLESHEET: &str = "stylesheet";
    pub(super) const SUBSCRIPT: &str = "sub";
    pub(super) const SUBTITLE: &str = "subtitle";
    pub(super) const SUPERSCRIPT: &str = "sup";
    pub(super) const TABLE: &str = "table";
    pub(super) const TABLE_DATA: &str = "td";
    pub(super) const TABLE_HEAD: &str = "th";
    pub(super) const TABLE_ROW: &str = "tr";
    pub(super) const TEXT_AUTHOR: &str = "text-author";
    pub(super) const TITLE: &str = "title";
    pub(super) const TITLE_INFO: &str = "title-info";
    pub(super) const TRANSLATOR: &str = "translator";
    pub(super) const VERSION: &str = "version";
    pub(super) const YEAR: &str = "year";
}

mod attr {
    pub(super) const ALIGN: &str = "align";
    pub(super) const ALT: &str = "alt";
    pub(super) const COLUMN_SPAN: &str = "colspan";
    pub(super) const CONTENT_TYPE: &str = "content-type";
    pub(super) const CREATE: &str = "create";
    pub(super) const CURRENCY: &str = "currency";
    pub(super) const HREF: &str = "href";
    pub(super) const ID: &str = "id";
    pub(super) const INCLUDE: &str = "include";
    pub(super) const INCLUDE_ALL: &str = "include-all";
    pub(super) const INFO_TYPE: &str = "info-type";
    pub(super) const LANG: &str = "lang";
    pub(super) const MATCH: &str = "match";
    pub(super) const MODE: &str = "mode";
    pub(super) const NAME: &str = "name";
    pub(super) const NUMBER: &str = "number";
    pub(super) const PRICE: &str = "price";
    pub(super) const ROW_SPAN: &str = "rowspan";
    pub(super) const STYLE: &str = "style";
    pub(super) const TITLE: &str = "title";
    pub(super) const TYPE: &str = "type";
    pub(super) const VALUE: &str = "value";
    pub(super) const VERTICAL_ALIGN: &str = "valign";
}
