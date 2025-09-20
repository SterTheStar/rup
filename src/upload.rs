use anyhow::Result;

pub async fn upload_files(files: Vec<String>, config: &crate::config::Config) -> Result<()> {
    match config.api.api_type.as_str() {
        "litterbox" => crate::api::litterbox::upload_files(files, config).await,
        "temp_sh" => crate::api::temp_sh::upload_files(files, config).await,
        "uguu" => crate::api::uguu::upload_files(files, config).await,
        "bashupload" => crate::api::bashupload::upload_files(files, config).await,
        _ => Err(anyhow::anyhow!("Unsupported API type: {}", config.api.api_type)),
    }
}
