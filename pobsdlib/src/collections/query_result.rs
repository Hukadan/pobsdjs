#[derive(Serialize, Clone, Default, Debug)]
pub struct QueryResult<T> {
    pub count: usize,
    pub items: Vec<T>,
}

impl<T: PartialOrd> QueryResult<T> {
    pub fn new(mut items: Vec<T>) -> Self {
        items.sort_by(|a, b| a.partial_cmp(b).unwrap());
        Self {
            count: items.len(),
            items,
        }
    }
}