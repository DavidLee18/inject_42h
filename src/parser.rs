use chrono::{DateTime, FixedOffset, NaiveDateTime};
use nom::bytes::complete::tag;
use nom::character::complete::{alphanumeric1, anychar, newline, space1};
use nom::multi::many_till;
use nom::IResult;

pub struct _42Header {
    pub file_name: String,
    pub name: String,
    pub email: String,
    pub created: DateTime<FixedOffset>,
    pub author_name: String,
    pub updated: DateTime<FixedOffset>,
    pub updater_name: String,
}

pub fn _42header(s: &str) -> IResult<&str, _42Header> {
    let (s, _) =
        tag("/* ************************************************************************** */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) =
        tag("/*                                                                            */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) =
        tag("/*                                                        :::      ::::::::   */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = tag("/*   ")(s)?;
    let (s, (file_name, _)): (&str, (Vec<char>, &str)) = many_till(anychar, space1)(s)?;
    let (s, _) = many_till(space1, tag(":+:      :+:    :+:   */"))(s)?;
    let (s, _) = newline(s)?;
    let (s, _) =
        tag("/*                                                    +:+ +:+         +:+     */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = tag("/*   By: ")(s)?;
    let (s, name) = alphanumeric1(s)?;
    let (s, _) = tag(" <")(s)?;
    let (s, (email, _)): (&str, (Vec<char>, &str)) = many_till(anychar, tag(">"))(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = tag("+#+  +:+       +#+        */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) =
        tag("/*                                                +#+#+#+#+#+   +#+           */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = tag("/*   Created: ")(s)?;
    let (s, (created, _)): (&str, (Vec<char>, &str)) = many_till(anychar, tag(" by "))(s)?;
    let (s, author_name) = alphanumeric1(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = tag("#+#    #+#             */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) = tag("/*   Updated: ")(s)?;
    let (s, (updated, _)): (&str, (Vec<char>, &str)) = many_till(anychar, tag(" by "))(s)?;
    let (s, updater_name) = alphanumeric1(s)?;
    let (s, _) = space1(s)?;
    let (s, _) = tag("###   ########.fr       */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) =
        tag("/*                                                                            */")(s)?;
    let (s, _) = newline(s)?;
    let (s, _) =
        tag("/* ************************************************************************** */")(s)?;
    let (s, _) = newline(s)?;

    Ok((
        s,
        _42Header {
            file_name: file_name.into_iter().collect(),
            name: name.to_string(),
            email: email.into_iter().collect(),
            created: DateTime::from_naive_utc_and_offset(
                NaiveDateTime::parse_from_str(
                    &created.into_iter().collect::<String>(),
                    "%Y/%m/%d %H:%M:%S",
                )
                .expect("not a valid datetime format"),
                FixedOffset::east_opt(0).unwrap(),
            ),

            author_name: author_name.to_string(),
            updated: DateTime::from_naive_utc_and_offset(
                NaiveDateTime::parse_from_str(
                    &updated.into_iter().collect::<String>(),
                    "%Y/%m/%d %H:%M:%S",
                )
                .expect("not a valid datetime format"),
                FixedOffset::east_opt(0).unwrap(),
            ),
            updater_name: updater_name.to_string(),
        },
    ))
}
