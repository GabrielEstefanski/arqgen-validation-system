use crate::models::empreendimento::Empreendimento;
use serde_json::from_str;
use std::error::Error;
use std::fs;

pub fn read_json(path: &str) -> Result<Vec<Empreendimento>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let results: Vec<Empreendimento> = from_str(&data)?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn create_temp_json(content: &str) -> NamedTempFile {
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        writeln!(temp_file, "{}", content).unwrap();
        temp_file
    }

    #[test]
    fn test_read_json_kebab_case() {
        let json_content = r#"[
            {
                "construtora": "Alpha",
                "cidade": "São Paulo",
                "area-do-terreno": 1000.0,
                "numero-de-torres": 2,
                "altura-da-torre": 25.0,
                "area-da-torre": 300.0,
                "area-de-lazer": 150.0
            }
        ]"#;
        
        let temp_file = create_temp_json(json_content);
        let result = read_json(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 1);
        
        let emp = &empreendimentos[0];
        assert_eq!(emp.construtora, "Alpha");
        assert_eq!(emp.cidade, "São Paulo");
        assert_eq!(emp.area_do_terreno, 1000.0);
        assert_eq!(emp.numero_de_torres, 2);
        assert_eq!(emp.altura_da_torre, 25.0);
        assert_eq!(emp.area_da_torre, 300.0);
        assert_eq!(emp.area_de_lazer, Some(150.0));
    }

    #[test]
    fn test_read_json_snake_case() {
        let json_content = r#"[
            {
                "construtora": "Beta",
                "cidade": "Rio de Janeiro",
                "area_do_terreno": 800.0,
                "numero_de_torres": 3,
                "altura_da_torre": 30.0,
                "area_da_torre": 250.0,
                "area_de_lazer": 100.0
            }
        ]"#;
        
        let temp_file = create_temp_json(json_content);
        let result = read_json(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 1);
        
        let emp = &empreendimentos[0];
        assert_eq!(emp.construtora, "Beta");
        assert_eq!(emp.cidade, "Rio de Janeiro");
        assert_eq!(emp.area_do_terreno, 800.0);
        assert_eq!(emp.numero_de_torres, 3);
        assert_eq!(emp.altura_da_torre, 30.0);
        assert_eq!(emp.area_da_torre, 250.0);
        assert_eq!(emp.area_de_lazer, Some(100.0));
    }

    #[test]
    fn test_read_json_mixed_case() {
        let json_content = r#"[
            {
                "construtora": "Gamma",
                "cidade": "Brasília",
                "area-do-terreno": 1200.0,
                "numero_de_torres": 4,
                "altura-da-torre": 35.0,
                "area_da_torre": 400.0,
                "area-de-lazer": 200.0
            }
        ]"#;
        
        let temp_file = create_temp_json(json_content);
        let result = read_json(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 1);
        
        let emp = &empreendimentos[0];
        assert_eq!(emp.construtora, "Gamma");
        assert_eq!(emp.cidade, "Brasília");
        assert_eq!(emp.area_do_terreno, 1200.0);
        assert_eq!(emp.numero_de_torres, 4);
        assert_eq!(emp.altura_da_torre, 35.0);
        assert_eq!(emp.area_da_torre, 400.0);
        assert_eq!(emp.area_de_lazer, Some(200.0));
    }

    #[test]
    fn test_read_json_without_area_de_lazer() {
        let json_content = r#"[
            {
                "construtora": "Delta",
                "cidade": "Salvador",
                "area-do-terreno": 600.0,
                "numero-de-torres": 1,
                "altura-da-torre": 20.0,
                "area-da-torre": 500.0
            }
        ]"#;
        
        let temp_file = create_temp_json(json_content);
        let result = read_json(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 1);
        
        let emp = &empreendimentos[0];
        assert_eq!(emp.construtora, "Delta");
        assert_eq!(emp.cidade, "Salvador");
        assert_eq!(emp.area_do_terreno, 600.0);
        assert_eq!(emp.numero_de_torres, 1);
        assert_eq!(emp.altura_da_torre, 20.0);
        assert_eq!(emp.area_da_torre, 500.0);
        assert_eq!(emp.area_de_lazer, None);
    }

    #[test]
    fn test_read_json_multiple_empreendimentos() {
        let json_content = r#"[
            {
                "construtora": "Alpha",
                "cidade": "São Paulo",
                "area-do-terreno": 1000.0,
                "numero-de-torres": 2,
                "altura-da-torre": 25.0,
                "area-da-torre": 300.0,
                "area-de-lazer": 150.0
            },
            {
                "construtora": "Beta",
                "cidade": "Rio de Janeiro",
                "area_do_terreno": 800.0,
                "numero_de_torres": 3,
                "altura_da_torre": 30.0,
                "area_da_torre": 250.0,
                "area_de_lazer": 100.0
            }
        ]"#;
        
        let temp_file = create_temp_json(json_content);
        let result = read_json(temp_file.path().to_str().unwrap());
        
        assert!(result.is_ok());
        let empreendimentos = result.unwrap();
        assert_eq!(empreendimentos.len(), 2);
        
        let emp1 = &empreendimentos[0];
        assert_eq!(emp1.construtora, "Alpha");
        assert_eq!(emp1.cidade, "São Paulo");
        
        let emp2 = &empreendimentos[1];
        assert_eq!(emp2.construtora, "Beta");
        assert_eq!(emp2.cidade, "Rio de Janeiro");
    }
}
