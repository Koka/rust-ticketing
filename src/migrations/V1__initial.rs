pub fn migration() -> String {
    include_str!("sql/initial_schema.sql").to_owned()
}
