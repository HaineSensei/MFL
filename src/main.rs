pub mod mfl_regex {
    use regex::Regex;
    use std::sync::LazyLock;

    pub const COMMENT_STR : &str = r"//[^\n]*";
    pub static COMMENT : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(COMMENT_STR).unwrap()
    });

    pub const LET_STR : &str = r"let";
    pub static LET : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(LET_STR).unwrap()
    });

    pub const IF_STR : &str = r"if";
    pub static IF : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(IF_STR).unwrap()
    });

    pub const ELSE_STR : &str = r"else";
    pub static ELSE : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(ELSE_STR).unwrap()
    });

    pub const FN_STR : &str = r"fn";
    pub static FN : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(FN_STR).unwrap()
    });

    pub const WHILE_STR : &str = r"while";
    pub static WHILE : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(WHILE_STR).unwrap()
    });

    pub const IDENTIFIER_STR : &str = r"[^0-9\s\(\)\+\-\*\[\]\{\}@\?\|\.\^][^\s\(\)\+\-\*\[\]\{\}@\?\|\.\^]*";
    pub static IDENTIFIER : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(IDENTIFIER_STR).unwrap()
    });
    
    pub static KEYWORD_STRING : LazyLock<String> = LazyLock::new(|| format!(r"(?P<let>{})|(?P<if>{})|(?P<else>{})|(?P<while>{})|(?P<fn>{})", LET_STR, IF_STR, ELSE_STR, WHILE_STR, FN_STR));
    pub static KEYWORD : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(KEYWORD_STRING.as_str()).unwrap()
    });

    pub const WHITESPACE_STR : &str = r"\s+";
    pub static WHITESPACE : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(WHITESPACE_STR).unwrap()
    });

    pub const NUMBER_STR : &str = r"[0-9]+";
    pub static NUMBER : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(NUMBER_STR).unwrap()
    });

    pub const OPERATOR_STR : &str = r"(?P<add>\+)|(?P<mul>\*)|(?P<sub>\-)|(?P<div>/)|(?P<mod>%)|(?P<lte><=)|(?P<lt><(?!=))|(?P<gte>>=)|(?P<gt>>(?!=))|(?P<eq>==)|(?P<neq>!=)|(?P<not>!(?!=))|(?P<and>&&)|(?P<or>\|\|)";
    pub static OPERATOR : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(OPERATOR_STR).unwrap()
    });

    pub const PUNCTUATION_STR : &str = r"(?P<assign>=)|(?P<lp>\()|(?P<rp>\))|(?P<lb>\{)|(?P<rb>\})|(?P<comma>,)|(?P<sc>;)|(?P<colon>:)|(?P<arrow>\->)";
    pub static PUNCTUATION : LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(PUNCTUATION_STR).unwrap()
    });
}

fn main() {
    println!("HEYA!");
}
