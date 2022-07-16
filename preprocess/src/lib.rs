use std::fs::{create_dir_all, read, write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub struct File {
    pub source: PathBuf,
    pub dest: PathBuf,
    pub content: Vec<u8>,
}

impl File {
    fn from(source: PathBuf, config: &Config) -> Self {
        let mut dest = PathBuf::from(&config.dist);
        dest.push(source.strip_prefix(&config.assets).unwrap());
        let content = read(&source).expect("error reading file");
        Self { source, dest, content }
    }
}

pub trait Preprocess {
    fn preprocess(&self, path: &mut File);
}

pub struct Config {
    pub dist: PathBuf,
    pub assets: PathBuf,
    pub prefix: String,
    preprocessors: Vec<Box<dyn Preprocess>>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            dist: PathBuf::from("dist"),
            assets: PathBuf::from("src/assets"),
            prefix: String::from("/"),
            preprocessors: Vec::new(),
        }
    }

    pub fn run(self) {
        run(self)
    }

    pub fn set_dist(mut self, dist: PathBuf) -> Self {
        self.dist = dist;
        self
    }

    pub fn set_assets(mut self, assets: PathBuf) -> Self {
        self.assets = assets;
        self
    }

    pub fn set_prefix(mut self, prefix: String) -> Self {
        self.prefix = prefix;
        self
    }

    pub fn add_preprocessor(mut self, preprocessor: Box<dyn Preprocess>) -> Self {
        self.preprocessors.push(preprocessor);
        self
    }
}

pub fn configure() -> Config {
    Config::new()
}

const VARIABLE_NAME_ALLOWED_CHARS: [char; 37] = ['Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Z', 'X', 'C', 'V', 'B', 'N', 'M', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '_'];

pub fn run(config: Config) {
    println!("cargo:rerun-if-changed={}", config.assets.to_str().unwrap());

    if !config.assets.exists() {
        return;
    }

    create_dir_all(&config.dist).expect("error creating destination dir");

    let mut result = Vec::new();

    for entry in WalkDir::new(&config.assets) {
        let path = entry.unwrap().into_path();

        if path.is_dir() {
            continue;
        }

        let mut file = File::from(path, &config);

        for preprocessor in &config.preprocessors {
            preprocessor.preprocess(&mut file);
        }

        write(&file.dest, &file.content).expect("error writing file");

        let mut variable_name = String::new();

        for char in file.source.file_name().unwrap().to_str().unwrap().chars() {
            let char = char.to_uppercase().next().unwrap();

            if char == '.' || char == ' ' {
                variable_name.push('_');
            } else if VARIABLE_NAME_ALLOWED_CHARS.contains(&char) {
                variable_name.push(char);
            }
        }

        result.extend_from_slice(b"\npub const ");
        result.extend_from_slice(variable_name.as_bytes());
        result.extend_from_slice(b": &str = \"");
        result.extend_from_slice(config.prefix.as_bytes());
        result.extend_from_slice(file.dest.file_name().unwrap().to_str().unwrap().as_bytes());
        result.extend_from_slice(b"\";");
    }

    write(Path::new(&std::env::var("OUT_DIR").unwrap()).join("assets.rs"), result).expect("error writing assets.rs");
}