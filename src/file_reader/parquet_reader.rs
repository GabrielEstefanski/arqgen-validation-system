use crate::models::empreendimento::Empreendimento;
use crate::models::field_converter::normalize_field_name;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use std::error::Error;
use std::fs::File;
use std::collections::HashMap;

pub fn read_parquet(path: &str) -> Result<Vec<Empreendimento>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = SerializedFileReader::new(file)?;
    let mut results = Vec::new();

    let metadata = reader.metadata();
    let schema = metadata.file_metadata().schema();
    let column_mapping = create_column_mapping(schema);

    for row in reader.get_row_iter(None)? {
        let empreendimento = deserialize_row(&row, &column_mapping)?;
        results.push(empreendimento);
    }

    Ok(results)
}

fn create_column_mapping(schema: &parquet::schema::types::Type) -> HashMap<String, usize> {
    let mut mapping = HashMap::new();
    
    if let parquet::schema::types::Type::GroupType { fields, .. } = schema {
        for (index, field) in fields.iter().enumerate() {
            let normalized_name = normalize_field_name(field.name());
            mapping.insert(normalized_name.to_string(), index);
        }
    }
    
    mapping
}

fn deserialize_row(row: &parquet::record::Row, column_mapping: &HashMap<String, usize>) -> Result<Empreendimento, Box<dyn Error>> {
    let get_field = |field_name: &str| -> Result<String, Box<dyn Error>> {
        let index = column_mapping.get(field_name)
            .ok_or_else(|| format!("Campo '{}' não encontrado no Parquet", field_name))?;
        Ok(row.get_string(*index)?.to_string())
    };

    let get_numeric_field = |field_name: &str| -> Result<f64, Box<dyn Error>> {
        let index = column_mapping.get(field_name)
            .ok_or_else(|| format!("Campo '{}' não encontrado no Parquet", field_name))?;
        Ok(row.get_double(*index)?)
    };

    let get_u32_field = |field_name: &str| -> Result<u32, Box<dyn Error>> {
        let index = column_mapping.get(field_name)
            .ok_or_else(|| format!("Campo '{}' não encontrado no Parquet", field_name))?;
        Ok(row.get_int(*index)? as u32)
    };

    let area_de_lazer = if column_mapping.contains_key("area_de_lazer") {
        match get_numeric_field("area_de_lazer") {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    } else {
        None
    };

    Ok(Empreendimento {
        construtora: get_field("construtora")?,
        cidade: get_field("cidade")?,
        area_do_terreno: get_numeric_field("area_do_terreno")?,
        numero_de_torres: get_u32_field("numero_de_torres")?,
        altura_da_torre: get_numeric_field("altura_da_torre")?,
        area_da_torre: get_numeric_field("area_da_torre")?,
        area_de_lazer,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use parquet::schema::parser::parse_message_type;

    #[test]
    fn test_column_mapping_creation() {
        let schema = "
            message empreendimento {
                REQUIRED BINARY construtora (UTF8);
                REQUIRED BINARY cidade (UTF8);
                REQUIRED DOUBLE area_do_terreno;
                REQUIRED INT32 numero_de_torres;
                REQUIRED DOUBLE altura_da_torre;
                REQUIRED DOUBLE area_da_torre;
                OPTIONAL DOUBLE area_de_lazer;
            }
        ";
        
        let message_type = parse_message_type(schema).unwrap();
        let mapping = create_column_mapping(&message_type);
        
        assert!(mapping.contains_key("construtora"));
        assert!(mapping.contains_key("cidade"));
        assert!(mapping.contains_key("area_do_terreno"));
        assert!(mapping.contains_key("numero_de_torres"));
        assert!(mapping.contains_key("altura_da_torre"));
        assert!(mapping.contains_key("area_da_torre"));
        assert!(mapping.contains_key("area_de_lazer"));
    }

    #[test]
    fn test_column_mapping_with_kebab_case() {
        let schema = "
            message empreendimento {
                REQUIRED BINARY construtora (UTF8);
                REQUIRED BINARY cidade (UTF8);
                REQUIRED DOUBLE area-do-terreno;
                REQUIRED INT32 numero-de-torres;
                REQUIRED DOUBLE altura-da-torre;
                REQUIRED DOUBLE area-da-torre;
                OPTIONAL DOUBLE area-de-lazer;
            }
        ";
        
        let message_type = parse_message_type(schema).unwrap();
        let mapping = create_column_mapping(&message_type);
        
        assert!(mapping.contains_key("area_do_terreno"));
        assert!(mapping.contains_key("numero_de_torres"));
        assert!(mapping.contains_key("altura_da_torre"));
        assert!(mapping.contains_key("area_da_torre"));
        assert!(mapping.contains_key("area_de_lazer"));
    }
}
