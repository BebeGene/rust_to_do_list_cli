#[derive(Debug)]
pub struct Task {
    pub id: usize,
    pub name: String,
    pub status: String, // "pending" or "completed"
}

impl Task {
    pub fn new(id: usize, name: &str) -> Self {
        Task {
            id,
            name: name.to_string(),
            status: "pending".to_string(),
        }
    }

    pub fn update_status(&mut self, status: &str) {
        self.status = status.to_string();
    }
}
