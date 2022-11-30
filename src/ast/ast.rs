#[derive(Debug)]
pub enum Ast {
    Map(Vec<Field>),
}

#[derive(Debug)]
pub struct Field {
    source: String,
    cast: Option<FieldCast>,
    alias: Option<String>,
}

#[derive(Debug)]
pub enum FieldCast {
    Str,
    Int,
    /// Represents a timestamp with an optionnal date format.
    Timestamp(Option<String>),
}
