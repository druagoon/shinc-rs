use nom::branch::alt;
use nom::bytes::complete::{tag, take_while1};
use nom::character::complete::{char, space0, space1};
use nom::combinator::{eof, map, rest, success};
use nom::multi::many1;
use nom::sequence::{pair, preceded};
use nom::Parser;

use super::tag as argc_tag;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Event {
    pub position: Position,
    pub data: EventData,
}

pub type Position = usize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EventData {
    /// Metadata
    Meta(String, String),
    /// Include file
    Include(String),
    /// Placeholder for unknown or invalid tag
    Unknown(String),
}

/// Tokenize shell script
#[allow(dead_code)]
pub fn parse(source: &str) -> anyhow::Result<Vec<Event>> {
    let mut result = vec![];
    let lines: Vec<&str> = source.lines().collect();
    let mut line_idx = 0;
    while line_idx < lines.len() {
        let line = lines[line_idx];
        let position = line_idx + 1;
        match parse_line(line) {
            Ok((_, maybe_token)) => {
                if let Some(maybe_data) = maybe_token {
                    if let Some(data) = maybe_data {
                        result.push(Event { position, data });
                    } else {
                        anyhow::bail!("syntax error at line {}", position);
                    }
                } else {
                    result.push(Event { position, data: EventData::Unknown(line.to_string()) });
                }
            }
            Err(err) => {
                anyhow::bail!("fail to parse at line {}, {}", position, err);
            }
        }
        line_idx += 1;
    }
    Ok(result)
}

fn parse_line(line: &str) -> nom::IResult<&str, Option<Option<EventData>>> {
    alt((map(alt((parse_tag,)), Some), success(None))).parse(line)
}

fn parse_tag(input: &str) -> nom::IResult<&str, Option<EventData>> {
    preceded(
        (many1(char('#')), space0, char('@')),
        alt((parse_tag_text, parse_tag_meta, parse_tag_unknown)),
    )
    .parse(input)
}

fn parse_tag_text(input: &str) -> nom::IResult<&str, Option<EventData>> {
    map(pair(alt((tag(argc_tag::INCLUDE),)), parse_tail), |(tag, text)| {
        let text = text.to_string();
        Some(match tag {
            argc_tag::INCLUDE => EventData::Include(text),
            _ => unreachable!(),
        })
    })
    .parse(input)
}

fn parse_tag_meta(input: &str) -> nom::IResult<&str, Option<EventData>> {
    preceded(
        tag(argc_tag::META),
        map(preceded(space1, parse_key_value), |kv| {
            kv.map(|(k, v)| EventData::Meta(k.to_string(), v.to_string()))
        }),
    )
    .parse(input)
}

fn parse_tag_unknown(input: &str) -> nom::IResult<&str, Option<EventData>> {
    map(rest, |v: &str| Some(EventData::Unknown(format!("# @{}", v)))).parse(input)
}

fn parse_key_value(input: &str) -> nom::IResult<&str, Option<(&str, &str)>> {
    let input = input.trim_end();
    let key_value = map(pair(parse_name, parse_tail), |(key, value)| Some((key, value)));

    alt((key_value, success(None))).parse(input)
}

fn parse_tail(input: &str) -> nom::IResult<&str, &str> {
    alt((eof, preceded(space1, alt((eof, map(rest, |v: &str| v.trim())))))).parse(input)
}

fn parse_name(input: &str) -> nom::IResult<&str, &str> {
    take_while1(is_name_char)(input)
}

fn is_name_char(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_' || is_special_var_char(c)
}

pub fn is_special_var_char(c: char) -> bool {
    matches!(c, '-' | '.' | ':' | '@')
}
