#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    #[test]
    fn test_ensure_file_creates_new_file_and_dirs() {
        // Create an isolated temporary directory
        let temp_dir = tempdir().expect("Failed to create temp dir");

        // Run the function against the temp directory
        let config_path = whatnow::ensure_file(temp_dir.path()).expect("Should succeed");

        // Assert that the file now exists on disk
        assert!(config_path.exists());
        assert!(config_path.is_file());

        // Assert the path structure (~/.config/whatnow)
        assert!(config_path.ends_with(".config/whatnow"));
    }

    #[test]
    fn test_ensure_file_idempotent_if_already_exists() {
        let temp_dir = tempdir().expect("Failed to create temp dir");

        // First call creates it
        let path1 = whatnow::ensure_file(temp_dir.path()).expect("First run failed");

        // Second call detects it already exists and returns early
        let path2 = whatnow::ensure_file(temp_dir.path()).expect("Second run failed");

        assert_eq!(path1, path2);
        assert!(path2.exists());
    }
}
