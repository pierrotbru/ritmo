include!(concat!(env!("OUT_DIR"), "/constants.rs"));

pub fn format_column(table: &str, column: &str) -> String {
    format!("{}.{}", table, column)
}