extern crate libvplan;

use libvplan::parser;

#[test]
fn parse_wrong_1() {
    assert_eq!(parser::parse("Hello World").is_err(), true)
}

#[test]
fn parse_wrong_2() {
    assert_eq!(parser::parse("<root>Hello World!</root>").is_err(), true)
}

#[test]
fn parse_wrong_3() {
    assert_eq!(parser::parse("<root>Hello World!").is_err(), true)
}
