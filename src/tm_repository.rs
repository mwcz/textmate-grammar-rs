use crate::tm_rule::TMRule;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
#[serde(bound(deserialize = "'de: 'a"))]
pub struct TMRepository<'a>(HashMap<&'a str, TMRule<'a>>);
