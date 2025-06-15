use ddd::ValueObject;

#[derive(ValueObject, Debug)]
pub struct Category {
    #[field]
    name: String,
}

impl Category {
    fn new(name: String) -> Self {
        Self { name }
    }
}

impl From<String> for Category {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}

impl From<&str> for Category {
    fn from(value: &str) -> Self {
        let value: String = value.into();
        value.into()
    }
}
