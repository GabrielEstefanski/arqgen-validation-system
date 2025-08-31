pub mod csv_reader;
pub mod json_reader;
pub mod parquet_reader;
pub mod xml_reader;

pub use csv_reader::read_csv;
pub use json_reader::read_json;
pub use parquet_reader::read_parquet;
pub use xml_reader::read_xml;

pub enum FileType {
    Csv,
    Json,
    Parquet,
    Xml,
}

pub fn read_file(
    file_type: FileType,
    path: &str,
) -> Result<Vec<crate::models::empreendimento::Empreendimento>, Box<dyn std::error::Error>> {
    match file_type {
        FileType::Csv => read_csv(path),
        FileType::Json => read_json(path),
        FileType::Parquet => read_parquet(path),
        FileType::Xml => read_xml(path),
    }
}
