pub struct FileHelper {}

impl FileHelper {
    pub async fn get_content_string(file: &String) -> String {
        if file.starts_with("http://") || file.starts_with("https://") {
            let content = crate::utils::http::Http::get(file).await.unwrap();
            String::from_utf8(content.content.to_vec()).unwrap()
        } else {
            std::fs::read_to_string(file).unwrap()
        }
    }

    pub async fn get_content_string_result(file: &String) -> Result<String, String> {
        if file.starts_with("http://") || file.starts_with("https://") {
            let content = crate::utils::http::Http::get(file).await.unwrap();
            String::from_utf8(content.content.to_vec()).map_err(|e| e.to_string())
        } else {
            std::fs::read_to_string(file).map_err(|e| e.to_string())
        }
    }
}