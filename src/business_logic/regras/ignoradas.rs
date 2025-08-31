use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref REGRAS_IGNORADAS_POR_CIDADE: HashMap<&'static str, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert("Rio de Janeiro", vec!["RegraAreaLazerMin"]);
        m.insert("São Paulo", vec!["RegraAlturaMax"]);
        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regras_ignoradas() {
        assert_eq!(
            REGRAS_IGNORADAS_POR_CIDADE.get("Rio de Janeiro").unwrap(),
            &vec!["RegraAreaLazerMin"]
        );
        assert_eq!(
            REGRAS_IGNORADAS_POR_CIDADE.get("São Paulo").unwrap(),
            &vec!["RegraAlturaMax"]
        );
        assert!(REGRAS_IGNORADAS_POR_CIDADE.get("CidadeX").is_none());
    }
}
