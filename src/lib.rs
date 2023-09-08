use language_tags::LanguageTag;
use serde::ser::{SerializeStructVariant, SerializeTupleVariant};
use serde::{Deserialize, Serialize, Serializer};
use std::num::ParseIntError;

mod defaults {
    use super::{
        Date, Genre, GenreWithMatch, HorizontalAlign, MaybeEmptyLocalizedText, VerticalAlign,
    };

    const DEFAULT_LINK_TYPE: &str = "simple";

    pub(super) fn genres() -> Vec<GenreWithMatch> {
        vec![GenreWithMatch {
            match_percentage: genre_match(),
            value: Genre::default(),
        }]
    }

    pub(super) const fn genre_match() -> i32 {
        100
    }

    pub(super) fn is_default_genre_match(value: &i32) -> bool {
        *value == genre_match()
    }

    pub(super) fn link_type() -> String {
        DEFAULT_LINK_TYPE.to_string()
    }

    pub(super) fn is_default_link_type(value: &str) -> bool {
        value == DEFAULT_LINK_TYPE
    }

    pub(super) fn is_default_horizontal_align(value: &HorizontalAlign) -> bool {
        HorizontalAlign::default() == *value
    }

    pub(super) fn is_default_vertical_align(value: &VerticalAlign) -> bool {
        VerticalAlign::default() == *value
    }

    pub(super) fn should_skip_serializing_date(value: &Option<Date>) -> bool {
        if let Some(Date {
            iso_date,
            display_date,
            ..
        }) = value.as_ref()
        {
            display_date.is_none() && iso_date.is_none()
        } else {
            true
        }
    }

    pub(super) fn should_skip_serializing_text(text: &Option<MaybeEmptyLocalizedText>) -> bool {
        if let Some(text) = text {
            text.value.is_empty()
        } else {
            true
        }
    }

    pub(super) fn should_skip_serializing_string(text: &Option<String>) -> bool {
        if let Some(text) = text {
            text.is_empty()
        } else {
            true
        }
    }
}

/// Root element
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FictionBook {
    /// This element contains an arbitrary stylesheet that is interpreted by a some processing programs,
    /// e.g. text/css stylesheets can be used by XSLT stylesheets to generate better looking html
    #[serde(default, rename = "stylesheet")]
    pub stylesheets: Vec<Stylesheet>,
    /// Book description
    pub description: Description,
    /// Main content of the book, multiple bodies are used for additional information, like footnotes,
    /// that do not appear in the main book flow. The first body is presented to the reader by default, and content in
    /// the other bodies should be accessible by hyperlinks. Name attribute should describe the meaning of this body,
    /// this is optional for the main body.
    #[serde(rename = "body")]
    pub bodies: Vec<Body>,
    /// Any binary data that is required for the presentation of this book in base64 format. Currently
    /// only images are used.
    #[serde(default, rename = "binary")]
    pub binaries: Vec<Binary>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Stylesheet {
    #[serde(rename = "@type")]
    pub kind: String,
    #[serde(rename = "$text")]
    pub content: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Description {
    /// Generic information about the book
    #[serde(rename = "title-info")]
    pub title_info: TitleInfo,
    /// Generic information about the original book (for translations)
    #[serde(rename = "src-title-info", skip_serializing_if = "Option::is_none")]
    pub src_title_info: Option<TitleInfo>,
    /// Information about this particular (xml) document
    #[serde(rename = "document-info", skip_serializing_if = "Option::is_none")]
    pub document_info: Option<DocumentInfo>,
    /// Information about some paper/outher published document, that was used as a source of
    /// this xml document
    #[serde(rename = "publish-info", skip_serializing_if = "Option::is_none")]
    pub publish_info: Option<PublishInfo>,
    /// Any other information about the book/document that didnt fit in the above groups
    #[serde(default, rename = "custom-info")]
    pub custom_info: Vec<CustomInfo>,
    /// Describes, how the document should be presented to end-user, what parts are free, what
    /// parts should be sold and what price should be used
    #[serde(default)]
    pub output: Vec<ShareInstruction>,
}

/// Book (as a book opposite a document) description
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TitleInfo {
    /// Genre of this book, with the optional match percentage
    #[serde(rename = "genre", default = "defaults::genres")]
    pub genres: Vec<GenreWithMatch>,
    /// Author(s) of this book
    #[serde(default, rename = "author")]
    pub authors: Vec<Author>,
    /// Book title
    #[serde(rename = "book-title")]
    pub book_title: MaybeEmptyLocalizedText,
    /// Annotation for this book
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotation>,
    /// Any keywords for this book, intended for use in search engines
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keywords: Option<LocalizedText>,
    /// Date this book was written, can be not exact, e.g. 1863-1867. If an optional attribute is present,
    /// then it should contain some computer-readable date from the interval for use by search and indexingengines
    #[serde(skip_serializing_if = "defaults::should_skip_serializing_date")]
    pub date: Option<Date>,
    /// Any cover page items, currently only images
    #[serde(rename = "coverpage", skip_serializing_if = "Option::is_none")]
    pub cover_page: Option<Covers>,
    /// Book's language
    #[serde(default)]
    pub lang: String,
    /// Book's source language if this is a translation
    #[serde(rename = "src-lang", skip_serializing_if = "Option::is_none")]
    pub src_lang: Option<String>,
    /// Translators if this is a translation
    #[serde(default, rename = "translator")]
    pub translators: Vec<Author>,
    /// Any sequences this book might be part of
    #[serde(default, rename = "sequence")]
    pub sequences: Vec<Sequence>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct DocumentInfo {
    /// Author(s) of this particular document
    #[serde(default, rename = "author")]
    pub authors: Vec<Author>,
    /// Any software used in preparation of this document, in free format
    #[serde(
        rename = "program-used",
        skip_serializing_if = "defaults::should_skip_serializing_text"
    )]
    pub program_used: Option<MaybeEmptyLocalizedText>,
    /// Date this document was created, same guidelines as in the &lt;title-info&gt;
    /// section apply
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<Date>,
    /// Source URL if this document is a conversion of some other (online)
    /// document
    #[serde(default, rename = "src-url")]
    pub src_urls: Vec<String>,
    /// Author of the original (online) document, if this is a conversion
    #[serde(
        rename = "src-ocr",
        skip_serializing_if = "defaults::should_skip_serializing_text"
    )]
    pub src_ocr: Option<MaybeEmptyLocalizedText>,
    /// This is a unique identifier for a document. this must not change
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Document version, in free format, should be incremented if the document is
    /// changed and re-released to the public
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<f64>,
    /// Short description for all changes made to this document, like "Added
    /// missing chapter 6", in free form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Annotation>,
    /// Owner of the fb2 document copyrights
    #[serde(default, rename = "publisher")]
    pub publishers: Vec<Author>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PublishInfo {
    /// Original (paper) book name
    #[serde(rename = "book-name", skip_serializing_if = "Option::is_none")]
    pub book_name: Option<LocalizedText>,
    /// Original (paper) book publisher
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher: Option<LocalizedText>,
    /// City where the original (paper) book was published
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<LocalizedText>,
    /// Year of the original (paper) publication
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isbn: Option<LocalizedText>,
    #[serde(default, rename = "sequence")]
    pub sequences: Vec<Sequence>,
}

/// Any other information about the book/document that didnt fit in the above groups
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct CustomInfo {
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(default, rename = "@info-type")]
    pub info_type: String,
    #[serde(default, rename = "$text")]
    pub content: String,
}

/// In-document instruction for generating output free and payed documents
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct ShareInstruction {
    #[serde(rename = "@mode")]
    pub mode: ShareMode,
    #[serde(rename = "@include-all")]
    pub include_all: DocGenerationInstruction,
    #[serde(rename = "@price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "@currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<ShareInstructionElement>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ShareInstructionElement {
    #[serde(rename = "part")]
    Part(PartShareInstruction),
    #[serde(rename = "output-document-class")]
    OutputDocumentClass(OutputDocumentClass),
}

/// Modes for document sharing (free|paid for now)
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum ShareMode {
    #[serde(rename = "free")]
    Free,
    #[serde(rename = "paid")]
    Paid,
}

/// Selector for output documents. Defines, which rule to apply to any specific output documents
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct OutputDocumentClass {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@create", skip_serializing_if = "Option::is_none")]
    pub create: Option<DocGenerationInstruction>,
    #[serde(rename = "@price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(default, rename = "part")]
    pub parts: Vec<PartShareInstruction>,
}

/// Pointer to specific document section, explaining how to deal with it
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct PartShareInstruction {
    #[serde(
        rename = "@type",
        default = "defaults::link_type",
        skip_serializing_if = "defaults::is_default_link_type"
    )]
    pub kind: String,
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@include")]
    pub include: DocGenerationInstruction,
}

/// List of instructions to process sections (allow|deny|require)
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum DocGenerationInstruction {
    #[serde(rename = "require")]
    Require,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "deny")]
    Deny,
}

/// Main content of the book, multiple bodies are used for additional information, like footnotes, that do not
/// appear in the main book flow (extended from this class). The first body is presented to the reader by default, and content
/// in the other bodies should be accessible by hyperlinks.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Body {
    /// Body name, used for footnotes.
    /// According to the schema, should have no whitespaces.
    #[serde(rename = "@name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    /// Image to be displayed at the top of this section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// A fancy title for the entire book, should be used if the simple text version in &lt;description&gt; is
    /// not adequate, e.g. the book title has multiple paragraphs and/or character styles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    /// Epigraph(s) for the entire book, if any
    #[serde(default, rename = "epigraph")]
    pub epigraphs: Vec<Epigraph>,
    /// Sometimes bodies have no sections
    /// For example, a "notes" body can be generated by a problem
    /// even when there are no actual notes in a particular book
    #[serde(default, rename = "section")]
    pub sections: Vec<Section>,
}

/// Book sequences
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Sequence {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@number", skip_serializing_if = "Option::is_none")]
    pub number: Option<SequenceNumber>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(default, rename = "sequence")]
    pub sequences: Vec<Sequence>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "SequenceNumberInternal")]
pub struct SequenceNumber(pub i32);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct SequenceNumberInternal(String);

impl TryFrom<SequenceNumberInternal> for SequenceNumber {
    type Error = ParseIntError;

    fn try_from(
        SequenceNumberInternal(value): SequenceNumberInternal,
    ) -> Result<Self, Self::Error> {
        Ok(SequenceNumber(value.trim().parse::<i32>()?))
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Covers {
    #[serde(rename = "image")]
    pub images: Vec<InlineImage>,
}

/// Genre of this book, with the optional match percentage
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct GenreWithMatch {
    /// 100 unless a different percentage is specified
    #[serde(
        rename = "@match",
        default = "defaults::genre_match",
        skip_serializing_if = "defaults::is_default_genre_match"
    )]
    pub match_percentage: i32,
    #[serde(default, rename = "$text")]
    pub value: Genre,
}

/// Information about a single author
#[derive(Debug, PartialEq, Deserialize)]
#[serde(from = "AuthorInternal")]
pub enum Author {
    Verbose(VerboseAuthorDetails),
    Anonymous(AnonymousAuthorDetails),
}

impl Serialize for Author {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Author::Verbose(v) => v.serialize(serializer),
            Author::Anonymous(a) => a.serialize(serializer),
        }
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub struct VerboseAuthorDetails {
    #[serde(rename = "first-name")]
    pub first_name: MaybeEmptyLocalizedText,
    #[serde(
        rename = "middle-name",
        skip_serializing_if = "defaults::should_skip_serializing_text"
    )]
    pub middle_name: Option<MaybeEmptyLocalizedText>,
    #[serde(rename = "last-name")]
    pub last_name: MaybeEmptyLocalizedText,
    #[serde(skip_serializing_if = "defaults::should_skip_serializing_text")]
    pub nickname: Option<MaybeEmptyLocalizedText>,
    #[serde(rename = "home-page")]
    pub home_pages: Vec<String>,
    #[serde(rename = "email")]
    pub emails: Vec<String>,
    #[serde(
        rename = "id",
        skip_serializing_if = "defaults::should_skip_serializing_string"
    )]
    pub id: Option<String>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct AnonymousAuthorDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<MaybeEmptyLocalizedText>,
    #[serde(rename = "home-page")]
    pub home_pages: Vec<String>,
    #[serde(rename = "email")]
    pub emails: Vec<String>,
    #[serde(
        rename = "id",
        skip_serializing_if = "defaults::should_skip_serializing_string"
    )]
    pub id: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct AuthorInternal {
    #[serde(rename = "first-name")]
    first_name: Option<MaybeEmptyLocalizedText>,
    #[serde(rename = "middle-name")]
    middle_name: Option<MaybeEmptyLocalizedText>,
    #[serde(rename = "last-name")]
    last_name: Option<MaybeEmptyLocalizedText>,
    nickname: Option<MaybeEmptyLocalizedText>,
    #[serde(default, rename = "home-page")]
    home_pages: Vec<String>,
    #[serde(default, rename = "email")]
    emails: Vec<String>,
    #[serde(rename = "id")]
    id: Option<String>,
}

impl From<AuthorInternal> for Author {
    fn from(
        AuthorInternal {
            first_name,
            middle_name,
            last_name,
            nickname,
            home_pages,
            emails,
            id,
        }: AuthorInternal,
    ) -> Self {
        let verbose = first_name.is_some() || middle_name.is_some() || last_name.is_some();

        if verbose {
            Author::Verbose(VerboseAuthorDetails {
                first_name: first_name.unwrap_or_else(|| MaybeEmptyLocalizedText {
                    lang: None,
                    value: String::new(),
                }),
                middle_name,
                last_name: last_name.unwrap_or_else(|| MaybeEmptyLocalizedText {
                    lang: None,
                    value: String::new(),
                }),
                nickname,
                home_pages,
                emails,
                id,
            })
        } else {
            Author::Anonymous(AnonymousAuthorDetails {
                nickname,
                home_pages,
                emails,
                id,
            })
        }
    }
}

/// Any binary data that is required for the presentation of this book in base64 format. Currently
/// only images are used.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Binary {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@content-type")]
    pub content_type: String,
    #[serde(rename = "$text")]
    pub content: String,
}

/// A basic block of a book, can contain more child sections or textual content
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(try_from = "SectionInternal")]
pub struct Section {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(flatten, rename = "$value", skip_serializing_if = "Option::is_none")]
    pub content: Option<SectionContent>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct SectionContent {
    /// Section's title
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    /// Epigraph(s) for this section
    #[serde(rename = "epigraph")]
    pub epigraphs: Vec<Epigraph>,
    /// Image to be displayed at the top of this section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<Image>,
    /// Annotation for this section, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<Annotation>,
    #[serde(rename = "$value")]
    pub content: Option<(FirstSectionPart, Vec<RestSectionPart>)>,
    #[serde(rename = "section")]
    pub sections: Vec<Section>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum FirstSectionPart {
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "poem")]
    Poem(Poem),
    #[serde(rename = "subtitle")]
    Subtitle(Paragraph),
    #[serde(rename = "cite")]
    Cite(Cite),
    #[serde(rename = "table")]
    Table(Table),
    #[serde(rename = "empty-line")]
    EmptyLine,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum RestSectionPart {
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "poem")]
    Poem(Poem),
    #[serde(rename = "subtitle")]
    Subtitle(Paragraph),
    #[serde(rename = "cite")]
    Cite(Cite),
    #[serde(rename = "table")]
    Table(Table),
    #[serde(rename = "image")]
    Image(Image),
    #[serde(rename = "empty-line")]
    EmptyLine,
}

/// A basic block of a book, can contain more child sections or textual content
#[derive(Debug, PartialEq, Deserialize)]
struct SectionInternal {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@lang")]
    lang: Option<LanguageTag>,
    #[serde(default, rename = "$value")]
    elements: Vec<SectionChoice>,
}

#[derive(Debug, PartialEq, Deserialize)]
enum SectionChoice {
    #[serde(rename = "title")]
    Title(Title),
    #[serde(rename = "epigraph")]
    Epigraph(Epigraph),
    #[serde(rename = "image")]
    Image(Image),
    #[serde(rename = "annotation")]
    Annotation(Annotation),
    #[serde(rename = "section")]
    Section(Section),
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "poem")]
    Poem(Poem),
    #[serde(rename = "subtitle")]
    Subtitle(Paragraph),
    #[serde(rename = "cite")]
    Cite(Cite),
    #[serde(rename = "table")]
    Table(Table),
    #[serde(rename = "empty-line")]
    EmptyLine,
    // will be converted to Paragraph if occurs
    // some real FB2 files have text where it is prohibited
    // so trying to fix those files without failing parsing
    #[serde(rename = "$text")]
    Text(String),
}

impl TryFrom<SectionInternal> for Section {
    type Error = String;

    fn try_from(
        SectionInternal { id, lang, elements }: SectionInternal,
    ) -> Result<Self, Self::Error> {
        if elements.is_empty() {
            return Ok(Section {
                id,
                lang,
                content: None,
            });
        }
        let mut iter = elements.into_iter();
        let mut element = iter.next();
        let title = if let Some(SectionChoice::Title(t)) = element {
            element = iter.next();
            Some(t)
        } else {
            None
        };
        let mut epigraphs = vec![];
        while let Some(SectionChoice::Epigraph(e)) = element {
            epigraphs.push(e);
            element = iter.next();
        }
        let mut image = if let Some(SectionChoice::Image(i)) = element {
            element = iter.next();
            Some(i)
        } else {
            None
        };
        let annotation = if let Some(SectionChoice::Annotation(a)) = element {
            element = iter.next();
            Some(a)
        } else {
            None
        };
        let mut sections = Vec::new();
        let mut first_part = None;
        let mut rest_parts = Vec::new();

        if let Some(element) = element {
            match element {
                SectionChoice::Title(t) => {
                    for element in t.elements {
                        match element {
                            TitleElement::Paragraph(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Paragraph(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Paragraph(p));
                                }
                            }
                            TitleElement::EmptyLine => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::EmptyLine);
                                } else {
                                    rest_parts.push(RestSectionPart::EmptyLine);
                                }
                            }
                        }
                    }
                }
                SectionChoice::Epigraph(e) => {
                    for element in e.elements {
                        match element {
                            EpigraphElement::Paragraph(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Paragraph(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Paragraph(p));
                                }
                            }
                            EpigraphElement::Poem(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Poem(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Poem(p));
                                }
                            }
                            EpigraphElement::Cite(c) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Cite(c));
                                } else {
                                    rest_parts.push(RestSectionPart::Cite(c));
                                }
                            }
                            EpigraphElement::EmptyLine => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::EmptyLine);
                                } else {
                                    rest_parts.push(RestSectionPart::EmptyLine);
                                }
                            }
                        }
                    }
                }
                SectionChoice::Image(i) => {
                    if image.is_some() {
                        return Err("the first section content element must not be an image, a section image should be declared once and right after the section title and epigraphs".to_string());
                    } else {
                        image = Some(i);
                    }
                }
                SectionChoice::Annotation(a) => {
                    for element in a.elements {
                        match element {
                            AnnotationElement::Paragraph(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Paragraph(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Paragraph(p));
                                }
                            }
                            AnnotationElement::Poem(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Poem(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Poem(p));
                                }
                            }
                            AnnotationElement::Cite(c) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Cite(c));
                                } else {
                                    rest_parts.push(RestSectionPart::Cite(c));
                                }
                            }
                            AnnotationElement::Subtitle(s) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Subtitle(s));
                                } else {
                                    rest_parts.push(RestSectionPart::Subtitle(s));
                                }
                            }
                            AnnotationElement::Table(t) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Table(t));
                                } else {
                                    rest_parts.push(RestSectionPart::Table(t));
                                }
                            }
                            AnnotationElement::EmptyLine => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::EmptyLine);
                                } else {
                                    rest_parts.push(RestSectionPart::EmptyLine);
                                }
                            }
                        }
                    }
                }
                SectionChoice::Section(s) => sections.push(s),
                SectionChoice::Paragraph(p) => first_part = Some(FirstSectionPart::Paragraph(p)),
                SectionChoice::Poem(p) => first_part = Some(FirstSectionPart::Poem(p)),
                SectionChoice::Subtitle(s) => first_part = Some(FirstSectionPart::Subtitle(s)),
                SectionChoice::Cite(c) => first_part = Some(FirstSectionPart::Cite(c)),
                SectionChoice::Table(t) => first_part = Some(FirstSectionPart::Table(t)),
                SectionChoice::EmptyLine => first_part = Some(FirstSectionPart::EmptyLine),
                // trying to fix invalid FB2 without losing information
                SectionChoice::Text(text) => {
                    first_part = Some(FirstSectionPart::Paragraph(Paragraph {
                        id: None,
                        lang: None,
                        style: None,
                        elements: vec![StyleElement::Text(text)],
                    }))
                }
            }
        }

        for element in iter {
            match element {
                SectionChoice::Title(t) => {
                    for element in t.elements {
                        match element {
                            TitleElement::Paragraph(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Paragraph(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Paragraph(p));
                                }
                            }
                            TitleElement::EmptyLine => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::EmptyLine);
                                } else {
                                    rest_parts.push(RestSectionPart::EmptyLine);
                                }
                            }
                        }
                    }
                }
                SectionChoice::Epigraph(e) => {
                    for element in e.elements {
                        match element {
                            EpigraphElement::Paragraph(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Paragraph(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Paragraph(p));
                                }
                            }
                            EpigraphElement::Poem(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Poem(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Poem(p));
                                }
                            }
                            EpigraphElement::Cite(c) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Cite(c));
                                } else {
                                    rest_parts.push(RestSectionPart::Cite(c));
                                }
                            }
                            EpigraphElement::EmptyLine => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::EmptyLine);
                                } else {
                                    rest_parts.push(RestSectionPart::EmptyLine);
                                }
                            }
                        }
                    }
                }
                SectionChoice::Image(i) => {
                    if first_part.is_none() {
                        if image.is_none() {
                            image = Some(i);
                        } else {
                            return Err("the first section content element must not be an image, a section image should be declared once and right after the section title and epigraphs".to_string());
                        }
                    } else {
                        rest_parts.push(RestSectionPart::Image(i));
                    }
                }
                SectionChoice::Annotation(a) => {
                    for element in a.elements {
                        match element {
                            AnnotationElement::Paragraph(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Paragraph(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Paragraph(p));
                                }
                            }
                            AnnotationElement::Poem(p) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Poem(p));
                                } else {
                                    rest_parts.push(RestSectionPart::Poem(p));
                                }
                            }
                            AnnotationElement::Cite(c) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Cite(c));
                                } else {
                                    rest_parts.push(RestSectionPart::Cite(c));
                                }
                            }
                            AnnotationElement::Subtitle(s) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Subtitle(s));
                                } else {
                                    rest_parts.push(RestSectionPart::Subtitle(s));
                                }
                            }
                            AnnotationElement::Table(t) => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::Table(t));
                                } else {
                                    rest_parts.push(RestSectionPart::Table(t));
                                }
                            }
                            AnnotationElement::EmptyLine => {
                                if first_part.is_none() {
                                    first_part = Some(FirstSectionPart::EmptyLine);
                                } else {
                                    rest_parts.push(RestSectionPart::EmptyLine);
                                }
                            }
                        }
                    }
                }
                SectionChoice::Section(s) => sections.push(s),
                SectionChoice::Paragraph(p) => {
                    if first_part.is_none() {
                        first_part = Some(FirstSectionPart::Paragraph(p));
                    } else {
                        rest_parts.push(RestSectionPart::Paragraph(p));
                    }
                }
                SectionChoice::Poem(p) => {
                    if first_part.is_none() {
                        first_part = Some(FirstSectionPart::Poem(p));
                    } else {
                        rest_parts.push(RestSectionPart::Poem(p));
                    }
                }
                SectionChoice::Subtitle(s) => {
                    if first_part.is_none() {
                        first_part = Some(FirstSectionPart::Subtitle(s));
                    } else {
                        rest_parts.push(RestSectionPart::Subtitle(s));
                    }
                }
                SectionChoice::Cite(c) => {
                    if first_part.is_none() {
                        first_part = Some(FirstSectionPart::Cite(c));
                    } else {
                        rest_parts.push(RestSectionPart::Cite(c));
                    }
                }
                SectionChoice::Table(t) => {
                    if first_part.is_none() {
                        first_part = Some(FirstSectionPart::Table(t));
                    } else {
                        rest_parts.push(RestSectionPart::Table(t));
                    }
                }
                SectionChoice::EmptyLine => {
                    if first_part.is_none() {
                        first_part = Some(FirstSectionPart::EmptyLine);
                    } else {
                        rest_parts.push(RestSectionPart::EmptyLine);
                    }
                }
                SectionChoice::Text(text) => {
                    if first_part.is_none() {
                        first_part = Some(FirstSectionPart::Paragraph(Paragraph {
                            id: None,
                            lang: None,
                            style: None,
                            elements: vec![StyleElement::Text(text)],
                        }));
                    } else {
                        rest_parts.push(RestSectionPart::Paragraph(Paragraph {
                            id: None,
                            lang: None,
                            style: None,
                            elements: vec![StyleElement::Text(text)],
                        }));
                    }
                }
            }
        }

        Ok(Section {
            id,
            lang,
            content: Some(SectionContent {
                title,
                epigraphs,
                image,
                annotation,
                content: first_part.map(|first_part| (first_part, rest_parts)),
                sections,
            }),
        })
    }
}

/// A cut-down version of section used in annotations
#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(from = "AnnotationInternal")]
pub struct Annotation {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(rename = "$value")]
    pub elements: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum AnnotationElement {
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "poem")]
    Poem(Poem),
    #[serde(rename = "cite")]
    Cite(Cite),
    #[serde(rename = "subtitle")]
    Subtitle(Paragraph),
    #[serde(rename = "table")]
    Table(Table),
    #[serde(rename = "empty-line")]
    EmptyLine,
}

/// A cut-down version of section used in annotations
#[derive(Debug, PartialEq, Deserialize)]
struct AnnotationInternal {
    #[serde(rename = "@id")]
    id: Option<String>,
    #[serde(rename = "@lang")]
    lang: Option<LanguageTag>,
    #[serde(default, rename = "$value")]
    elements: Vec<AnnotationChoice>,
}

#[derive(Debug, PartialEq, Deserialize)]
enum AnnotationChoice {
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "poem")]
    Poem(Poem),
    #[serde(rename = "cite")]
    Cite(Cite),
    #[serde(rename = "subtitle")]
    Subtitle(Paragraph),
    #[serde(rename = "table")]
    Table(Table),
    #[serde(rename = "empty-line")]
    EmptyLine,
    #[serde(rename = "$text")]
    Text(String),
}

impl From<AnnotationInternal> for Annotation {
    fn from(AnnotationInternal { id, lang, elements }: AnnotationInternal) -> Self {
        let elements = elements
            .into_iter()
            .map(|element| match element {
                AnnotationChoice::Paragraph(p) => AnnotationElement::Paragraph(p),
                AnnotationChoice::Poem(p) => AnnotationElement::Poem(p),
                AnnotationChoice::Cite(c) => AnnotationElement::Cite(c),
                AnnotationChoice::Subtitle(s) => AnnotationElement::Subtitle(s),
                AnnotationChoice::Table(t) => AnnotationElement::Table(t),
                AnnotationChoice::EmptyLine => AnnotationElement::EmptyLine,
                AnnotationChoice::Text(text) => AnnotationElement::Paragraph(Paragraph {
                    id: None,
                    lang: None,
                    style: None,
                    elements: vec![StyleElement::Text(text)],
                }),
            })
            .collect();
        Annotation { id, lang, elements }
    }
}

/// An epigraph
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Epigraph {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<EpigraphElement>,
    #[serde(default, rename = "text-author")]
    pub text_authors: Vec<Paragraph>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum EpigraphElement {
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "poem")]
    Poem(Poem),
    #[serde(rename = "cite")]
    Cite(Cite),
    #[serde(rename = "empty-line")]
    EmptyLine,
}

/// A citation with an optional citation author at the end
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Cite {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<CiteElement>,
    #[serde(default, rename = "text-author")]
    pub text_authors: Vec<Paragraph>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum CiteElement {
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "poem")]
    Poem(Poem),
    #[serde(rename = "subtitle")]
    Subtitle(Paragraph),
    #[serde(rename = "table")]
    Table(Table),
    #[serde(rename = "empty-line")]
    EmptyLine,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Poem {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(default, rename = "epigraph")]
    pub epigraphs: Vec<Epigraph>,
    #[serde(rename = "$value")]
    pub stanzas: Vec<PoemStanza>,
    #[serde(default, rename = "text-author")]
    pub text_authors: Vec<Paragraph>,
    /// Date this poem was written.
    #[serde(skip_serializing_if = "defaults::should_skip_serializing_date")]
    pub date: Option<Date>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum PoemStanza {
    #[serde(rename = "subtitle")]
    Subtitle(Paragraph),
    #[serde(rename = "stanza")]
    Stanza(Stanza),
}

/// Each poem should have at least one stanza. Stanzas are usually separated with empty lines by user
/// agents.
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Stanza {
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<Title>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<Paragraph>,
    #[serde(default, rename = "v")]
    pub lines: Vec<Paragraph>,
}

/// A title, used in sections, poems and body elements
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Title {
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<TitleElement>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TitleElement {
    #[serde(rename = "p")]
    Paragraph(Paragraph),
    #[serde(rename = "empty-line")]
    EmptyLine,
}

/// A basic paragraph, may include simple formatting inside
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Paragraph {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<StyleElement>,
}

/// Basic html-like tables
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Table {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "tr")]
    pub rows: Vec<TableRow>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TableRow {
    #[serde(
        default,
        rename = "@align",
        skip_serializing_if = "defaults::is_default_horizontal_align"
    )]
    pub align: HorizontalAlign,
    #[serde(rename = "$value")]
    pub cells: Vec<TableCellElement>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum TableCellElement {
    #[serde(rename = "th")]
    Head(TableCell),
    #[serde(rename = "td")]
    Data(TableCell),
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct TableCell {
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(rename = "@style", skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "@colspan", skip_serializing_if = "Option::is_none")]
    pub column_span: Option<i32>,
    #[serde(rename = "@rowspan", skip_serializing_if = "Option::is_none")]
    pub row_span: Option<i32>,
    #[serde(
        default,
        rename = "@align",
        skip_serializing_if = "defaults::is_default_horizontal_align"
    )]
    pub horizontal_align: HorizontalAlign,
    #[serde(
        default,
        rename = "@valign",
        skip_serializing_if = "defaults::is_default_vertical_align"
    )]
    pub vertical_align: VerticalAlign,
    #[serde(default, rename = "$value")]
    pub elements: Vec<StyleElement>,
}

/// Align for table cells
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum HorizontalAlign {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "center")]
    Center,
}

impl Default for HorizontalAlign {
    fn default() -> Self {
        HorizontalAlign::Left
    }
}

/// Align for table cells
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub enum VerticalAlign {
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "middle")]
    Middle,
    #[serde(rename = "bottom")]
    Bottom,
}

impl Default for VerticalAlign {
    fn default() -> Self {
        VerticalAlign::Top
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct NamedStyle {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<StyleElement>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Style {
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<StyleElement>,
}

/// Markup
#[derive(Debug, PartialEq, Deserialize)]
pub enum StyleElement {
    #[serde(rename = "strong")]
    Strong(Style),
    #[serde(rename = "emphasis")]
    Emphasis(Style),
    #[serde(rename = "style")]
    Style(NamedStyle),
    #[serde(rename = "a")]
    Link(Link),
    #[serde(rename = "strikethrough")]
    Strikethrough(Style),
    #[serde(rename = "sub")]
    Subscript(Style),
    #[serde(rename = "sup")]
    Superscript(Style),
    #[serde(rename = "code")]
    Code(Style),
    #[serde(rename = "image")]
    Image(InlineImage),
    #[serde(rename = "$text")]
    Text(String),
}

impl Serialize for StyleElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use StyleElement::*;
        match self {
            Strong(style) => {
                let mut state =
                    serializer.serialize_tuple_variant("StyleElement", 0, "strong", 1)?;
                state.serialize_field(style)?;
                state.end()
            }
            Emphasis(style) => {
                let mut state =
                    serializer.serialize_tuple_variant("StyleElement", 1, "emphasis", 1)?;
                state.serialize_field(style)?;
                state.end()
            }
            Style(style) => {
                let mut state =
                    serializer.serialize_tuple_variant("StyleElement", 2, "style", 1)?;
                state.serialize_field(style)?;
                state.end()
            }
            Link(link) => {
                let mut state = serializer.serialize_tuple_variant("StyleElement", 3, "a", 1)?;
                state.serialize_field(link)?;
                state.end()
            }
            Strikethrough(style) => {
                let mut state =
                    serializer.serialize_tuple_variant("StyleElement", 4, "strikethrough", 1)?;
                state.serialize_field(style)?;
                state.end()
            }
            Subscript(style) => {
                let mut state = serializer.serialize_tuple_variant("StyleElement", 5, "sub", 1)?;
                state.serialize_field(style)?;
                state.end()
            }
            Superscript(style) => {
                let mut state = serializer.serialize_tuple_variant("StyleElement", 6, "sup", 1)?;
                state.serialize_field(style)?;
                state.end()
            }
            Code(style) => {
                let mut state = serializer.serialize_tuple_variant("StyleElement", 7, "code", 1)?;
                state.serialize_field(style)?;
                state.end()
            }
            Image(image) => {
                let mut state =
                    serializer.serialize_tuple_variant("StyleElement", 8, "image", 1)?;
                state.serialize_field(image)?;
                state.end()
            }
            Text(text) => text.serialize(serializer),
        }
    }
}

/// Generic hyperlinks. Cannot be nested. Footnotes should be implemented by links referring to additional bodies
/// in the same document
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Link {
    #[serde(rename = "@href")]
    pub href: String,
    #[serde(rename = "@type", skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(default, rename = "$value")]
    pub elements: Vec<StyleLinkElement>,
}

/// Markup
#[derive(Debug, PartialEq, Deserialize)]
pub enum StyleLinkElement {
    #[serde(rename = "strong")]
    Strong {
        #[serde(rename = "$value")]
        elements: Vec<StyleLinkElement>,
    },
    #[serde(rename = "emphasis")]
    Emphasis {
        #[serde(rename = "$value")]
        elements: Vec<StyleLinkElement>,
    },
    #[serde(rename = "style")]
    Style {
        #[serde(rename = "$value")]
        elements: Vec<StyleLinkElement>,
    },
    #[serde(rename = "strikethrough")]
    Strikethrough {
        #[serde(rename = "$value")]
        elements: Vec<StyleLinkElement>,
    },
    #[serde(rename = "sub")]
    Subscript {
        #[serde(rename = "$value")]
        elements: Vec<StyleLinkElement>,
    },
    #[serde(rename = "sup")]
    Superscript {
        #[serde(rename = "$value")]
        elements: Vec<StyleLinkElement>,
    },
    #[serde(rename = "code")]
    Code {
        #[serde(rename = "$value")]
        elements: Vec<StyleLinkElement>,
    },
    #[serde(rename = "image")]
    Image(InlineImage),
    #[serde(rename = "$text")]
    Text(String),
}

impl Serialize for StyleLinkElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use StyleLinkElement::*;
        match self {
            Strong { elements } => {
                let mut state =
                    serializer.serialize_struct_variant("StyleLinkElement", 0, "strong", 1)?;
                state.serialize_field("$value", elements)?;
                state.end()
            }
            Emphasis { elements } => {
                let mut state =
                    serializer.serialize_struct_variant("StyleLinkElement", 1, "emphasis", 1)?;
                state.serialize_field("$value", elements)?;
                state.end()
            }
            Style { elements } => {
                let mut state =
                    serializer.serialize_struct_variant("StyleLinkElement", 2, "style", 1)?;
                state.serialize_field("$value", elements)?;
                state.end()
            }
            Strikethrough { elements } => {
                let mut state = serializer.serialize_struct_variant(
                    "StyleLinkElement",
                    3,
                    "strikethrough",
                    1,
                )?;
                state.serialize_field("$value", elements)?;
                state.end()
            }
            Subscript { elements } => {
                let mut state =
                    serializer.serialize_struct_variant("StyleLinkElement", 4, "sub", 1)?;
                state.serialize_field("$value", elements)?;
                state.end()
            }
            Superscript { elements } => {
                let mut state =
                    serializer.serialize_struct_variant("StyleLinkElement", 5, "sup", 1)?;
                state.serialize_field("$value", elements)?;
                state.end()
            }
            Code { elements } => {
                let mut state =
                    serializer.serialize_struct_variant("StyleLinkElement", 6, "code", 1)?;
                state.serialize_field("$value", elements)?;
                state.end()
            }
            Image(image) => {
                let mut state =
                    serializer.serialize_tuple_variant("StyleLinkElement", 7, "image", 1)?;
                state.serialize_field(image)?;
                state.end()
            }
            Text(text) => text.serialize(serializer),
        }
    }
}

/// A human readable date, maybe not exact, with an optional computer readable variant
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Date {
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(rename = "@value", skip_serializing_if = "Option::is_none")]
    pub iso_date: Option<chrono::NaiveDate>,
    #[serde(rename = "$text")]
    pub display_date: Option<String>,
}

/// An empty element with an image name as an attribute
#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Image {
    #[serde(
        rename = "@type",
        default = "defaults::link_type",
        skip_serializing_if = "defaults::is_default_link_type"
    )]
    pub kind: String,
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "@alt", skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
    #[serde(rename = "@title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "@id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct InlineImage {
    #[serde(
        rename = "@type",
        default = "defaults::link_type",
        skip_serializing_if = "defaults::is_default_link_type"
    )]
    pub kind: String,
    #[serde(rename = "@href", skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(rename = "@alt", skip_serializing_if = "Option::is_none")]
    pub alt: Option<String>,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct LocalizedText {
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    #[serde(rename = "$text")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct MaybeEmptyLocalizedText {
    #[serde(rename = "@lang", skip_serializing_if = "Option::is_none")]
    pub lang: Option<LanguageTag>,
    /// Text fields sometimes have empty value to bypass field requirement
    #[serde(default, rename = "$text")]
    pub value: String,
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Genre {
    Accounting,
    AdvAnimal,
    AdvGeo,
    AdvHistory,
    AdvMaritime,
    AdvWestern,
    Adventure,
    Antique,
    AntiqueAnt,
    AntiqueEast,
    AntiqueEuropean,
    AntiqueMyths,
    AntiqueRussian,
    AphorismQuote,
    ArchitectureBook,
    AutoRegulations,
    Banking,
    BeginningAuthors,
    ChildAdv,
    ChildDet,
    ChildEducation,
    ChildProse,
    ChildSf,
    ChildTale,
    ChildVerse,
    Children,
    CinemaTheatre,
    CityFantasy,
    CompDb,
    CompHard,
    CompOsnet,
    CompProgramming,
    CompSoft,
    CompWww,
    Computers,
    Design,
    DetAction,
    DetClassic,
    DetCrime,
    DetEspionage,
    DetHard,
    DetHistory,
    DetIrony,
    DetPolice,
    DetPolitical,
    Detective,
    DragonFantasy,
    Dramaturgy,
    Economics,
    Essays,
    FantasyFight,
    ForeignAction,
    ForeignAdventure,
    ForeignAntique,
    ForeignBusiness,
    ForeignChildren,
    ForeignComp,
    ForeignContemporary,
    ForeignContemporaryLit,
    ForeignDesc,
    ForeignDetective,
    ForeignDramaturgy,
    ForeignEdu,
    ForeignFantasy,
    ForeignHome,
    ForeignHumor,
    ForeignLanguage,
    ForeignLove,
    ForeignNovel,
    ForeignOther,
    ForeignPoetry,
    ForeignProse,
    ForeignPsychology,
    ForeignPublicism,
    ForeignReligion,
    ForeignSf,
    GeoGuides,
    GeographyBook,
    GlobalEconomy,
    HistoricalFantasy,
    Home,
    HomeCooking,
    HomeCrafts,
    HomeDiy,
    HomeEntertain,
    HomeGarden,
    HomeHealth,
    HomePets,
    HomeSex,
    HomeSport,
    Humor,
    HumorAnecdote,
    HumorFantasy,
    HumorProse,
    HumorVerse,
    Industries,
    JobHunting,
    Literature18,
    Literature19,
    Literature20,
    LoveContemporary,
    LoveDetective,
    LoveErotica,
    LoveFantasy,
    LoveHistory,
    LoveSf,
    LoveShort,
    MagicianBook,
    Management,
    Marketing,
    MilitarySpecial,
    MusicDancing,
    Narrative,
    Newspapers,
    NonfBiography,
    NonfCriticism,
    NonfPublicism,
    Nonfiction,
    OrgBehavior,
    PaperWork,
    PedagogyBook,
    Periodic,
    PersonalFinance,
    Poetry,
    Popadanec,
    PopularBusiness,
    ProseClassic,
    ProseContemporary,
    ProseCounter,
    ProseHistory,
    ProseMilitary,
    ProseRusClassic,
    ProseSuClassics,
    PsyAlassic,
    PsyChilds,
    PsyGeneric,
    PsyPersonal,
    PsySexAndFamily,
    PsySocial,
    PsyTheraphy,
    RealEstate,
    RefDict,
    RefEncyc,
    RefGuide,
    RefRef,
    Reference,
    Religion,
    ReligionEsoterics,
    ReligionRel,
    ReligionSelf,
    RussianContemporary,
    RussianFantasy,
    SciBiology,
    SciChem,
    SciCulture,
    SciHistory,
    SciJuris,
    SciLinguistic,
    SciMath,
    SciMedicine,
    SciPhilosophy,
    SciPhys,
    SciPsychology,
    SciPolitics,
    SciReligion,
    SciTech,
    Science,
    Sf,
    SfAction,
    SfCyberpunk,
    SfDetective,
    SfFantasy,
    SfHeroic,
    SfHistory,
    SfHorror,
    SfHumor,
    SfSocial,
    SfSpace,
    ShortStory,
    Sketch,
    SmallBusiness,
    SociologyBook,
    Stock,
    Thriller,
    UpbringingBook,
    VampireBook,
    VisualArts,
    Unrecognised,
}

impl Default for Genre {
    fn default() -> Self {
        Genre::Unrecognised
    }
}
