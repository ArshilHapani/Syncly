pub mod state;

use std::{error::Error, fs::File, io::Read, path::PathBuf};

pub fn hash_file(path: &PathBuf) -> Result<String, Box<dyn Error>> {
    let file_result = File::open(path);
    match file_result {
        Ok(mut file) => {
            let mut buffer = vec![0; 1024];
            let mut hasher = blake3::Hasher::new();
            loop {
                let bytes = file.read(&mut buffer).unwrap_or(0);
                if bytes == 0 {
                    break;
                }
                hasher.update(&buffer[..bytes]);
            }
            Ok(hasher.finalize().to_hex().to_string())
        }
        Err(e) => Err(Box::new(e)),
    }
}
// TODO implement further snapshot functions

#[cfg(test)]
mod hash_file_test {
    use super::*;
    use std::time::SystemTime;

    /// Try changing the content of `Cargo.toml` file inside **snapshots** dir and rerun the test!
    #[test]
    fn test_hash_file() {
        let st = SystemTime::now();
        let file = PathBuf::from("Cargo.toml"); // snapshots/Cargo.toml file
        let hash = hash_file(&file).unwrap_or("".to_string());
        println!("{}", hash);
        assert_eq!(
            hash,
            "abc318a7a48ea6169c95461f05ac28f133712f08af64fc2b970b3a1bacd8ff30"
        );
        let st_end = SystemTime::now();
        println!("{:?}", st);
        println!("{:?}", st_end);

        assert!(st < st_end);
    }
}
