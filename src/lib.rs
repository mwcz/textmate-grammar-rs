/// TextMate grammar deserializer.
///
/// See https://macromates.com/manual/en/language_grammars
mod tm_patterns;
mod tm_regex;
mod tm_repository;
mod tm_rule;

use crate::tm_regex::TMRegex;
use serde::Deserialize;
use tm_patterns::TMPatterns;
use tm_repository::TMRepository;

#[derive(Debug, Deserialize)]
#[allow(unused)]
struct TMGrammar<'a> {
    #[serde(rename = "scopeName")]
    scope_name: &'a str,
    #[serde(rename = "fileTypes")]
    file_types: Option<Vec<&'a str>>,
    #[serde(rename = "foldingStartMarker")]
    folding_start_marker: Option<TMRegex>,
    #[serde(rename = "foldingStopMarker")]
    folding_stop_marker: Option<TMRegex>,
    patterns: Option<TMPatterns<'a>>,
    #[serde(rename = "firstLineMatch")]
    first_line_match: Option<TMRegex>,
    repository: Option<TMRepository<'a>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn abap_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/abap.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn actionscript_3_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/actionscript-3.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn ada_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ada.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn apache_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/apache.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn apex_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/apex.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn apl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/apl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn applescript_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/applescript.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn ara_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ara.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn g_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/asm.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn astro_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/astro.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn awk_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/awk.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn ballerina_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ballerina.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn bat_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/bat.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn beancount_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/beancount.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn berry_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/berry.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn bibtex_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/bibtex.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn bicep_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/bicep.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn blade_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/blade.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn cadence_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cadence.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn clarity_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/clarity.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn clojure_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/clojure.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn cmake_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cmake.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn cobol_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cobol.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn codeql_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/codeql.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn coffee_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/coffee.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn cpp_macro_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cpp-macro.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn cpp_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cpp.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn crystal_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/crystal.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn csharp_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/csharp.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn css_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/css.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn c_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/c.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn cue_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cue.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn cypher_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cypher.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn dart_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/dart.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn dax_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/dax.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn diff_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/diff.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn docker_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/docker.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn dream_maker_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/dream-maker.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn d_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/d.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn elixir_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/elixir.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn elm_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/elm.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn erb_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/erb.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn erlang_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/erlang.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn fish_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/fish.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn fsharp_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/fsharp.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn gdresource_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gdresource.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn gdscript_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gdscript.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn gdshader_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gdshader.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn gherkin_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gherkin.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn git_commit_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/git-commit.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn git_rebase_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/git-rebase.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn glimmer_js_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/glimmer-js.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn glimmer_ts_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/glimmer-ts.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn glsl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/glsl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn gnuplot_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gnuplot.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn go_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/go.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn graphql_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/graphql.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn groovy_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/groovy.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn hack_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hack.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn haml_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/haml.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn handlebars_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/handlebars.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn haskell_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/haskell.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn hcl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hcl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn hjson_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hjson.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn hlsl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hlsl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn html_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/html.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn http_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/http.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn imba_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/imba.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn ini_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ini.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn javascript_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/javascript.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn java_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/java.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jinja_html_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jinja-html.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jinja_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jinja.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jison_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jison.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn json5_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/json5.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jsonc_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsonc.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jsonl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsonl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jsonnet_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsonnet.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn json_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/json.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jssm_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jssm.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn jsx_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsx.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn julia_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/julia.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn kotlin_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/kotlin.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn kusto_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/kusto.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn latex_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/latex.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn less_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/less.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn liquid_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/liquid.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn lisp_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/lisp.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn logo_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/logo.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn lua_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/lua.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn make_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/make.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn markdown_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/markdown.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn marko_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/marko.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn matlab_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/matlab.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn mdx_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/mdx.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn mermaid_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/mermaid.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn mojo_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/mojo.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn narrat_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/narrat.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn nextflow_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nextflow.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn nginx_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nginx.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn nim_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nim.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn nix_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nix.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn objective_cpp_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/objective-cpp.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn objective_c_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/objective-c.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn ocaml_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ocaml.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn pascal_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/pascal.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn perl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/perl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn php_html_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/php-html.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn php_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/php.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn plsql_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/plsql.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn postcss_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/postcss.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn powerquery_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/powerquery.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn powershell_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/powershell.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn prisma_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/prisma.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn prolog_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/prolog.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn proto_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/proto.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn pug_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/pug.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn puppet_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/puppet.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn purescript_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/purescript.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn python_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/python.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn raku_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/raku.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn razor_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/razor.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn reg_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/reg.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn rel_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/rel.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn riscv_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/riscv.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn rst_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/rst.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn r_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/r.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn ruby_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ruby.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn rust_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/rust.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn sass_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sass.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn sas_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sas.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn scala_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/scala.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn scheme_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/scheme.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn scss_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/scss.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn shaderlab_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/shaderlab.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn shellscript_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/shellscript.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn shellsession_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/shellsession.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn smalltalk_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/smalltalk.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn solidity_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/solidity.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn sparql_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sparql.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn splunk_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/splunk.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn sql_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sql.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn ssh_config_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ssh-config.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn stata_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/stata.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn stylus_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/stylus.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn svelte_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/svelte.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn swift_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/swift.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn system_verilog_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/system-verilog.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn tasl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tasl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn tcl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tcl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn tex_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tex.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn toml_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/toml.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn tsx_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tsx.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn turtle_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/turtle.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn twig_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/twig.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn typescript_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/typescript.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn vb_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vb.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn verilog_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/verilog.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn vhdl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vhdl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn viml_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/viml.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn v_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/v.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn vue_html_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vue-html.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn vue_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vue.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn vyper_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vyper.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn wg_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wasm.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn wenyan_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wenyan.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn wgsl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wgsl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn wolfram_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wolfram.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn xml_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/xml.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn xsl_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/xsl.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn yaml_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/yaml.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn zenscript_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/zenscript.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
    #[test]
    fn zig_test() {
        let g: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/zig.tmLanguage.json"));

        println!("{g:#?}");

        assert!(g.is_ok());
    }
}
