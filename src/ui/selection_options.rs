#[derive(Clone)]
pub struct SelectionOptions<T> {
    pub selected: Option<T>,
    pub items: Vec<T>,
}

impl<T> Default for SelectionOptions<T> {
    fn default() -> Self {
        Self {
            selected: None,
            items: Vec::new(),
        }
    }
}
