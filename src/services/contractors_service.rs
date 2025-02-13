use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use crate::{enums::response_enum::{ResponseErrorMessage, ResponseOkMessage}, structs::{basic_struct::RequestById, contractors_struct::{Contractors, ReturnContractors, UpdateContractorsById}, pool_conn_struct::PoolConnectionState}};

pub async fn add_contractors (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<Contractors>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnContractors, sqlx::Error> = sqlx::query_as("SELECT * FROM contractors WHERE contractor_name LIKE ?")
        .bind(format!("{}", &request.contractor_contact_name)).fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO contractors (contractor_name, contractor_email, contractor_address_street, contractor_address_barangay, contractor_address_municipality, contractor_address_province, contractor_description, contractor_contact_name, contractor_contact_position, contractor_contract_number) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(request.contractor_name)
            .bind(request.contractor_email)
            .bind(request.contractor_address_street)
            .bind(request.contractor_address_barangay)
            .bind(request.contractor_address_municipality)
            .bind(request.contractor_address_province)
            .bind(request.contractor_description)
            .bind(request.contractor_contact_name)
            .bind(request.contractor_contact_position)
            .bind(request.contractor_contract_number).execute(&sql_pool.connection.to_owned()).await {
                Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
                Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
            }
}

pub async fn update_contractors_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateContractorsById>
) -> impl IntoResponse {

    let check_if_exists: Result<ReturnContractors, sqlx::Error> = sqlx::query_as("SELECT * FROM contractors WHERE id = ?")
        .bind(&request.id).fetch_one(&sql_pool.connection).await;

    if let Err(_) = check_if_exists {
        return (StatusCode::BAD_REQUEST, format!("{:?}", ResponseErrorMessage::DataNotFound));
    }

    match sqlx::query("UPDATE contractors 
        SET 
            contractor_name = ?, 
            contractor_email = ?, 
            contractor_address_street = ?, 
            contractor_address_barangay = ?,
            contractor_address_municipality = ?,
            contractor_address_province = ?,
            contractor_description = ?,
            contractor_contact_name = ?,
            contractor_contact_position = ?,
            contractor_contract_number = ? WHERE id = ?")
                .bind(request.contractor_name)
                .bind(request.contractor_email)
                .bind(request.contractor_address_street)
                .bind(request.contractor_address_barangay)
                .bind(request.contractor_address_municipality)
                .bind(request.contractor_address_province)
                .bind(request.contractor_description)
                .bind(request.contractor_contact_name)
                .bind(request.contractor_contact_position)
                .bind(request.contractor_contract_number)
                .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
                    Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
                    Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
                }

}

pub async fn delete_contractors_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM contractors WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
        }

}

pub async fn get_all_contractors (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM contractors ORDER BY id DESC").fetch_all(&sql_pool.connection.to_owned()).await {
        Ok(result) => {
            let result: Vec<ReturnContractors> = result;
            (StatusCode::OK, Json(result))
        },
        Err(_) => {
            (StatusCode::OK, Json(Vec::default()))
        }
    }

}

