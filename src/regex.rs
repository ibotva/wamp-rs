
pub mod uri_rules {
    pub use regex::Regex;

    pub struct WampUriRule {
        pub loose: Regex,
        pub strict: Regex
    }

    pub trait Rule {
        fn rule(&self) -> WampUriRule;
    }

    pub enum EasyRule {
        WithEmpty,
        NoEmpty
    }

    impl Rule for EasyRule {
        fn rule(&self) -> WampUriRule {
            match &self {
                EasyRule::WithEmpty => {
                    WampUriRule {
                        loose: Regex::new(r"^(([^\s\.#]+\.)|\.)*([^\s\.#]+)?$").unwrap(),
                        strict: Regex::new(r"^(([0-9a-z_]+\.)|\.)*([0-9a-z_]+)?$").unwrap()
                    }
                },
                EasyRule::NoEmpty => {
                    WampUriRule {
                        strict: Regex::new(r"^([0-9a-z_]+\.)*([0-9a-z_]+)$").unwrap(),
                        loose: Regex::new(r"^([^\s\.#]+\.)*([^\s\.#]+)$").unwrap()
                    }
                }
            }
        }
    }
    
    /// Wamp URI Rules
    /// Read More: https://wamp-proto.org/wamp_latest_ietf.html#section-16.1.2-11
    pub enum WampRules {
        Name,
        URI,
        Prefix,
        PrefixOrWildcard,
    }

    impl Rule for WampRules {
        fn rule(&self) -> WampUriRule {
            match self {
                WampRules::Name => {
                    WampUriRule {
                        loose: Regex::new(r"^[^\s\.#]+$").unwrap(),
                        strict: Regex::new(r"^[\da-z_]+$").unwrap()
                    }
                }

                WampRules::URI => {
                    WampUriRule {
                        loose: Regex::new(r"^([^\s\.#]+\.)*([^\s\.#]+)$").unwrap(),
                        strict: Regex::new(r"^([\da-z_]+\.)*([\da-z_]+)$").unwrap()
                    }
                }

                WampRules::PrefixOrWildcard => {
                    WampUriRule {
                        loose: Regex::new(r"^(([^\s\.#]+\.)|\.)*([^\s\.#]+)?$").unwrap(),
                        strict: Regex::new(r"^(([\da-z_]+\.)|\.)*([\da-z_]+)?$").unwrap()
                    }
                }

                WampRules::Prefix => {
                    WampUriRule {
                        loose: Regex::new(r"^([^\s\.#]+\.)*([^\s\.#]*)$").unwrap(),
                        strict: Regex::new(r"^([\da-z_]+\.)*([\da-z_]*)$").unwrap()
                    }
                }
            }
        }
    }
    
}

//pub struct URI(String);
//
//impl URI {
//    pub fn validate<R: uri_rules::Rule>(r: R, v: String) -> Self {
//        todo!();
//        let rule = r.rule().loose;
//        //let values = rule.capture
//    }
//}
