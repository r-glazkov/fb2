use chrono::NaiveDate;

use fb2::*;

use crate::common::compare;

mod common;

#[test]
fn parse_many_bodies() {
    let expected = FictionBook {
        stylesheets: vec![],
        description: Description {
            title_info: TitleInfo {
                genres: vec![
                    GenreWithMatch {
                        match_percentage: 100,
                        value: Genre::NonfBiography,
                    },
                    GenreWithMatch {
                        match_percentage: 100,
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
                                lang: None,
                                value: "Уинстон".into(),
                            },
                            middle_name: Some(
                                LocalizedText {
                                    lang: None,
                                    value: "Спенсер".into(),
                                },
                            ),
                            last_name: LocalizedText {
                                lang: None,
                                value: "Черчилль".into(),
                            },
                            nickname: None,
                            home_pages: vec![],
                            emails: vec![],
                            id: Some(
                                "7dc6a193-2a83-102a-9ae1-2dfe723fe7c7".into(),
                            ),
                        },
                    ),
                ],
                book_title: LocalizedText {
                    lang: None,
                    value: "Вторая мировая война".into(),
                },
                annotation: Some(
                    Annotation {
                        id: None,
                        lang: None,
                        elements: vec![
                            AnnotationElement::Paragraph(
                                Paragraph {
                                    id: None,
                                    lang: None,
                                    style: None,
                                    elements: vec![
                                        StyleElement::Text(
                                            "Шеститомный труд У.\u{a0}Черчилля – героическая эпопея народов, выступивших против планетарной опасности, написанная выдающимся политиком, скрупулезным историком и талантливым литератором. Это летопись повседневного руководства страной государственного деятеля, чей вклад в общее дело победы антигитлеровской коалиции ни у кого не вызывает сомнений. Это размышления над прошлым, призванные послужить назиданием потомкам. В первой книге публикуются в сокращенном переводе с английского I и II тома мемуаров и описаны события с 1919 года по декабрь 1940 года, которые привели к ненужной, по словам автора, войне, которой можно было избежать. Во второй книге публикуются третий и четвертый тома мемуаров и описаны события в период с января 1941 по июнь 1943\u{a0}г.: вторжение фашистской Германии в Советский Союз, нападение милитаристской Японии на США, создание антигитлеровской коалиции, переход союзников от обороны к наступлению. В третьей книге публикуются пятый и шестой тома мемуаров и описаны события в период с июня 1943\u{a0}г. по июль 1945\u{a0}г.\u{a0}– капитуляция союзников Германии, Тегеранская, Ялтинская и Потсдамская конференции, высадка американских, английских и канадских войск в Нормандии, разгром гитлеровских войск в результате исторических побед Советской армии и союзников, капитуляция Германии.".into(),
                                        ),
                                        StyleElement::Emphasis(
                                            Style {
                                                lang: None,
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
                        lang: None,
                        value: "Вторая мировая война,военная история,Великобритания,политическая публицистика,знаменитые политики".into(),
                    },
                ),
                date: Some(
                    Date {
                        lang: None,
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
                            middle_name: None,
                            last_name: LocalizedText {
                                lang: None,
                                value: "Орлов".into(),
                            },
                            nickname: None,
                            home_pages: vec![],
                            emails: vec![],
                            id: Some(
                                "14fc587f-0911-11e5-99b8-0025905a069a".into(),
                            ),
                        },
                    ),
                ],
                sequences: vec![],
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
                        lang: None,
                        value: "FictionBook Editor Release 2.6.6".into(),
                    },
                ),
                date: Some(Date {
                    lang: None,
                    iso_date: NaiveDate::from_ymd_opt(2014, 6, 11),
                    display_date: Some("11 June 2014".into()),
                }),
                src_urls: vec![
                    "http://www.litres.ru/pages/biblio_book/?art=7003942&lfrom=700971545".into(),
                ],
                src_ocr: Some(
                    LocalizedText {
                        lang: None,
                        value: "Текст предоставлен издательством".into(),
                    },
                ),
                id: Some("a75a6f71-f140-11e3-871d-0025905a0812".into()),
                version: Some(1.0),
                history: Some(
                    Annotation {
                        id: None,
                        lang: None,
                        elements: vec![
                            AnnotationElement::Paragraph(
                                Paragraph {
                                    id: None,
                                    lang: None,
                                    style: None,
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
                            lang: None,
                            value: "Вторая мировая война: В 6 тт. Т. 1: Надвигающаяся буря; Т. 2: Их звездный\n                час; Т. 3: Великий союз; Т. 4: Поворот судьбы; Т. 5: Кольцо смыкается; Т. 6: Триумф\n                и трагедия / Уинстон Черчилль; Сокр. пер. с\u{a0}англ. 4-е изд.".into(),
                        },
                    ),
                    publisher: Some(
                        LocalizedText {
                            lang: None,
                            value: "Альпина нон-фикшн".into(),
                        },
                    ),
                    city: Some(
                        LocalizedText {
                            lang: None,
                            value: "Москва".into(),
                        },
                    ),
                    year: Some(
                        2013,
                    ),
                    isbn: Some(
                        LocalizedText {
                            lang: None,
                            value: "978-5-9614-3115-5".into(),
                        },
                    ),
                    sequences: vec![],
                },
            ),
            custom_info: vec![],
            output: vec![],
        },
        bodies: vec![Body {
            name: None,
            lang: None,
            image: None,
            title: Some(
                Title {
                    lang: None,
                    elements: vec![
                        TitleElement::Paragraph(
                            Paragraph {
                                id: None,
                                lang: None,
                                style: None,
                                elements: vec![
                                    StyleElement::Text(
                                        "Уинстон Черчилль".into(),
                                    ),
                                ],
                            },
                        ),
                        TitleElement::Paragraph(
                            Paragraph {
                                id: None,
                                lang: None,
                                style: None,
                                elements: vec![
                                    StyleElement::Text(
                                        "Вторая мировая война".into(),
                                    ),
                                ],
                            },
                        ),
                    ],
                },
            ),
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
                                                "Руководитель проекта".into(),
                                            ),
                                            StyleElement::Emphasis(
                                                Style {
                                                    lang: None,
                                                    elements: vec![
                                                        StyleElement::Text(
                                                            "А. Шувалова".into(),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "Технический редактор".into(),
                                            ),
                                            StyleElement::Emphasis(
                                                Style {
                                                    lang: None,
                                                    elements: vec![
                                                        StyleElement::Text(
                                                            "Н. Лисицына".into(),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "Корректор".into(),
                                            ),
                                            StyleElement::Emphasis(
                                                Style {
                                                    lang: None,
                                                    elements: vec![
                                                        StyleElement::Text(
                                                            "Е. Аксёнова".into(),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "Компьютерная верстка".into(),
                                            ),
                                            StyleElement::Emphasis(
                                                Style {
                                                    lang: None,
                                                    elements: vec![
                                                        StyleElement::Text(
                                                            "М. Поташкин, А. Фоминов".into(),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "Художник обложки".into(),
                                            ),
                                            StyleElement::Emphasis(
                                                Style {
                                                    lang: None,
                                                    elements: vec![
                                                        StyleElement::Text(
                                                            "Ю. Буга".into(),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::EmptyLine,
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "I: The Gathering Storm © The Estate of Sir Winston S Churchill".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "First published by Casell 1948".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "II: Their Finest Hour © The Estate of Sir Winston S Churchill".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "First published by Casell 1949".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "III: The Grand Alliance © The Estate of Sir Winston S Churchill".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "First published by Casell 1950".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "IV: The Hinge of Fate © The Estate of Sir Winston S Churchill".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "First published by Casell 1950".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "V: Triumph and Tragedy © The Estate of Sir Winston S Churchill".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "First published by Casell 1951".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "VI: Closing the Ring © The Estate of Sir Winston S Churchill".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "First published by Casell 1953".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::EmptyLine,
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Text(
                                                "© Издание на русском языке, оформление. «Альпина нон-фикшн», 2010".into(),
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::EmptyLine,
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![
                                            StyleElement::Emphasis(
                                                Style {
                                                    lang: None,
                                                    elements: vec![
                                                        StyleElement::Text(
                                                            "Все права защищены. Никакая часть электронной версии этой книги не может быть воспроизведена в какой бы то ни было форме и какими бы то ни было средствами, включая размещение в сети Интернет и в корпоративных сетях, для частного и публичного использования без письменного разрешения владельца авторских прав.".into(),
                                                        ),
                                                    ],
                                                },
                                            ),
                                        ],
                                    },
                                ),
                                SectionPart::EmptyLine,
                                SectionPart::Paragraph(
                                    Paragraph {
                                        id: None,
                                        lang: None,
                                        style: None,
                                        elements: vec![],
                                    },
                                ),
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
                     Body {
                         name: Some("notes".into()),
                         lang: None,
                         image: None,
                         title: Some(Title {
                             lang: None,
                             elements: vec![
                                 TitleElement::Paragraph(Paragraph {
                                     id: None,
                                     lang: None,
                                     style: None,
                                     elements: vec![
                                         StyleElement::Text("Примечания".into())
                                     ],
                                 })
                             ],
                         }),
                         epigraphs: vec![],
                         sections: vec![
                             Section {
                                 id: Some("n_1".into()),
                                 lang: None,
                                 content: Some(SectionContent {
                                     title: Some(Title {
                                         lang: None,
                                         elements: vec![
                                             TitleElement::Paragraph(Paragraph {
                                                 id: None,
                                                 lang: None,
                                                 style: None,
                                                 elements: vec![
                                                     StyleElement::Text("1".into())
                                                 ],
                                             })
                                         ],
                                     }),
                                     epigraphs: vec![],
                                     image: None,
                                     annotation: None,
                                     content: vec![
                                         SectionPart::Paragraph(Paragraph {
                                             id: None,
                                             lang: None,
                                             style: None,
                                             elements: vec![
                                                 StyleElement::Emphasis(Style {
                                                     lang: None,
                                                     elements: vec![
                                                         StyleElement::Text("Достоевский Ф. М.".into())
                                                     ],
                                                 }),
                                                 StyleElement::Text("Полн. собр. соч.: В. 30 т. Л., 1980. Т. 21. С. 133.".into()),
                                             ],
                                         }),
                                     ],
                                     sections: vec![],
                                 }),
                             }
                         ],
                     },
                     Body {
                         name: Some("comments".into()),
                         lang: None,
                         image: None,
                         title: Some(Title {
                             lang: None,
                             elements: vec![
                                 TitleElement::Paragraph(Paragraph {
                                     id: None,
                                     lang: None,
                                     style: None,
                                     elements: vec![
                                         StyleElement::Text("Комментарии".into())
                                     ],
                                 })
                             ],
                         }),
                         epigraphs: vec![],
                         sections: vec![
                             Section {
                                 id: Some("c_1".into()),
                                 lang: None,
                                 content: Some(SectionContent {
                                     title: Some(Title {
                                         lang: None,
                                         elements: vec![
                                             TitleElement::Paragraph(Paragraph {
                                                 id: None,
                                                 lang: None,
                                                 style: None,
                                                 elements: vec![
                                                     StyleElement::Text("1".into())
                                                 ],
                                             })
                                         ],
                                     }),
                                     epigraphs: vec![],
                                     image: None,
                                     annotation: None,
                                     content: vec![
                                         SectionPart::Paragraph(Paragraph {
                                             id: None,
                                             lang: None,
                                             style: None,
                                             elements: vec![
                                                 StyleElement::Text("С. 31.".into()),
                                                 StyleElement::Emphasis(Style {
                                                     lang: None,
                                                     elements: vec![
                                                         StyleElement::Text("Ох уж эти мне сказочники!..".into())
                                                     ],
                                                 }),
                                                 StyleElement::Text(" — Эпиграф взят из рассказа В. Ф. Одоевского «Живой мертвец» (1839; ср.:".into()),
                                                 StyleElement::Emphasis(Style {
                                                     lang: None,
                                                     elements: vec![
                                                         StyleElement::Text("Одоевский В. Ф.".into())
                                                     ],
                                                 }),
                                             ],
                                         }),
                                         SectionPart::Paragraph(Paragraph {
                                             id: None,
                                             lang: None,
                                             style: None,
                                             elements: vec![
                                                 StyleElement::Text("Соч. СПб., 1844. Ч. 3. С. 140); в конце отрывка слово «запретить» переделано Достоевским на «запретил».".into()),
                                             ],
                                         }),
                                     ],
                                     sections: vec![],
                                 }),
                             }
                         ],
                     },
        ],
        binaries: vec![],
    };
    compare("tests/resources/many_bodies.fb2", expected)
}
