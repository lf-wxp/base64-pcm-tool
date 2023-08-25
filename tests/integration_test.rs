use std::fs;
use std::path::Path;

#[test]
fn test_with_external_file() {
    let test_data_path = Path::new("tests/data/pcm_test.txt");
    let test_data = fs::read_to_string(test_data_path).expect("Failed to read test data file");
    // 使用 test_data 进行测试
}
