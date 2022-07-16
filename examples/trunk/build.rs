use preprocess_hash::ConfigHashPreprocessorExt;

fn main() {
    preprocess::configure()
        .hash()
        .run();
}