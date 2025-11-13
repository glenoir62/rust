// Modules publics
pub mod models;
pub mod reader;
pub mod writer;
pub mod appender;
pub mod updater;

// Ré-exporter les types principaux pour un usage plus simple
pub use models::Person;

// Ré-exporter les fonctions principales
pub use reader::{read_csv_raw, read_csv_deserialize};
pub use writer::{write_csv, write_csv_serialize, get_sample_data};
pub use appender::append_to_csv;
pub use updater::{update_csv, update_age_by_name, delete_by_name};
