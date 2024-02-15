use std::fs;
use std::path::Path;

/// Custom error type for environment loading operations.
#[derive(Debug, PartialEq)]
pub enum EnvLoaderError {
    /// Error variant indicating the .env file was not found.
    FileNotFound,
}

/// Module for environment loading functionality.
pub mod load {
    use crate::{EnvLoaderError, file_exists};
    use dotenv::from_filename;

    /// Loads environment variables from a specified .env file.
    ///
    /// # Arguments
    ///
    /// * `path` - A string slice that holds the path to the .env file, including file itself.
    ///
    /// # Returns
    ///
    /// A result indicating success (`Ok(())`) or containing an `EnvLoaderError` on failure.
    pub fn load(path: &str) -> Result<(), EnvLoaderError> {
        match file_exists(path) {
            Ok(_) => {
                from_filename(path).ok();
                Ok(())
            },
            Err(err) => Err(err),
        }
    }
}

/// Checks if a file exists at the given path.
///
/// # Arguments
///
/// * `path_str` - A string slice that holds the path to the file.
///
/// # Returns
///
/// A result indicating success (`Ok(())`) if the file exists, or an `EnvLoaderError::FileNotFound` if not.
fn file_exists(path_str: &str) -> Result<(), EnvLoaderError> {
    let path = Path::new(path_str);
    if fs::metadata(path).is_ok() {
        Ok(())
    } else {
        Err(EnvLoaderError::FileNotFound)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir; // You might need to add the `tempfile` crate for this.

    #[test]
    fn test_file_exists_positive() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file_exists.txt");
        File::create(file_path.clone()).unwrap();

        assert!(file_exists(file_path.to_str().unwrap()).is_ok());
    }

    #[test]
    fn test_file_exists_negative() {
        let non_existent_path = "non_existent_file.txt";
        assert!(file_exists(non_existent_path).is_err());
    }
}

#[cfg(test)]
mod env_loader_tests {
    use std::env;
    use super::*;
    use crate::load::load;

    #[test]
    fn test_load_env_file_exists() {
        let path = ".env";
        match load(path) {
            Ok(_) => assert_eq!(env::var("TEST_VALUE").unwrap(), String::from("exists")),
            Err(err) => assert_eq!(err, EnvLoaderError::FileNotFound)
        }
    }

    #[test]
    fn test_load_env_file_not_found() {
        let non_existent_path = "non_existent_env_file.env";
        assert_eq!(load(non_existent_path), Err(EnvLoaderError::FileNotFound));
    }
}
