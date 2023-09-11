use chrono::NaiveDate;
use language_tags::LanguageTag;

use fb2::*;

use crate::common::compare;

mod common;

#[test]
fn parse_complex() {
    let expected = FictionBook {
        stylesheets: vec![
            Stylesheet {
                kind: "text/css".into(),
                content: "body { padding: 1px; }".into(),
            },
            Stylesheet {
                kind: "text/css".into(),
                content: "footer { padding: 1px; }".into(),
            },
        ],
        description: Description {
            title_info: TitleInfo {
                genres: vec![
                    GenreWithMatch {
                        match_percentage: 100,
                        value: Genre::NonfBiography,
                    },
                    GenreWithMatch {
                        match_percentage: 10,
                        value: Genre::SciHistory,
                    },
                    GenreWithMatch {
                        match_percentage: 100,
                        value: Genre::Nonfiction,
                    },
                ],
                authors: vec![
                    Author::Verbose(
                        VerboseAuthorDetails {
                            first_name: LocalizedText {
                                lang: Some(LanguageTag::parse("ru").unwrap()),
                                value: "Уинстон".into(),
                            },
                            middle_name: Some(
                                LocalizedText {
                                    lang: Some(LanguageTag::parse("ru-RU").unwrap()),
                                    value: "Спенсер".into(),
                                },
                            ),
                            last_name: LocalizedText {
                                lang: Some(LanguageTag::parse("ru-LT").unwrap()),
                                value: "Черчилль".into(),
                            },
                            nickname: Some(
                                LocalizedText {
                                    lang: Some(LanguageTag::parse("en").unwrap()),
                                    value: "winston_cigarette".into(),
                                },
                            ),
                            home_pages: vec![
                                "winston.example.com".into(),
                                "churchill.example.com".into(),
                            ],
                            emails: vec![
                                "winston@example.com".into(),
                                "churchill@example.com".into(),
                            ],
                            id: Some(
                                "7dc6a193-2a83-102a-9ae1-2dfe723fe7c7".into(),
                            ),
                        },
                    ),
                    Author::Anonymous(
                        AnonymousAuthorDetails {
                            nickname: Some(LocalizedText {
                                lang: Some(LanguageTag::parse("en").unwrap()),
                                value: "vovchik".into(),
                            }),
                            home_pages: vec![
                                "profile.example.com".into(),
                                "blog.example.com".into(),
                            ],
                            emails: vec![
                                "vovchik@example.com".into(),
                                "vladimir@example.com".into(),
                            ],
                            id: Some(
                                "fe6b1dae-acf9-4bd1-947e-78065eda64c2".into(),
                            ),
                        },
                    ),
                ],
                book_title: LocalizedText {
                    lang: Some(LanguageTag::parse("ru").unwrap()),
                    value: "Вторая мировая война".into(),
                },
                annotation: Some(
                    Annotation {
                        id: Some(
                            "ID-1".into(),
                        ),
                        lang: Some(LanguageTag::parse("ru").unwrap()),
                        elements: vec![
                            AnnotationElement::Paragraph(
                                Paragraph {
                                    id: Some(
                                        "ID-2".into(),
                                    ),
                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                    style: Some(
                                        "some style".into(),
                                    ),
                                    elements: vec![
                                        StyleElement::Text(
                                            "Шеститомный труд У.\u{a0}Черчилля – героическая эпопея народов, выступивших против\n                    планетарной опасности, написанная выдающимся политиком, скрупулезным историком и талантливым литератором. Это летопись\n                    повседневного руководства страной государственного деятеля, чей вклад в общее дело победы антигитлеровской коалиции ни у\n                    кого не вызывает сомнений. Это размышления над прошлым, призванные послужить назиданием потомкам. В первой книге\n                    публикуются в сокращенном переводе с английского I и II тома мемуаров и описаны события с 1919 года по декабрь 1940\n                    года, которые привели к ненужной, по словам автора, войне, которой можно было избежать. Во второй книге публикуются\n                    третий и четвертый тома мемуаров и описаны события в период с января 1941 по июнь 1943\u{a0}г.: вторжение фашистской Германии\n                    в Советский Союз, нападение милитаристской Японии на США, создание антигитлеровской коалиции, переход союзников от\n                    обороны к наступлению. В третьей книге публикуются пятый и шестой тома мемуаров и описаны события в период с июня\n                    1943\u{a0}г. по июль 1945\u{a0}г.\u{a0}– капитуляция союзников Германии, Тегеранская, Ялтинская и Потсдамская конференции, высадка\n                    американских, английских и канадских войск в Нормандии, разгром гитлеровских войск в результате исторических побед\n                    Советской армии и союзников, капитуляция Германии.".into(),
                                        ),
                                        StyleElement::Emphasis(
                                            Style {
                                                lang: Some(LanguageTag::parse("ru").unwrap()),
                                                elements: vec![
                                                    StyleElement::Text(
                                                        "4-е издание.".into(),
                                                    ),
                                                ],
                                            },
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                keywords: Some(
                    LocalizedText {
                        lang: Some(LanguageTag::parse("ru").unwrap()),
                        value: "Вторая мировая война,военная история,Великобритания,политическая публицистика,знаменитые политики".into(),
                    },
                ),
                date: Some(
                    Date {
                        lang: Some(LanguageTag::parse("en").unwrap()),
                        iso_date: NaiveDate::from_ymd_opt(1953, 01, 01),
                        display_date: Some("1948-53".into()),
                    },
                ),
                cover_page: Some(
                    Covers {
                        images: vec![
                            InlineImage {
                                kind: "simple".into(),
                                href: Some("#cover.jpg".into()),
                                alt: None,
                            },
                            InlineImage {
                                kind: "simple".into(),
                                href: Some("#cover.jpg".into()),
                                alt: None,
                            },
                        ],
                    },
                ),
                lang: "ru".into(),
                src_lang: Some(
                    "en".into(),
                ),
                translators: vec![
                    Author::Verbose(
                        VerboseAuthorDetails {
                            first_name: LocalizedText {
                                lang: None,
                                value: "Александр".into(),
                            },
                            middle_name: Some(
                                LocalizedText {
                                    lang: None,
                                    value: "Владимирович".into(),
                                },
                            ),
                            last_name: LocalizedText {
                                lang: None,
                                value: "Орлов".into(),
                            },
                            nickname: Some(
                                LocalizedText {
                                    lang: None,
                                    value: "alfa".into(),
                                },
                            ),
                            home_pages: vec![
                                "orlov.example.com".into(),
                                "alex.example.com".into(),
                            ],
                            emails: vec![
                                "orlov@example.com".into(),
                                "alex@example.com".into(),
                            ],
                            id: Some(
                                "14fc587f-0911-11e5-99b8-0025905a069a".into(),
                            ),
                        },
                    ),
                    Author::Anonymous(
                        AnonymousAuthorDetails {
                            nickname: Some(LocalizedText {
                                lang: None,
                                value: "alfa".into(),
                            }),
                            home_pages: vec![
                                "orlov.example.com".into(),
                                "alex.example.com".into(),
                            ],
                            emails: vec![
                                "orlov@example.com".into(),
                                "alex@example.com".into(),
                            ],
                            id: Some(
                                "14fc587f-0911-11e5-99b8-0025905a069a".into(),
                            ),
                        },
                    ),
                ],
                sequences: vec![
                    Sequence {
                        name: Some("Книга, чё".into()),
                        number: Some(
                            1,
                        ),
                        lang: Some(LanguageTag::parse("ru").unwrap()),
                        sequences: vec![
                            Sequence {
                                name: Some("Два".into()),
                                number: Some(
                                    2,
                                ),
                                lang: Some(LanguageTag::parse("ru-RU").unwrap()),
                                sequences: vec![],
                            },
                            Sequence {
                                name: Some("Three".into()),
                                number: Some(
                                    3,
                                ),
                                lang: Some(LanguageTag::parse("en-AU").unwrap()),
                                sequences: vec![],
                            },
                        ],
                    },
                    Sequence {
                        name: Some("Зачем это нужно".into()),
                        number: Some(
                            1,
                        ),
                        lang: Some(LanguageTag::parse("ru-KZ").unwrap()),
                        sequences: vec![
                            Sequence {
                                name: Some("Два".into()),
                                number: Some(
                                    2,
                                ),
                                lang: Some(LanguageTag::parse("ru-RU").unwrap()),
                                sequences: vec![],
                            },
                            Sequence {
                                name: Some("Three".into()),
                                number: Some(
                                    3,
                                ),
                                lang: Some(LanguageTag::parse("en-AU").unwrap()),
                                sequences: vec![],
                            },
                        ],
                    },
                ],
            },
            src_title_info: None,
            document_info: Some(DocumentInfo {
                authors: vec![
                    Author::Anonymous(
                        AnonymousAuthorDetails {
                            nickname: Some(LocalizedText {
                                lang: None,
                                value: "On84ly".into(),
                            }),
                            home_pages: vec![],
                            emails: vec![],
                            id: None,
                        },
                    ),
                ],
                program_used: Some(
                    LocalizedText {
                        lang: Some(LanguageTag::parse("en").unwrap()),
                        value: "FictionBook Editor Release 2.6.6".into(),
                    },
                ),
                date: Some(Date {
                    lang: Some(LanguageTag::parse("en").unwrap()),
                    iso_date: NaiveDate::from_ymd_opt(2014, 6, 11),
                    display_date: Some("11 June 2014".into()),
                }),
                src_urls: vec![
                    "http://www.litres.ru/pages/biblio_book/?art=7003942&lfrom=700971545".into(),
                    "http://example.com".into(),
                ],
                src_ocr: Some(
                    LocalizedText {
                        lang: Some(LanguageTag::parse("ru").unwrap()),
                        value: "Текст предоставлен издательством".into(),
                    },
                ),
                id: Some("a75a6f71-f140-11e3-871d-0025905a0812".into()),
                version: Some(1.0),
                history: Some(
                    Annotation {
                        id: Some(
                            "ID-3".into(),
                        ),
                        lang: Some(LanguageTag::parse("ru").unwrap()),
                        elements: vec![
                            AnnotationElement::Paragraph(
                                Paragraph {
                                    id: Some(
                                        "ID-4".into(),
                                    ),
                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                    style: Some(
                                        "sss".into(),
                                    ),
                                    elements: vec![
                                        StyleElement::Text(
                                            "v 1.0 – создание fb2 – (On84ly)".into(),
                                        ),
                                    ],
                                },
                            ),
                        ],
                    },
                ),
                publishers: vec![
                    Author::Verbose(
                        VerboseAuthorDetails {
                            first_name: LocalizedText {
                                lang: None,
                                value: "Литагент".into(),
                            },
                            middle_name: None,
                            last_name: LocalizedText {
                                lang: None,
                                value: "Альпина".into(),
                            },
                            nickname: None,
                            home_pages: vec![],
                            emails: vec![],
                            id: Some(
                                "6bdeff1e-120c-11e2-86b3-b737ee03444a".into(),
                            ),
                        },
                    ),
                ],
            }),
            publish_info: Some(
                PublishInfo {
                    book_name: Some(
                        LocalizedText {
                            lang: Some(LanguageTag::parse("ru").unwrap()),
                            value: "Вторая мировая война: В 6 тт. Т. 1: Надвигающаяся буря; Т. 2: Их звездный\n                час; Т. 3: Великий союз; Т. 4: Поворот судьбы; Т. 5: Кольцо смыкается; Т. 6: Триумф\n                и трагедия / Уинстон Черчилль; Сокр. пер. с\u{a0}англ. 4-е изд.".into(),
                        },
                    ),
                    publisher: Some(
                        LocalizedText {
                            lang: Some(LanguageTag::parse("ru").unwrap()),
                            value: "Альпина нон-фикшн".into(),
                        },
                    ),
                    city: Some(
                        LocalizedText {
                            lang: Some(LanguageTag::parse("ru").unwrap()),
                            value: "Москва".into(),
                        },
                    ),
                    year: Some(
                        2013,
                    ),
                    isbn: Some(
                        LocalizedText {
                            lang: Some(LanguageTag::parse("en").unwrap()),
                            value: "978-5-9614-3115-5".into(),
                        },
                    ),
                    sequences: vec![
                        Sequence {
                            name: Some("Очередная последовательность".into()),
                            number: Some(
                                6,
                            ),
                            lang: Some(LanguageTag::parse("ru").unwrap()),
                            sequences: vec![],
                        },
                        Sequence {
                            name: Some("Очередная посредственность".into()),
                            number: Some(
                                7,
                            ),
                            lang: Some(LanguageTag::parse("ru").unwrap()),
                            sequences: vec![
                                Sequence {
                                    name: Some("yep".into()),
                                    number: Some(
                                        8,
                                    ),
                                    lang: Some(LanguageTag::parse("en").unwrap()),
                                    sequences: vec![],
                                },
                                Sequence {
                                    name: Some("nope".into()),
                                    number: Some(
                                        9,
                                    ),
                                    lang: Some(LanguageTag::parse("en").unwrap()),
                                    sequences: vec![],
                                },
                            ],
                        },
                    ],
                },
            ),
            custom_info: vec![
                CustomInfo {
                    info_type: "test-custom-info".into(),
                    lang: Some(LanguageTag::parse("ru").unwrap()),
                    content: "вот немного нетипичного текста".into(),
                },
                CustomInfo {
                    info_type: "test-custom-info-2".into(),
                    lang: Some(LanguageTag::parse("en").unwrap()),
                    content: "here is some custom text".into(),
                },
            ],
            output: vec![
                ShareInstruction {
                    mode: ShareMode::Paid,
                    include_all: DocGenerationInstruction::Require,
                    price: Some(10.0),
                    currency: Some("RUB".into()),
                    elements: vec![
                        ShareInstructionElement::Part(PartShareInstruction {
                            kind: "simple".into(),
                            href: "http://example.com".into(),
                            include: DocGenerationInstruction::Allow,
                        }),
                        ShareInstructionElement::OutputDocumentClass(OutputDocumentClass {
                            name: "paid-doc".into(),
                            create: Some(DocGenerationInstruction::Allow),
                            price: Some(10.0),
                            parts: vec![
                                PartShareInstruction {
                                    kind: "simple".into(),
                                    href: "#ID-3".into(),
                                    include: DocGenerationInstruction::Deny,
                                }
                            ],
                        }),
                    ],
                },
                ShareInstruction {
                    mode: ShareMode::Free,
                    include_all: DocGenerationInstruction::Deny,
                    price: None,
                    currency: None,
                    elements: vec![],
                },
            ],
        },
        bodies: vec![
            Body {
                name: None,
                lang: Some(LanguageTag::parse("ru").unwrap()),
                image: Some(
                    Image {
                        kind: "simple".into(),
                        href: Some("#body.jpg".into()),
                        alt: Some(
                            "подмена".into(),
                        ),
                        title: Some(
                            "Заголовок".into(),
                        ),
                        id: Some(
                            "ID-5".into(),
                        ),
                    },
                ),
                title: Some(
                    Title {
                        lang: Some(LanguageTag::parse("ru").unwrap()),
                        elements: vec![
                            TitleElement::Paragraph(
                                Paragraph {
                                    id: Some(
                                        "ID-6".into(),
                                    ),
                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                    style: Some(
                                        "wip".into(),
                                    ),
                                    elements: vec![
                                        StyleElement::Text(
                                            "Уинстон Черчилль".into(),
                                        ),
                                    ],
                                },
                            ),
                            TitleElement::EmptyLine,
                        ],
                    },
                ),
                epigraphs: vec![
                    Epigraph {
                        id: Some(
                            "ep-1".into(),
                        ),
                        elements: vec![
                            EpigraphElement::Paragraph(
                                Paragraph {
                                    id: Some(
                                        "pg-1".into(),
                                    ),
                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                    style: Some(
                                        "pg-s-1".into(),
                                    ),
                                    elements: vec![
                                        StyleElement::Text(
                                            "Content".into(),
                                        ),
                                    ],
                                },
                            ),
                            EpigraphElement::Poem(
                                Poem {
                                    id: Some(
                                        "po-1".into(),
                                    ),
                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                    title: Some(
                                        Title {
                                            lang: Some(LanguageTag::parse("ru").unwrap()),
                                            elements: vec![
                                                TitleElement::Paragraph(
                                                    Paragraph {
                                                        id: None,
                                                        lang: None,
                                                        style: None,
                                                        elements: vec![
                                                            StyleElement::Text(
                                                                "Some title".into(),
                                                            ),
                                                        ],
                                                    },
                                                ),
                                            ],
                                        },
                                    ),
                                    epigraphs: vec![
                                        Epigraph {
                                            id: Some(
                                                "ep-2".into(),
                                            ),
                                            elements: vec![
                                                EpigraphElement::EmptyLine,
                                            ],
                                            text_authors: vec![],
                                        },
                                        Epigraph {
                                            id: Some(
                                                "ep-3".into(),
                                            ),
                                            elements: vec![
                                                EpigraphElement::EmptyLine,
                                            ],
                                            text_authors: vec![],
                                        },
                                    ],
                                    stanzas: vec![
                                        PoemStanza::Subtitle(
                                            Paragraph {
                                                id: Some(
                                                    "st-1".into(),
                                                ),
                                                lang: Some(LanguageTag::parse("en").unwrap()),
                                                style: Some(
                                                    "st-s-1".into(),
                                                ),
                                                elements: vec![
                                                    StyleElement::Text(
                                                        "fasdf".into(),
                                                    ),
                                                ],
                                            },
                                        ),
                                        PoemStanza::Stanza(
                                            Stanza {
                                                lang: Some(LanguageTag::parse("ru").unwrap()),
                                                title: Some(
                                                    Title {
                                                        lang: Some(LanguageTag::parse("ru").unwrap()),
                                                        elements: vec![
                                                            TitleElement::Paragraph(
                                                                Paragraph {
                                                                    id: None,
                                                                    lang: None,
                                                                    style: None,
                                                                    elements: vec![
                                                                        StyleElement::Text(
                                                                            "Танечка".into(),
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                        ],
                                                    },
                                                ),
                                                subtitle: Some(
                                                    Paragraph {
                                                        id: Some(
                                                            "st-2".into(),
                                                        ),
                                                        lang: Some(LanguageTag::parse("en").unwrap()),
                                                        style: Some(
                                                            "st-s-2".into(),
                                                        ),
                                                        elements: vec![
                                                            StyleElement::Text(
                                                                "Subtitle".into(),
                                                            ),
                                                        ],
                                                    },
                                                ),
                                                lines: vec![
                                                    Paragraph {
                                                        id: Some(
                                                            "v-1".into(),
                                                        ),
                                                        lang: Some(LanguageTag::parse("ru").unwrap()),
                                                        style: Some(
                                                            "v-s-1".into(),
                                                        ),
                                                        elements: vec![
                                                            StyleElement::Text(
                                                                "Плач".into(),
                                                            ),
                                                        ],
                                                    },
                                                    Paragraph {
                                                        id: Some(
                                                            "v-2".into(),
                                                        ),
                                                        lang: Some(LanguageTag::parse("ru").unwrap()),
                                                        style: Some(
                                                            "v-s-2".into(),
                                                        ),
                                                        elements: vec![
                                                            StyleElement::Text(
                                                                "Мяч".into(),
                                                            ),
                                                        ],
                                                    },
                                                ],
                                            },
                                        ),
                                    ],
                                    text_authors: vec![
                                        Paragraph {
                                            id: Some(
                                                "ta-2".into(),
                                            ),
                                            lang: Some(LanguageTag::parse("ru").unwrap()),
                                            style: Some(
                                                "ta-s-2".into(),
                                            ),
                                            elements: vec![
                                                StyleElement::Text(
                                                    "Барто".into(),
                                                ),
                                            ],
                                        },
                                        Paragraph {
                                            id: Some(
                                                "ta-3".into(),
                                            ),
                                            lang: Some(LanguageTag::parse("ru").unwrap()),
                                            style: Some(
                                                "ta-s-3".into(),
                                            ),
                                            elements: vec![
                                                StyleElement::Text(
                                                    "Агния".into(),
                                                ),
                                            ],
                                        },
                                    ],
                                    date: Some(
                                        Date {
                                            lang: Some(LanguageTag::parse("ru").unwrap()),
                                            iso_date: NaiveDate::from_ymd_opt(10, 10, 21),
                                            display_date: Some("Лохматые".into()),
                                        },
                                    ),
                                },
                            ),
                            EpigraphElement::Cite(
                                Cite {
                                    id: Some(
                                        "ci-1".into(),
                                    ),
                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                    elements: vec![
                                        CiteElement::Paragraph(
                                            Paragraph {
                                                id: None,
                                                lang: None,
                                                style: None,
                                                elements: vec![
                                                    StyleElement::Text(
                                                        "fasdf".into(),
                                                    ),
                                                ],
                                            },
                                        ),
                                        CiteElement::Poem(
                                            Poem {
                                                id: None,
                                                lang: None,
                                                title: None,
                                                epigraphs: vec![],
                                                stanzas: vec![
                                                    PoemStanza::Stanza(
                                                        Stanza {
                                                            title: None,
                                                            subtitle: None,
                                                            lines: vec![
                                                                Paragraph {
                                                                    id: None,
                                                                    lang: None,
                                                                    style: None,
                                                                    elements: vec![
                                                                        StyleElement::Text(
                                                                            "s".into(),
                                                                        ),
                                                                    ],
                                                                },
                                                            ],
                                                            lang: None,
                                                        },
                                                    ),
                                                ],
                                                text_authors: vec![],
                                                date: None,
                                            },
                                        ),
                                    ],
                                    text_authors: vec![],
                                },
                            ),
                            EpigraphElement::EmptyLine,
                        ],
                        text_authors: vec![
                            Paragraph {
                                id: Some(
                                    "ta-1".into(),
                                ),
                                lang: Some(LanguageTag::parse("ru").unwrap()),
                                style: Some(
                                    "ta-style".into(),
                                ),
                                elements: vec![
                                    StyleElement::Text(
                                        "Автор".into(),
                                    ),
                                ],
                            },
                            Paragraph {
                                id: Some(
                                    "ta-4".into(),
                                ),
                                lang: Some(LanguageTag::parse("ru").unwrap()),
                                style: Some(
                                    "ta-s-4".into(),
                                ),
                                elements: vec![
                                    StyleElement::Text(
                                        "Ещё Автор".into(),
                                    ),
                                ],
                            },
                        ],
                    },
                ],
                sections: vec![
                    Section {
                        id: Some(
                            "ID-8".into(),
                        ),
                        lang: Some(LanguageTag::parse("ru").unwrap()),
                        content: Some(
                            SectionContent {
                                title: Some(
                                    Title {
                                        lang: Some(LanguageTag::parse("ru").unwrap()),
                                        elements: vec![
                                            TitleElement::EmptyLine,
                                        ],
                                    },
                                ),
                                epigraphs: vec![],
                                image: None,
                                annotation: None,
                                content: vec![],
                                sections: vec![
                                    Section {
                                        id: None,
                                        lang: None,
                                        content: Some(
                                            SectionContent {
                                                title: None,
                                                epigraphs: vec![
                                                    Epigraph {
                                                        id: Some(
                                                            "ID-9".into(),
                                                        ),
                                                        elements: vec![
                                                            EpigraphElement::EmptyLine,
                                                            EpigraphElement::Paragraph(
                                                                Paragraph {
                                                                    id: Some(
                                                                        "ID-10".into(),
                                                                    ),
                                                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                                                    style: Some(
                                                                        "st".into(),
                                                                    ),
                                                                    elements: vec![
                                                                        StyleElement::Text(
                                                                            "Parag".into(),
                                                                        ),
                                                                    ],
                                                                },
                                                            ),
                                                            EpigraphElement::Cite(
                                                                Cite {
                                                                    id: Some(
                                                                        "ID-12".into(),
                                                                    ),
                                                                    lang: Some(LanguageTag::parse("ru").unwrap()),
                                                                    elements: vec![],
                                                                    text_authors: vec![],
                                                                },
                                                            ),
                                                        ],
                                                        text_authors: vec![],
                                                    },
                                                ],
                                                image: None,
                                                annotation: None,
                                                content: vec![
                                                    SectionPart::Subtitle(
                                                        Paragraph {
                                                            id: None,
                                                            lang: None,
                                                            style: None,
                                                            elements: vec![
                                                                StyleElement::Text(
                                                                    "* * *".into(),
                                                                ),
                                                            ],
                                                        },
                                                    ),
                                                ],
                                                sections: vec![],
                                            },
                                        ),
                                    },
                                ],
                            },
                        ),
                    },
                ],
            },
            Body {
                name: Some(
                    "notes".into(),
                ),
                lang: None,
                image: None,
                title: None,
                epigraphs: vec![],
                sections: vec![
                    Section {
                        id: None,
                        lang: None,
                        content: Some(
                            SectionContent {
                                title: None,
                                epigraphs: vec![],
                                image: None,
                                annotation: None,
                                content: vec![
                                    SectionPart::Paragraph(
                                        Paragraph {
                                            id: None,
                                            lang: None,
                                            style: None,
                                            elements: vec![
                                                StyleElement::Text(
                                                    "yep".into(),
                                                ),
                                            ],
                                        },
                                    ),
                                ],
                                sections: vec![],
                            },
                        ),
                    },
                ],
            },
        ],
        binaries: vec![
            Binary {
                id: "cover.jpg".into(),
                content_type: "image/jpeg".into(),
                content: "YXNkZgo=".into(),
            },
            Binary {
                id: "c1.jpg".into(),
                content_type: "image/jpeg".into(),
                content: "MTIzNAo=".into(),
            },
        ],
    };

    compare("tests/resources/complex.fb2", expected);
}
