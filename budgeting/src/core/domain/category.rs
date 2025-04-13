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
