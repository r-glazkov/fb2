use language_tags::LanguageTag;

pub use error::{Error, ErrorKind};
pub use parser::parse;

mod error;
mod parser;

pub const LINK_TYPE: &str = "simple";

pub(crate) trait FromStrOpt {
    fn from_str_or_none(value: &str) -> Option<Self>
    where
        Self: Sized;
}

/// Root element
#[derive(Debug, PartialEq)]
pub struct FictionBook {
    /// This element contains an arbitrary stylesheet that is interpreted by a some processing programs,
    /// e.g. text/css stylesheets can be used by XSLT stylesheets to generate better looking html
    pub stylesheets: Vec<Stylesheet>,
    /// Book description
    pub description: Description,
    /// Main content of the book, multiple bodies are used for additional information, like footnotes,
    /// that do not appear in the main book flow. The first body is presented to the reader by default, and content in
    /// the other bodies should be accessible by hyperlinks. Name attribute should describe the meaning of this body,
    /// this is optional for the main body.
    pub body: Body,
    /// Multiple bodies are used for additional information, like footnotes or comments,
    /// that do not appear in the main book flow. The first body is presented to the reader by default, and content in
    /// the other bodies should be accessible by hyperlinks. Name attribute should describe the meaning of this body,
    /// this is optional for the main body.
    pub extra_bodies: Vec<Body>,
    /// Any binary data that is required for the presentation of this book in base64 format. Currently
    /// only images are used.
    pub binaries: Vec<Binary>,
}

#[derive(Debug, PartialEq)]
pub struct Stylesheet {
    /// "type" is reserved in Rust
    pub kind: String,
    pub content: String,
}

#[derive(Debug, PartialEq)]
pub struct Description {
    /// Generic information about the book
    pub title_info: TitleInfo,
    /// Generic information about the original book (for translations)
    pub src_title_info: Option<TitleInfo>,
    /// Information about this particular (xml) document
    pub document_info: DocumentInfo,
    /// Information about some paper/outher published document, that was used as a source of
    /// this xml document
    pub publish_info: Option<PublishInfo>,
    /// Any other information about the book/document that didnt fit in the above groups
    pub custom_info: Vec<CustomInfo>,
    /// Describes, how the document should be presented to end-user, what parts are free, what
    /// parts should be sold and what price should be used
    pub output: Vec<ShareInstruction>,
}

/// Book (as a book opposite a document) description
#[derive(Debug, PartialEq)]
pub struct TitleInfo {
    /// Genre of this book, with the optional match percentage
    pub genres: Vec<GenreWithMatch>,
    /// Author(s) of this book
    pub authors: Vec<Author>,
    /// Book title
    pub book_title: LocalizedText,
    /// Annotation for this book
    pub annotation: Option<Annotation>,
    /// Any keywords for this book, intended for use in search engines
    pub keywords: Option<LocalizedText>,
    /// Date this book was written, can be not exact, e.g. 1863-1867. If an optional attribute is present,
    /// then it should contain some computer-readable date from the interval for use by search and indexingengines
    pub date: Option<Date>,
    /// Any cover page items, currently only images
    pub cover_page: Option<CoverPage>,
    /// Book's language
    pub lang: String,
    /// Book's source language if this is a translation
    pub src_lang: Option<String>,
    /// Translators if this is a translation
    pub translators: Vec<Author>,
    /// Any sequences this book might be part of
    pub sequences: Vec<Sequence>,
}

#[derive(Debug, PartialEq)]
pub struct DocumentInfo {
    /// Author(s) of this particular document
    pub authors: Vec<Author>,
    /// Any software used in preparation of this document, in free format
    pub program_used: Option<LocalizedText>,
    /// Date this document was created, same guidelines as in the &lt;title-info&gt;
    /// section apply
    pub date: Date,
    /// Source URL if this document is a conversion of some other (online)
    /// document
    pub src_urls: Vec<String>,
    /// Author of the original (online) document, if this is a conversion
    pub src_ocr: Option<LocalizedText>,
    /// This is a unique identifier for a document. this must not change
    pub id: String,
    /// Document version, in free format, should be incremented if the document is
    /// changed and re-released to the public
    pub version: f64,
    /// Short description for all changes made to this document, like "Added
    /// missing chapter 6", in free form.
    pub history: Option<Annotation>,
    /// Owner of the fb2 document copyrights
    pub publishers: Vec<Author>,
}

#[derive(Debug, PartialEq)]
pub struct PublishInfo {
    /// Original (paper) book name
    pub book_name: Option<LocalizedText>,
    /// Original (paper) book publisher
    pub publisher: Option<LocalizedText>,
    /// City where the original (paper) book was published
    pub city: Option<LocalizedText>,
    /// Year of the original (paper) publication
    pub year: Option<i32>,
    pub isbn: Option<LocalizedText>,
    pub sequences: Vec<Sequence>,
}

/// Any other information about the book/document that didnt fit in the above groups
#[derive(Debug, PartialEq)]
pub struct CustomInfo {
    pub info_type: String,
    pub content: LocalizedText,
}

/// In-document instruction for generating output free and payed documents
#[derive(Debug, PartialEq)]
pub struct ShareInstruction {
    pub mode: ShareMode,
    pub include_all: DocGenerationInstruction,
    pub price: Option<f64>,
    pub currency: Option<String>,
    pub elements: Vec<ShareInstructionElement>,
}

#[derive(Debug, PartialEq)]
pub enum ShareInstructionElement {
    Part(PartShareInstruction),
    OutputDocumentClass(OutputDocumentClass),
}

/// Modes for document sharing (free|paid for now)
#[derive(Debug, PartialEq)]
pub enum ShareMode {
    Free,
    Paid,
}

impl FromStrOpt for ShareMode {
    fn from_str_or_none(value: &str) -> Option<ShareMode> {
        use ShareMode::*;
        Some(match value {
            "free" => Free,
            "paid" => Paid,
            _ => return None,
        })
    }
}

/// Selector for output documents. Defines, which rule to apply to any specific output documents
#[derive(Debug, PartialEq)]
pub struct OutputDocumentClass {
    pub name: String,
    pub create: Option<DocGenerationInstruction>,
    pub price: Option<f64>,
    pub parts: Vec<PartShareInstruction>,
}

/// Pointer to specific document section, explaining how to deal with it
#[derive(Debug, PartialEq)]
pub struct PartShareInstruction {
    /// "type" is reserved in Rust
    pub kind: &'static str,
    pub href: String,
    pub include: DocGenerationInstruction,
}

/// List of instructions to process sections (allow|deny|require)
#[derive(Debug, PartialEq)]
pub enum DocGenerationInstruction {
    Require,
    Allow,
    Deny,
}

impl FromStrOpt for DocGenerationInstruction {
    fn from_str_or_none(value: &str) -> Option<DocGenerationInstruction> {
        use DocGenerationInstruction::*;
        Some(match value {
            "require" => Require,
            "allow" => Allow,
            "deny" => Deny,
            _ => return None,
        })
    }
}

/// Main content of the book, multiple bodies are used for additional information, like footnotes, that do not
/// appear in the main book flow (extended from this class). The first body is presented to the reader by default, and content
/// in the other bodies should be accessible by hyperlinks.
#[derive(Debug, PartialEq)]
pub struct Body {
    /// Body name, used for footnotes.
    /// According to the schema, should have no whitespaces.
    pub name: Option<String>,
    pub lang: Option<LanguageTag>,
    /// Image to be displayed at the top of this section
    pub image: Option<Image>,
    /// A fancy title for the entire book, should be used if the simple text version in &lt;description&gt; is
    /// not adequate, e.g. the book title has multiple paragraphs and/or character styles
    pub title: Option<Title>,
    /// Epigraph(s) for the entire book, if any
    pub epigraphs: Vec<Epigraph>,
    pub sections: Vec<Section>,
}

/// Book sequences
#[derive(Debug, PartialEq)]
pub struct Sequence {
    pub name: String,
    pub number: Option<i32>,
    pub lang: Option<LanguageTag>,
    pub sequences: Vec<Sequence>,
}

#[derive(Debug, PartialEq)]
pub struct CoverPage(pub Vec<InlineImage>);

/// Genre of this book, with the optional match percentage
#[derive(Debug, PartialEq)]
pub struct GenreWithMatch {
    /// 100 unless a different percentage is specified
    pub match_percentage: i32,
    pub value: Genre,
}

/// Information about a single author
#[derive(Debug, PartialEq)]
pub enum Author {
    Verbose(VerboseAuthorDetails),
    Anonymous(AnonymousAuthorDetails),
}

#[derive(Debug, PartialEq)]
pub struct VerboseAuthorDetails {
    pub first_name: LocalizedText,
    pub middle_name: Option<LocalizedText>,
    pub last_name: LocalizedText,
    pub nickname: Option<LocalizedText>,
    pub home_pages: Vec<String>,
    pub emails: Vec<String>,
    pub id: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct AnonymousAuthorDetails {
    pub nickname: LocalizedText,
    pub home_pages: Vec<String>,
    pub emails: Vec<String>,
    pub id: Option<String>,
}

/// Any binary data that is required for the presentation of this book in base64 format. Currently
/// only images are used.
#[derive(Debug, PartialEq)]
pub struct Binary {
    pub id: String,
    pub content_type: String,
    pub content: String,
}

/// A basic block of a book, can contain more child sections or textual content
#[derive(Debug, PartialEq)]
pub struct Section {
    pub id: Option<String>,
    pub lang: Option<LanguageTag>,
    pub content: Option<SectionContent>,
}

#[derive(Debug, PartialEq)]
pub struct SectionContent {
    /// Section's title
    pub title: Option<Title>,
    /// Epigraph(s) for this section
    pub epigraphs: Vec<Epigraph>,
    /// Image to be displayed at the top of this section
    pub image: Option<Image>,
    /// Annotation for this section, if any
    pub annotation: Option<Annotation>,
    pub value: SectionContentValue,
}

#[derive(Debug, PartialEq)]
pub enum SectionContentValue {
    ChildSections(Vec<Section>),
    SectionParts(SectionParts),
}

#[derive(Debug, PartialEq)]
pub struct SectionParts {
    pub first: FirstSectionPart,
    pub rest: Vec<RestSectionPart>,
}

#[derive(Debug, PartialEq)]
pub enum FirstSectionPart {
    Paragraph(Paragraph),
    Poem(Poem),
    Subtitle(Paragraph),
    Cite(Cite),
    Table(Table),
    EmptyLine,
}

#[derive(Debug, PartialEq)]
pub enum RestSectionPart {
    Paragraph(Paragraph),
    Poem(Poem),
    Subtitle(Paragraph),
    Cite(Cite),
    Table(Table),
    Image(Image),
    EmptyLine,
}

/// A cut-down version of section used in annotations
#[derive(Debug, PartialEq)]
pub struct Annotation {
    pub id: Option<String>,
    pub lang: Option<LanguageTag>,
    pub elements: Vec<AnnotationElement>,
}

#[derive(Debug, PartialEq)]
pub enum AnnotationElement {
    Paragraph(Paragraph),
    Poem(Poem),
    Cite(Cite),
    Subtitle(Paragraph),
    Table(Table),
    EmptyLine,
}

/// An epigraph
#[derive(Debug, PartialEq)]
pub struct Epigraph {
    pub id: Option<String>,
    pub elements: Vec<EpigraphElement>,
    pub text_authors: Vec<Paragraph>,
}

#[derive(Debug, PartialEq)]
pub enum EpigraphElement {
    Paragraph(Paragraph),
    Poem(Poem),
    Cite(Cite),
    EmptyLine,
}

/// A citation with an optional citation author at the end
#[derive(Debug, PartialEq)]
pub struct Cite {
    pub id: Option<String>,
    pub lang: Option<LanguageTag>,
    pub elements: Vec<CiteElement>,
    pub text_authors: Vec<Paragraph>,
}

#[derive(Debug, PartialEq)]
pub enum CiteElement {
    Paragraph(Paragraph),
    Poem(Poem),
    Subtitle(Paragraph),
    Table(Table),
    EmptyLine,
}

#[derive(Debug, PartialEq)]
pub struct Poem {
    pub id: Option<String>,
    pub lang: Option<LanguageTag>,
    pub title: Option<Title>,
    pub epigraphs: Vec<Epigraph>,
    pub stanzas: Vec<PoemStanza>,
    pub text_authors: Vec<Paragraph>,
    /// Date this poem was written.
    pub date: Option<Date>,
}

#[derive(Debug, PartialEq)]
pub enum PoemStanza {
    Subtitle(Paragraph),
    Stanza(Stanza),
}

/// Each poem should have at least one stanza. Stanzas are usually separated with empty lines by user
/// agents.
#[derive(Debug, PartialEq)]
pub struct Stanza {
    pub lang: Option<LanguageTag>,
    pub title: Option<Title>,
    pub subtitle: Option<Paragraph>,
    pub lines: Vec<Paragraph>,
}

/// A title, used in sections, poems and body elements
#[derive(Debug, PartialEq)]
pub struct Title {
    pub lang: Option<LanguageTag>,
    pub elements: Vec<TitleElement>,
}

#[derive(Debug, PartialEq)]
pub enum TitleElement {
    Paragraph(Paragraph),
    EmptyLine,
}

/// A basic paragraph, may include simple formatting inside
#[derive(Debug, PartialEq)]
pub struct Paragraph {
    pub id: Option<String>,
    pub style: Option<String>,
    pub content: Style,
}

/// Basic html-like tables
#[derive(Debug, PartialEq)]
pub struct Table {
    pub id: Option<String>,
    pub style: Option<String>,
    pub rows: Vec<TableRow>,
}

#[derive(Debug, PartialEq)]
pub struct TableRow {
    pub align: HorizontalAlign,
    pub cells: Vec<TableCellElement>,
}

#[derive(Debug, PartialEq)]
pub enum TableCellElement {
    Head(TableCell),
    Data(TableCell),
}

#[derive(Debug, PartialEq)]
pub struct TableCell {
    pub id: Option<String>,
    pub style: Option<String>,
    pub column_span: Option<i32>,
    pub row_span: Option<i32>,
    pub horizontal_align: HorizontalAlign,
    pub vertical_align: VerticalAlign,
    pub content: Style,
}

/// Align for table cells
#[derive(Debug, PartialEq)]
pub enum HorizontalAlign {
    Left,
    Right,
    Center,
}

impl FromStrOpt for HorizontalAlign {
    fn from_str_or_none(value: &str) -> Option<HorizontalAlign> {
        use HorizontalAlign::*;
        Some(match value {
            "left" => Left,
            "right" => Right,
            "center" => Center,
            _ => return None,
        })
    }
}

impl Default for HorizontalAlign {
    fn default() -> Self {
        HorizontalAlign::Left
    }
}

/// Align for table cells
#[derive(Debug, PartialEq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

impl FromStrOpt for VerticalAlign {
    fn from_str_or_none(value: &str) -> Option<VerticalAlign> {
        use VerticalAlign::*;
        Some(match value {
            "top" => Top,
            "middle" => Middle,
            "bottom" => Bottom,
            _ => return None,
        })
    }
}

impl Default for VerticalAlign {
    fn default() -> Self {
        VerticalAlign::Top
    }
}

#[derive(Debug, PartialEq)]
pub struct NamedStyle {
    pub name: String,
    pub lang: Option<LanguageTag>,
    pub elements: Vec<StyleElement>,
}

#[derive(Debug, PartialEq)]
pub struct Style {
    pub lang: Option<LanguageTag>,
    pub elements: Vec<StyleElement>,
}

/// Markup
#[derive(Debug, PartialEq)]
pub enum StyleElement {
    Strong(Style),
    Emphasis(Style),
    Style(NamedStyle),
    Link(Link),
    Strikethrough(Style),
    Subscript(Style),
    Superscript(Style),
    Code(Style),
    Image(InlineImage),
    Text(String),
}

/// Generic hyperlinks. Cannot be nested. Footnotes should be implemented by links referring to additional bodies
/// in the same document
#[derive(Debug, PartialEq)]
pub struct Link {
    /// "type" is reserved in Rust
    pub kind: Option<String>,
    pub href: String,
    pub elements: Vec<StyleLinkElement>,
}

/// Markup
#[derive(Debug, PartialEq)]
pub enum StyleLinkElement {
    Strong(Vec<StyleLinkElement>),
    Emphasis(Vec<StyleLinkElement>),
    Style(Vec<StyleLinkElement>),
    Strikethrough(Vec<StyleLinkElement>),
    Subscript(Vec<StyleLinkElement>),
    Superscript(Vec<StyleLinkElement>),
    Code(Vec<StyleLinkElement>),
    Image(InlineImage),
    Text(String),
}

/// A human readable date, maybe not exact, with an optional computer readable variant
#[derive(Debug, PartialEq)]
pub struct Date {
    pub lang: Option<LanguageTag>,
    pub date: Option<chrono::NaiveDate>,
    pub display_date: String,
}

/// An empty element with an image name as an attribute
#[derive(Debug, PartialEq)]
pub struct Image {
    /// "type" is reserved in Rust
    pub kind: &'static str,
    pub href: Option<String>,
    pub alt: Option<String>,
    pub title: Option<String>,
    pub id: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct InlineImage {
    /// "type" is reserved in Rust
    pub kind: &'static str,
    pub href: Option<String>,
    pub alt: Option<String>,
}

#[derive(Debug, PartialEq)]
pub struct LocalizedText {
    pub lang: Option<LanguageTag>,
    pub content: String,
}

#[derive(Debug, PartialEq)]
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

impl FromStrOpt for Genre {
    fn from_str_or_none(value: &str) -> Option<Self> {
        use Genre::*;
        Some(match value {
            "accounting" => Accounting,
            "adv_animal" => AdvAnimal,
            "adv_geo" => AdvGeo,
            "adv_history" => AdvHistory,
            "adv_maritime" => AdvMaritime,
            "adv_western" => AdvWestern,
            "adventure" => Adventure,
            "antique" => Antique,
            "antique_ant" => AntiqueAnt,
            "antique_east" => AntiqueEast,
            "antique_european" => AntiqueEuropean,
            "antique_myths" => AntiqueMyths,
            "antique_russian" => AntiqueRussian,
            "aphorism_quote" => AphorismQuote,
            "architecture_book" => ArchitectureBook,
            "auto_regulations" => AutoRegulations,
            "banking" => Banking,
            "beginning_authors" => BeginningAuthors,
            "child_adv" => ChildAdv,
            "child_det" => ChildDet,
            "child_education" => ChildEducation,
            "child_prose" => ChildProse,
            "child_sf" => ChildSf,
            "child_tale" => ChildTale,
            "child_verse" => ChildVerse,
            "children" => Children,
            "cinema_theatre" => CinemaTheatre,
            "city_fantasy" => CityFantasy,
            "comp_db" => CompDb,
            "comp_hard" => CompHard,
            "comp_osnet" => CompOsnet,
            "comp_programming" => CompProgramming,
            "comp_soft" => CompSoft,
            "comp_www" => CompWww,
            "computers" => Computers,
            "design" => Design,
            "det_action" => DetAction,
            "det_classic" => DetClassic,
            "det_crime" => DetCrime,
            "det_espionage" => DetEspionage,
            "det_hard" => DetHard,
            "det_history" => DetHistory,
            "det_irony" => DetIrony,
            "det_police" => DetPolice,
            "det_political" => DetPolitical,
            "detective" => Detective,
            "dragon_fantasy" => DragonFantasy,
            "dramaturgy" => Dramaturgy,
            "economics" => Economics,
            "essays" => Essays,
            "fantasy_fight" => FantasyFight,
            "foreign_action" => ForeignAction,
            "foreign_adventure" => ForeignAdventure,
            "foreign_antique" => ForeignAntique,
            "foreign_business" => ForeignBusiness,
            "foreign_children" => ForeignChildren,
            "foreign_comp" => ForeignComp,
            "foreign_contemporary" => ForeignContemporary,
            "foreign_contemporary_lit" => ForeignContemporaryLit,
            "foreign_desc" => ForeignDesc,
            "foreign_detective" => ForeignDetective,
            "foreign_dramaturgy" => ForeignDramaturgy,
            "foreign_edu" => ForeignEdu,
            "foreign_fantasy" => ForeignFantasy,
            "foreign_home" => ForeignHome,
            "foreign_humor" => ForeignHumor,
            "foreign_language" => ForeignLanguage,
            "foreign_love" => ForeignLove,
            "foreign_novel" => ForeignNovel,
            "foreign_other" => ForeignOther,
            "foreign_poetry" => ForeignPoetry,
            "foreign_prose" => ForeignProse,
            "foreign_psychology" => ForeignPsychology,
            "foreign_publicism" => ForeignPublicism,
            "foreign_religion" => ForeignReligion,
            "foreign_sf" => ForeignSf,
            "geo_guides" => GeoGuides,
            "geography_book" => GeographyBook,
            "global_economy" => GlobalEconomy,
            "historical_fantasy" => HistoricalFantasy,
            "home" => Home,
            "home_cooking" => HomeCooking,
            "home_crafts" => HomeCrafts,
            "home_diy" => HomeDiy,
            "home_entertain" => HomeEntertain,
            "home_garden" => HomeGarden,
            "home_health" => HomeHealth,
            "home_pets" => HomePets,
            "home_sex" => HomeSex,
            "home_sport" => HomeSport,
            "humor" => Humor,
            "humor_anecdote" => HumorAnecdote,
            "humor_fantasy" => HumorFantasy,
            "humor_prose" => HumorProse,
            "humor_verse" => HumorVerse,
            "industries" => Industries,
            "job_hunting" => JobHunting,
            "literature_18" => Literature18,
            "literature_19" => Literature19,
            "literature_20" => Literature20,
            "love_contemporary" => LoveContemporary,
            "love_detective" => LoveDetective,
            "love_erotica" => LoveErotica,
            "love_fantasy" => LoveFantasy,
            "love_history" => LoveHistory,
            "love_sf" => LoveSf,
            "love_short" => LoveShort,
            "magician_book" => MagicianBook,
            "management" => Management,
            "marketing" => Marketing,
            "military_special" => MilitarySpecial,
            "music_dancing" => MusicDancing,
            "narrative" => Narrative,
            "newspapers" => Newspapers,
            "nonf_biography" => NonfBiography,
            "nonf_criticism" => NonfCriticism,
            "nonf_publicism" => NonfPublicism,
            "nonfiction" => Nonfiction,
            "org_behavior" => OrgBehavior,
            "paper_work" => PaperWork,
            "pedagogy_book" => PedagogyBook,
            "periodic" => Periodic,
            "personal_finance" => PersonalFinance,
            "poetry" => Poetry,
            "popadanec" => Popadanec,
            "popular_business" => PopularBusiness,
            "prose_classic" => ProseClassic,
            "prose_contemporary" => ProseContemporary,
            "prose_counter" => ProseCounter,
            "prose_history" => ProseHistory,
            "prose_military" => ProseMilitary,
            "prose_rus_classic" => ProseRusClassic,
            "prose_su_classics" => ProseSuClassics,
            "psy_alassic" => PsyAlassic,
            "psy_childs" => PsyChilds,
            "psy_generic" => PsyGeneric,
            "psy_personal" => PsyPersonal,
            "psy_sex_and_family" => PsySexAndFamily,
            "psy_social" => PsySocial,
            "psy_theraphy" => PsyTheraphy,
            "real_estate" => RealEstate,
            "ref_dict" => RefDict,
            "ref_encyc" => RefEncyc,
            "ref_guide" => RefGuide,
            "ref_ref" => RefRef,
            "reference" => Reference,
            "religion" => Religion,
            "religion_esoterics" => ReligionEsoterics,
            "religion_rel" => ReligionRel,
            "religion_self" => ReligionSelf,
            "russian_contemporary" => RussianContemporary,
            "russian_fantasy" => RussianFantasy,
            "sci_biology" => SciBiology,
            "sci_chem" => SciChem,
            "sci_culture" => SciCulture,
            "sci_history" => SciHistory,
            "sci_juris" => SciJuris,
            "sci_linguistic" => SciLinguistic,
            "sci_math" => SciMath,
            "sci_medicine" => SciMedicine,
            "sci_philosophy" => SciPhilosophy,
            "sci_phys" => SciPhys,
            "sci_psychology" => SciPsychology,
            "sci_politics" => SciPolitics,
            "sci_religion" => SciReligion,
            "sci_tech" => SciTech,
            "science" => Science,
            "sf" => Sf,
            "sf_action" => SfAction,
            "sf_cyberpunk" => SfCyberpunk,
            "sf_detective" => SfDetective,
            "sf_fantasy" => SfFantasy,
            "sf_heroic" => SfHeroic,
            "sf_history" => SfHistory,
            "sf_horror" => SfHorror,
            "sf_humor" => SfHumor,
            "sf_social" => SfSocial,
            "sf_space" => SfSpace,
            "short_story" => ShortStory,
            "sketch" => Sketch,
            "small_business" => SmallBusiness,
            "sociology_book" => SociologyBook,
            "stock" => Stock,
            "thriller" => Thriller,
            "upbringing_book" => UpbringingBook,
            "vampire_book" => VampireBook,
            "visual_arts" => VisualArts,
            "unrecognised" => Unrecognised,
            _ => return None,
        })
    }
}
