use std::{fs::File, io::Read, sync::Arc};

use axum::{extract::{Multipart, Path}, response::IntoResponse, Extension};
use http::{header, StatusCode};
use infer::{Infer, MatcherType};
use uuid::Uuid;

use crate::{enums::response_enum::ResponseErrorMessage, structs::{account_assets::AccountAssets, pool_conn_struct::PoolConnectionState}, utils::storages::Storage};


pub async fn storage (
        Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
        mut multipart: Multipart,
    ) -> impl IntoResponse {


    if let Some(field) = multipart.next_field().await.unwrap() {
        let get_file_bytes = match field.bytes().await {
            Ok(result) => result.to_owned(),
            Err(_) => {
                return (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::InvalidRequest));
            }
        };

        let verify_file = Storage::verify_file_mime_type(&get_file_bytes);

        if let Some(mime_type) = verify_file {
            if mime_type.ne(&MatcherType::Image) {
                return (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::InvalidRequest));
            }
            let host_server = dotenvy::var("HOST_SERVER").expect("Missing Host Server ENV");

            let file_ext = Storage::get_file_ext(&get_file_bytes);

            let file_information = AccountAssets {
                file_ext: file_ext.to_owned(),
                file_hash: Storage::hash_file(&get_file_bytes),
                file_name: format!("{}.{}", Uuid::new_v4().to_string().replace("-", ""), file_ext),
                file_size: (&get_file_bytes.len().to_owned()).to_string(),
                file_url: format!("{}/storage/image", host_server)
            };

            match sqlx::query("INSERT INTO account_assets (file_name, file_ext, file_url, file_size, file_hash) VALUES (?, ?, ?, ?, ?)")
                .bind(&file_information.file_name)
                .bind(file_information.file_ext)
                .bind(file_information.file_url)
                .bind(file_information.file_size)
                .bind(file_information.file_hash)
                .execute(&sql_pool.connection.to_owned()).await {
                    Ok(result) => {
                        let result = result.last_insert_id();

                        Storage::create_storage_if_not_exists();
                        let foldername = "images";
                        Storage::create_folder(&foldername);
                        Storage::create_file(foldername, &file_information.file_name, &get_file_bytes);

                        return (StatusCode::OK, format!("{}", result));
                    },
                    Err(_) => {

                        return (StatusCode::NO_CONTENT, format!("{:?}", ResponseErrorMessage::DataNotFound));
                    }
                }
        }

        return (StatusCode::NO_CONTENT, format!("{:?}", ResponseErrorMessage::DataNotFound));
    } else {
        return (StatusCode::NO_CONTENT, format!("{:?}", ResponseErrorMessage::DataNotFound));
    }

}

pub async fn serve_image (
    Path(image_name): Path<String>
) -> impl IntoResponse {

    let file_path = format!("Storage/images/{}", image_name);

    let mut file = File::open(file_path).unwrap();

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();

    let infer = Infer::new();

    let mime_type = match infer.get(&buffer) {
        Some(info) => info.mime_type(),
        None => return (StatusCode::UNSUPPORTED_MEDIA_TYPE, "Unsupported file type").into_response(),
    };
    ([(header::CONTENT_TYPE, mime_type.to_string())], buffer).into_response()
}