use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref FIELD_MAPPING: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("area-do-terreno", "area_do_terreno");
        m.insert("numero-de-torres", "numero_de_torres");
        m.insert("altura-da-torre", "altura_da_torre");
        m.insert("area-da-torre", "area_da_torre");
        m.insert("area-de-lazer", "area_de_lazer");
        m
    };
}

pub fn normalize_field_name(field_name: &str) -> &str {
    FIELD_MAPPING.get(field_name).copied().unwrap_or(field_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_field_name() {
        assert_eq!(normalize_field_name("area-do-terreno"), "area_do_terreno");
        assert_eq!(normalize_field_name("numero-de-torres"), "numero_de_torres");
        assert_eq!(normalize_field_name("altura-da-torre"), "altura_da_torre");
        assert_eq!(normalize_field_name("area-da-torre"), "area_da_torre");
        assert_eq!(normalize_field_name("area-de-lazer"), "area_de_lazer");
        assert_eq!(normalize_field_name("construtora"), "construtora");
        assert_eq!(normalize_field_name("cidade"), "cidade");
    }
}

