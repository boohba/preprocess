use preprocess::{Config, File, Preprocess};

pub struct HashPreprocessor;

impl HashPreprocessor {
    fn new() -> Self {
        Self
    }
}

impl Preprocess for HashPreprocessor {
    fn preprocess(&self, path: &mut File) {
        let hash = hex::encode(crc32fast::hash(&path.content).to_le_bytes());

        if let Some(ext) = path.source.extension() {
            path.dest.set_extension(format!("{}.{}", hash, ext.to_str().unwrap()));
        } else {
            path.dest.set_extension(hash);
        }
    }
}

pub trait ConfigHashPreprocessorExt {
    fn hash(self) -> Config;
}

impl ConfigHashPreprocessorExt for Config {
    fn hash(self) -> Config {
        self.add_preprocessor(Box::new(HashPreprocessor::new()))
    }
}