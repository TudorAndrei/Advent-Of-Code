pub struct File {
    size: u32,
    name: String,
}
pub struct Dir {
    name: String,
    files: Vec<File>,
}
impl Dir {
    pub fn new(name: String) -> Self {
        Dir {
            name,
            files: Vec::new(),
        }
    }
    pub fn insert(&mut self, file: File) {
        self.files.push(file);
    }
}
