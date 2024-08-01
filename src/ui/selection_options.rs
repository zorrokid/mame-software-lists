#[derive(Clone)]
pub struct SelectionOptions<T> {
    pub selected: Option<T>,
    pub items: Vec<T>,
}
