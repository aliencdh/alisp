use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Sym(String),
    Atom(Val),
    Func(String, Vec<Expr>),
}

#[derive(Clone, PartialEq, Debug)]
pub enum Val {
    Int(i64),
    Float(f64),
    Str(String),
    List(Vec<Val>),
}

mod regexes {
    pub use regex::Regex;

    lazy_static! {
        pub static ref INT: Regex = Regex::new(r"^([1-9]+||0)[0-9]*$").unwrap();
    }
    lazy_static! {
        pub static ref FLOAT: Regex = Regex::new(r"^([1-9]+||0)[0-9]*\.[0-9]+$").unwrap();
    }
    lazy_static! {
        pub static ref STR: Regex = Regex::new(r#"^".*"$"#).unwrap();
    }
    lazy_static! {
        pub static ref LIST: Regex = Regex::new(r"^'\(.+\)$").unwrap();
    }

    lazy_static! {
        pub static ref FUNC: Regex = Regex::new(r"^\(\s*(?P<funcname>[a-zA-Z_]+[0-9a-zA-Z_]*)\s*(?P<args>.*)\)$").unwrap();
    }
    lazy_static! {
        pub static ref SYM: Regex = Regex::new(r"^[a-zA-Z_]+[0-9a-zA-Z_]*$").unwrap();
    }
    lazy_static! {
        pub static ref SEP_ARGS: Regex = Regex::new(r"(.*)\(.*\)(.*)").unwrap();
    }
}

pub fn try_parse_atom(src: &str) -> anyhow::Result<Val> {
    use Val::*;
    use regexes::*;

    if INT.is_match(src) {
        Ok( Int(i64::from_str(src)?) )
    } else if FLOAT.is_match(src) {
        Ok( Float(f64::from_str(src)?) )
    } else if STR.is_match(src) {
        Ok(
            Str(
                src
                .chars()
                .skip(1)
                .take(src.len() - 2)
                .collect::<String>()
                )
           )
    } else if LIST.is_match(src) {
        todo!()
    } else {
        Err(anyhow::Error::msg(format!("malformed value: `{}`", src)))
    }
}

pub fn try_parse_expr(src: &str) -> anyhow::Result<Expr> {
    use Expr::*;
    use regexes::*;

    if SYM.is_match(src) {
        Ok(Sym(String::from(src)))
    } else if FUNC.is_match(src) {
        let mat = FUNC.captures(src).unwrap();
        let func_name = String::from(&mat["funcname"]);

        let mut args = vec![];

        if src.chars().skip(1).collect::<String>().contains("(") {
            // until first function
            let (before, nested_after) = &mat["args"].split_once("(").unwrap();
            let (nested, after) = nested_after.rsplit_once(")").unwrap();
    
            let before_args = before
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(str::trim)
                .collect::<Vec<_>>();
            let after_args = after
                .split(" ")
                .filter(|s| !s.is_empty())
                .map(str::trim)
                .collect::<Vec<_>>();
    
            for arg in before_args {
                args.push(try_parse_expr(arg)?);
            }
            args.push(try_parse_expr(
                    &(String::from("(") + nested + ")")
                    )?);
            for arg in after_args {
                args.push(try_parse_expr(arg)?);
            }
        } else {
            let raw_args = &mat["args"]
                .split(" ")
                .into_iter()
                .filter(|s| !s.is_empty())
                .map(str::trim)
                .collect::<Vec<_>>();
            for arg in raw_args {
                args.push(try_parse_expr(arg)?);
            }
        }

        Ok(Func(func_name, args))
    } else {
        Ok(Atom(try_parse_atom(src)?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_atom_number() {
        use Val::*;

        let input = vec!["1", "99", "39019272", "0", "89328.32378", "0.0"];
        let expected = vec![
            Int(1), 
            Int(99), 
            Int(39019272),
            Int(0),
            Float(89328.32378),
            Float(0.0),
        ];

        let result = input.iter()
            .map(|s| try_parse_atom(*s).unwrap())
            .collect::<Vec<_>>();

        assert_eq!(expected, result);
    }

    #[test]
    fn test_parse_atom_string() {
        use Val::Str;

        let input = vec![r#""ilahids89090""#, r#""some string \"quoted string\"""#];
        let result = input.iter()
            .map(|s| try_parse_atom(*s).unwrap())
            .collect::<Vec<_>>();
        let expected = vec![
            Str(String::from("ilahids89090")), 
            Str(String::from(r#"some string \"quoted string\""#))
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_expr_sym() {
        use Expr::Sym;

        let input = vec!["AuhLahdsd_93089", "_90380293____dlauhdkS"];
        let result = input.iter()
            .map(|s| try_parse_expr(*s).unwrap())
            .collect::<Vec<_>>();
        let expected = vec![
            Sym(String::from("AuhLahdsd_93089")),
            Sym(String::from("_90380293____dlauhdkS"))
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_expr_func() {
        use Expr::*;
        use Val::*;

        let input = vec![
            "(dlhadk_90898 980890 0.0 jlhdksd)", 
            "(___idojldi980_ jkd (defhkh dsakdj))"
        ];
        let result = input.iter()
            .map(|s| try_parse_expr(*s).unwrap())
            .collect::<Vec<_>>();
        let expected = vec![
            Func(
                String::from("dlhadk_90898"), 
                vec![ Atom(Int(980890)), Atom(Float(0.0)), Sym(String::from("jlhdksd")) ]
                ),
            Func(
                String::from("___idojldi980_"),
                vec![ 
                    Sym(String::from("jkd")),
                    Func(String::from("defhkh"), vec![Sym(String::from("dsakdj"))])
                    ]
                )
        ];
        assert_eq!(result, expected);
    }
}
