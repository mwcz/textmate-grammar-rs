use std::ops::Deref;

use serde::Deserialize;

use crate::tm_rule::TMRule;

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct TMPatterns<'a>(Vec<TMRule<'a>>);

impl<'a> Deref for TMPatterns<'a> {
    type Target = Vec<TMRule<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
