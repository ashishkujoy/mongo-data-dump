use std::{fs::File, io::Write};

pub(crate) struct JsonFileWriter {
    file: File,
    is_first_record: Option<bool>,
}

impl JsonFileWriter {
    pub(crate) fn new(file_name: String) -> Self {
        JsonFileWriter {
            file: File::create(file_name).unwrap(),
            is_first_record: Some(true),
        }
    }

    pub(crate) fn write<V>(&mut self, value: &V)
    where
        V: serde::ser::Serialize,
    {
        match &self.is_first_record {
            Some(_) => self.file.write(b"[").unwrap(),
            None => self.file.write(b",").unwrap(),
        };
        serde_json::to_writer(&self.file, &value).unwrap();
        self.is_first_record.take();
    }
}

impl Drop for JsonFileWriter {
    fn drop(&mut self) {
        self.file.write(b"]").unwrap();
        drop(&mut self.file);
    }
}
