use std::{ops::Deref, str::FromStr};

use onig::Regex;
use serde::de::{Deserialize, Deserializer, Error};

#[derive(Debug)]
pub struct TMRegex(pub Regex);

impl Deref for TMRegex {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for TMRegex {
    type Err = onig::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(onig::Regex::new(s)?))
    }
}

struct TMRegexVisitor;

impl<'de> serde::de::Visitor<'de> for TMRegexVisitor {
    type Value = Regex;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            formatter,
            "a string representing an Oniguruma-compatible regular expression"
        )
    }

    fn visit_str<E: Error>(self, value: &str) -> Result<Self::Value, E> {
        Regex::new(value).map_err(E::custom)
    }
}

impl<'de> Deserialize<'de> for TMRegex {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Regex::new(&s)
            .map(TMRegex)
            .map_err(serde::de::Error::custom)
    }
}
