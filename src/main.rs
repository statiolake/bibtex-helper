use std::io::prelude::*;
use std::io;
use std::collections::HashMap;

#[derive(PartialEq,Eq,Hash)]
enum Kind { // {{{
    Article,
    PhdThesis,
    MasterThesis,
    Proceedings,
    InProceedings,
    Conference,
    Book,
    Booklet,
    InBook,
    InCollection,
    Manual,
    TechReport,
    UnPublished,
    Misc,
} // }}}

fn prepare_map() -> HashMap<Kind, (&'static str, Vec<&'static str>, Vec<&'static str>)> { // {{{
    let mut map = HashMap::new();
    map.insert(
        Kind::Article,
        (
            "article",
            vec!["author", "title", "journal year"],
            vec!["volume", "number", "pages", "month", "note"]
        )
    );
    map.insert(
        Kind::PhdThesis,
        (
            "phdthesis",
            vec!["author", "title", "school", "year"],
            vec!["type", "address", "month", "note"]
        )
    );
    map.insert(
        Kind::MasterThesis,
        (
            "masterthesis",
            vec!["author", "title", "school", "year"],
            vec!["type", "address", "month", "note"]
        )
    );
    map.insert(
        Kind::Proceedings,
        (
            "proceedings",
            vec!["title", "year"],
            vec!["editor", "volume", "number", "series", "address", "month", "organization", "publisher", "note"]
        )
    );
    map.insert(
        Kind::InProceedings,
        (
            "inproceedings",
            vec!["author", "title", "booktitle", "year"],
            vec!["editor", "volume", "number", "series", "pages", "address", "month", "organization", "publisher", "note"]
        )
    );
    map.insert(
        Kind::Conference,
        (
            "conference",
            vec!["author", "title", "booktitle", "year"],
            vec!["editor", "volume", "number", "series", "pages", "address", "month", "organization", "publisher", "note"]
        )
    );
    map.insert(
        Kind::Book,
        (
            "book",
            vec!["author", "title", "pages", "publisher", "year"],
            vec!["volume","number", "series", "address", "edition", "month", "note"]
        )
    );
    map.insert(
        Kind::Booklet,
        (
            "booklet",
            vec!["title"],
            vec!["author", "howpublished", "address", "month", "year", "note"]
        )
    );
    map.insert(
        Kind::InBook,
        (
            "inbook",
            vec!["author", "title", "pages", "publisher", "year"],
            vec!["volume", "number", "series", "type", "address", "edition", "month", "year"]
        )
    );
    map.insert(
        Kind::InCollection,
        (
            "incollection",
            vec!["author", "title", "booktitle", "publisher", "year"],
            vec!["editor", "volume", "number", "series", "type", "chapter", "pages", "address", "edition", "month", "note"]
        )
    );
    map.insert(
        Kind::Manual,
        (
            "manual",
            vec!["title"],
            vec!["author", "organization", "address", "edition", "month", "year", "note"]
        )
    );
    map.insert(
        Kind::TechReport,
        (
            "techreport",
            vec!["author", "title", "institution", "year"],
            vec!["type", "number", "address", "month", "note"]
        )
    );
    map.insert(
        Kind::UnPublished,
        (
            "unpublished",
            vec!["author", "title", "note"],
            vec!["month", "year"]
        )
    );
    map.insert(
        Kind::Misc,
        (
            "misc",
            vec![],
            vec!["author", "title", "howpublished", "month", "year", "note"]
        )
    );

    map
} // }}}

fn input<S: AsRef<str>>(prompt: S) -> String {
    eprint!("{}", prompt.as_ref());
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).unwrap();

    result
}

fn main() {
    let map = prepare_map();

    for key in map.keys() {
        eprintln!("{}", map[key].0);
    }
    let kind = input("Select a type > ");
    let kind = kind.trim();
    let kind = map.iter().find(|x| (x.1).0 == kind).unwrap();
    let mut element = HashMap::new();

    let mut reference: String;
    loop {
        reference = input("first, what's the reference name? > ");
        if reference.trim() != "" { reference = reference.trim().into(); break; }
        eprintln!("this value is necessary.");
    }

    eprintln!("next, fill in needed elements");
    for title in (kind.1).1.iter() {
        let mut tmp: String;
        loop {
            tmp = input(format!("{} [needed] > ", title));
            if tmp.trim() != "" { break; }
            eprintln!("this value is necessary.");
        }
        element.insert(title, tmp.trim().to_string());
    }

    eprintln!("finally, fill in optional elements");
    for title in (kind.1).2.iter() {
        let tmp = input(format!("{} [optional] > ", title));
        if tmp.trim() != "" {
            element.insert(title, tmp.trim().to_string());
        }
    }

    println!(r"@{}{{
    {},
{}
}}",
    (kind.1).0,
    reference,
    element
        .iter()
        .map(|x| format!("    {} = \"{}\"", x.0, x.1))
        .collect::<Vec<_>>()
        .join(",\n")
);
}
