#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ConfigurationValue {
    Boolean(bool),
    Number(i64),
    String(String),
    Identifier(Id),
    Strings(Vec<String>),
    Identifiers(Vec<Id>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Configuration {
    parent: Option<Box<Configuration>>,
    this: HashMap<Id, ConfigurationValue, ::ahash::RandomState>,
}
