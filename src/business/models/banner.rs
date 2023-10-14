pub struct Banner {
    pub uuid: String,
    pub filename: String,
}

impl Banner {
    pub fn new(uuid: String, filename: String) -> Self {
        Self { uuid, filename }
    }
}
