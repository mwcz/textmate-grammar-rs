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
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/abap.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn actionscript_3_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/actionscript-3.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn ada_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ada.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn apache_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/apache.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn apex_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/apex.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn apl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/apl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn applescript_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/applescript.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn ara_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ara.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn asm_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/asm.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn astro_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/astro.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn awk_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/awk.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn ballerina_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ballerina.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn bat_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/bat.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn beancount_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/beancount.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn berry_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/berry.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn bibtex_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/bibtex.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn bicep_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/bicep.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn blade_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/blade.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn cadence_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cadence.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn clarity_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/clarity.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn clojure_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/clojure.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn cmake_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cmake.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn cobol_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cobol.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn codeql_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/codeql.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn coffee_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/coffee.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn cpp_macro_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cpp-macro.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn cpp_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cpp.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn crystal_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/crystal.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn csharp_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/csharp.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn css_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/css.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn c_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/c.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn cue_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cue.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn cypher_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/cypher.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn dart_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/dart.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn dax_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/dax.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn diff_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/diff.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn docker_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/docker.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn dream_maker_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/dream-maker.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn d_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/d.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn elixir_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/elixir.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn elm_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/elm.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn erb_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/erb.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn erlang_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/erlang.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn fish_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/fish.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn fsharp_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/fsharp.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn gdresource_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gdresource.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn gdscript_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gdscript.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn gdshader_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gdshader.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn gherkin_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gherkin.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn git_commit_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/git-commit.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn git_rebase_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/git-rebase.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn glimmer_js_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/glimmer-js.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn glimmer_ts_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/glimmer-ts.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn glsl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/glsl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn gnuplot_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/gnuplot.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn go_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/go.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn graphql_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/graphql.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn groovy_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/groovy.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn hack_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hack.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn haml_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/haml.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn handlebars_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/handlebars.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn haskell_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/haskell.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn hcl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hcl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn hjson_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hjson.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn hlsl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/hlsl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn html_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/html.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn http_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/http.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn imba_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/imba.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn ini_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ini.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn javascript_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/javascript.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn java_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/java.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jinja_html_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jinja-html.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jinja_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jinja.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jison_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jison.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn json5_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/json5.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jsonc_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsonc.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jsonl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsonl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jsonnet_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsonnet.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn json_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/json.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jssm_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jssm.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn jsx_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/jsx.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn julia_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/julia.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn kotlin_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/kotlin.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn kusto_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/kusto.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn latex_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/latex.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn less_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/less.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn liquid_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/liquid.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn lisp_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/lisp.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn logo_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/logo.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn lua_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/lua.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn make_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/make.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn markdown_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/markdown.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn marko_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/marko.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn matlab_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/matlab.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn mdx_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/mdx.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn mermaid_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/mermaid.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn mojo_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/mojo.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn narrat_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/narrat.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn nextflow_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nextflow.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn nginx_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nginx.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn nim_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nim.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn nix_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/nix.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn objective_cpp_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/objective-cpp.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn objective_c_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/objective-c.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn ocaml_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ocaml.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn pascal_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/pascal.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn perl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/perl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn php_html_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/php-html.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn php_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/php.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn plsql_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/plsql.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn postcss_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/postcss.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn powerquery_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/powerquery.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn powershell_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/powershell.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn prisma_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/prisma.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn prolog_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/prolog.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn proto_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/proto.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn pug_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/pug.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn puppet_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/puppet.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn purescript_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/purescript.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn python_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/python.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn raku_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/raku.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn razor_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/razor.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn reg_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/reg.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn rel_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/rel.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn riscv_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/riscv.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn rst_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/rst.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn r_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/r.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn ruby_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ruby.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn rust_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/rust.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn sass_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sass.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn sas_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sas.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn scala_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/scala.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn scheme_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/scheme.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn scss_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/scss.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn shaderlab_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/shaderlab.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn shellscript_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/shellscript.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn shellsession_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/shellsession.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn smalltalk_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/smalltalk.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn solidity_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/solidity.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn sparql_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sparql.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn splunk_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/splunk.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn sql_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/sql.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn ssh_config_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/ssh-config.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn stata_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/stata.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn stylus_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/stylus.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn svelte_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/svelte.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn swift_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/swift.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn system_verilog_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/system-verilog.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn tasl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tasl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn tcl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tcl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn tex_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tex.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn toml_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/toml.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn tsx_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/tsx.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn turtle_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/turtle.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn twig_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/twig.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn typescript_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/typescript.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn vb_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vb.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn verilog_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/verilog.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn vhdl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vhdl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn viml_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/viml.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn v_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/v.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn vue_html_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vue-html.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn vue_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vue.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn vyper_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/vyper.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn wasm_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wasm.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn wenyan_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wenyan.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn wgsl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wgsl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn wolfram_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/wolfram.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn xml_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/xml.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn xsl_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/xsl.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn yaml_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/yaml.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn zenscript_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/zenscript.tmLanguage.json"));
        println!("{asm:#?}");
    }
    #[test]
    fn zig_test() {
        let asm: Result<TMGrammar, _> =
            serde_json::from_str(include_str!("../languages/zig.tmLanguage.json"));
        println!("{asm:#?}");
    }
}
