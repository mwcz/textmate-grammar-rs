use std::{collections::HashMap, fmt, marker::PhantomData, ops::Deref};

use crate::{tm_patterns::TMPatterns, tm_regex::TMRegex};
use serde::{
    de::{MapAccess, Visitor},
    Deserialize, Deserializer,
};

// TODO: implement mutual exclusivity of match & begin/end

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TMRule<'a> {
    name: Option<&'a str>,

    #[serde(rename = "match")]
    match_re: Option<TMRegex>,
    begin: Option<TMRegex>,
    end: Option<TMRegex>,

    #[serde(rename = "contentName")]
    content_name: Option<&'a str>,

    captures: Option<TMCaptures<'a>>,
    #[serde(rename = "beginCaptures")]
    begin_captures: Option<TMCaptures<'a>>,
    #[serde(rename = "endCaptures")]
    end_captures: Option<TMCaptures<'a>>,

    include: Option<&'a str>,

    patterns: Option<TMPatterns<'a>>,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct TMCapture<'a> {
    name: &'a str,
}

#[derive(Debug)]
#[allow(unused)]
pub struct TMCaptures<'a>(TMCaptureInner<'a>);
type TMCaptureInner<'a> = HashMap<u16, TMCapture<'a>>;

impl<'a> Deref for TMCaptures<'a> {
    type Target = TMCaptureInner<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'de, 'a> Deserialize<'de> for TMCaptures<'a>
where
    'de: 'a,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct TMCapturesVisitor<'a> {
            _marker: PhantomData<&'a ()>,
        }

        impl<'de, 'a> Visitor<'de> for TMCapturesVisitor<'a>
        where
            'de: 'a,
        {
            type Value = TMCaptures<'a>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a map of u16 keys to TMCapture values")
            }

            fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
            where
                M: MapAccess<'de>,
            {
                let mut map = HashMap::new();
                while let Some(key) = access.next_key::<String>()? {
                    // standard fields here are... "0": { name: "..." }
                    // but many grammars contain non-standard fields.
                    // if those nonstandard fields are needed someday, this can be adjusted, but
                    // for now the following will parse keys which are parsable as u16, and values
                    // which are parsable as TMCapture (ie, { name: "" }).  fields which don't meet
                    // those requirements are skipped.
                    if let Ok(u16_key) = key.parse::<u16>() {
                        if let Ok(value) = access.next_value::<TMCapture<'a>>() {
                            map.insert(u16_key, value);
                        }
                    } else {
                        // Skip the value corresponding to the unparseable key
                        let _: serde_json::Value = access.next_value()?;
                    }
                }
                Ok(TMCaptures(map))
            }
        }

        deserializer.deserialize_map(TMCapturesVisitor {
            _marker: PhantomData,
        })
    }
}
