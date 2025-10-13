use crate::SECRET_MANAGER;
use axum::extract::multipart::Field;
use azure_storage::prelude::*;
use azure_storage_blobs::prelude::*;
use once_cell::sync::Lazy;
use std::path::Path;
use tracing::{debug, error};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct UploadMetadata {
    pub extension: Option<String>,
    pub content_type: Option<String>,
}

pub struct AzureStorageConfig {
    account: String,
    container: String,
    storage_credentials: StorageCredentials,
}

impl AzureStorageConfig {
    pub fn new() -> AzureStorageConfig {
        let account = SECRET_MANAGER.get("STORAGE_ACCOUNT");
        let access_key = SECRET_MANAGER.get("STORAGE_ACCESS_KEY");
        let container = SECRET_MANAGER.get("STORAGE_CONTAINER");
        let storage_credentials =
            StorageCredentials::access_key(account.clone(), access_key.clone());
        AzureStorageConfig {
            account,
            container,
            storage_credentials,
        }
    }

    pub async fn upload_stream(
        &self,
        mut field: Field<'_>,
        metadata: UploadMetadata,
    ) -> Result<String, String> {
        let filename = field.file_name().map(str::to_string);
        let inline_content_type = field.content_type().map(str::to_string);

        let extension = Self::determine_extension(
            metadata.extension.as_ref(),
            filename.as_ref(),
            inline_content_type.as_ref(),
        );
        let content_type = Self::determine_content_type(
            metadata.content_type.as_ref(),
            inline_content_type.as_ref(),
            extension.as_str(),
        );

        let blob_name = format!("{}.{}", generate_name(), extension);
        let blob_client = ClientBuilder::new(&self.account, self.storage_credentials.clone())
            .blob_client(&self.container, &blob_name);

        let mut block_list = BlockList::default();
        let mut block_index: u64 = 0;

        loop {
            match field.chunk().await {
                Ok(Some(chunk)) => {
                    if chunk.is_empty() {
                        continue;
                    }

                    block_index += 1;
                    let block_id_raw = format!("{:032}", block_index);
                    let block_id = BlockId::new(block_id_raw.clone());

                    blob_client
                        .put_block(block_id.clone(), chunk.clone())
                        .await
                        .map_err(|err| {
                            error!(
                                "Failed to stage block {} for blob `{}`: {:?}",
                                block_id_raw, blob_name, err
                            );
                            err.to_string()
                        })?;

                    block_list
                        .blocks
                        .push(BlobBlockType::new_uncommitted(block_id));
                }
                Ok(None) => break,
                Err(err) => {
                    error!("Failed to read multipart chunk for `{blob_name}`: {}", err);
                    return Err(err.to_string());
                }
            }
        }

        if block_index == 0 {
            return Err("Uploaded file was empty".to_string());
        }

        blob_client
            .put_block_list(block_list)
            .content_type(content_type.clone())
            .await
            .map_err(|err| {
                error!(
                    "Failed to commit block list for blob `{}`: {:?}",
                    blob_name, err
                );
                err.to_string()
            })?;

        let blob_url = format!(
            "https://{}.blob.core.windows.net/{}/{}",
            self.account, self.container, blob_name
        );
        debug!(
            "Uploaded blob `{}` with content type `{}`",
            blob_name, content_type
        );
        Ok(blob_url)
    }

    fn determine_extension(
        provided_extension: Option<&String>,
        filename: Option<&String>,
        inline_content_type: Option<&String>,
    ) -> String {
        provided_extension
            .and_then(|ext| Self::normalize_extension(ext))
            .or_else(|| filename.and_then(|name| Self::extension_from_filename(name)))
            .or_else(|| inline_content_type.and_then(|ct| Self::extension_from_mime(ct.as_str())))
            .unwrap_or_else(|| "bin".to_string())
    }

    fn determine_content_type(
        provided: Option<&String>,
        inline: Option<&String>,
        extension: &str,
    ) -> String {
        provided
            .and_then(|ct| Self::sanitize_content_type(ct))
            .or_else(|| inline.and_then(|ct| Self::sanitize_content_type(ct)))
            .unwrap_or_else(|| Self::extension_to_mime(extension).to_string())
    }

    fn normalize_extension(value: &str) -> Option<String> {
        let trimmed = value.trim().trim_start_matches('.');
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.to_ascii_lowercase())
        }
    }

    fn extension_from_filename(filename: &str) -> Option<String> {
        Path::new(filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .and_then(Self::normalize_extension)
    }

    fn extension_from_mime(mime: &str) -> Option<String> {
        match mime {
            "image/jpeg" | "image/jpg" => Some("jpg".to_string()),
            "image/png" => Some("png".to_string()),
            "image/gif" => Some("gif".to_string()),
            "image/svg+xml" => Some("svg".to_string()),
            "image/webp" => Some("webp".to_string()),
            "application/pdf" => Some("pdf".to_string()),
            "text/plain" => Some("txt".to_string()),
            "application/json" => Some("json".to_string()),
            "application/xml" => Some("xml".to_string()),
            "video/mp4" => Some("mp4".to_string()),
            "audio/mpeg" => Some("mp3".to_string()),
            _ => None,
        }
    }

    fn sanitize_content_type(value: &str) -> Option<String> {
        let trimmed = value.trim();
        if trimmed.is_empty() || !trimmed.contains('/') {
            None
        } else {
            Some(trimmed.to_string())
        }
    }

    fn extension_to_mime(extension: &str) -> &'static str {
        match extension {
            "jpg" | "jpeg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            "svg" | "svgz" => "image/svg+xml",
            "webp" => "image/webp",
            "pdf" => "application/pdf",
            "txt" => "text/plain",
            "json" => "application/json",
            "xml" => "application/xml",
            "mp4" => "video/mp4",
            "mp3" => "audio/mpeg",
            _ => "application/octet-stream",
        }
    }
}
fn generate_name() -> String {
    let id = Uuid::new_v4();
    id.to_string()
}

pub static AZURESTORAGECONFIG: Lazy<AzureStorageConfig> = Lazy::new(|| AzureStorageConfig::new());
