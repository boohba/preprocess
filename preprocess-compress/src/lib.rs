use std::io::Write;
use preprocess::{Config, File, Preprocess};

pub struct CompressPreprocessor;

impl CompressPreprocessor {
    pub fn new() -> Self {
        Self
    }
}

impl Preprocess for CompressPreprocessor {
    fn preprocess(&self, path: &mut File) {
        let mut result = Vec::new();

        brotli::CompressorWriter::new(&mut result, 0, 11, 22)
            .write_all(&path.content)
            .expect("error compressing file content");

        path.content = result;
    }
}

pub trait ConfigCompressPreprocessorExt {
    fn compress(self) -> Self;
}

impl ConfigCompressPreprocessorExt for Config {
    fn compress(self) -> Self {
        self.add_preprocessor(Box::new(CompressPreprocessor::new()))
    }
}