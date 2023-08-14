/// TextMate grammar deserializer.
///
/// See https://macromates.com/manual/en/language_grammars
mod tm_regex;
mod tm_rule;

use std::collections::HashMap;

use crate::tm_regex::TMRegex;
use serde::Deserialize;
use tm_rule::TMRule;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct TMGrammar<'a> {
    #[serde(rename = "scopeName")]
    scope_name: &'a str,
    #[serde(rename = "fileTypes")]
    file_types: Vec<&'a str>,
    #[serde(rename = "foldingStartMarker")]
    folding_start_marker: TMRegex,
    #[serde(rename = "foldingStopMarker")]
    folding_stop_marker: TMRegex,
    // patterns: (),
    #[serde(rename = "firstLineMatch")]
    first_line_match: TMRegex,

    // TODO: this doesn't adequately capture the valid types for repository
    repository: HashMap<&'a str, TMRule<'a>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn asm_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/asm.tmLanguage.json"));

        println!("{asm:#?}");
    }
}
