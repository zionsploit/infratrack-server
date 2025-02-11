use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use http::StatusCode;

use crate::{
    enums::response_enum::{ResponseErrorMessage, ResponseOkMessage}, 
    structs::{
        basic_struct::RequestById, 
        pool_conn_struct::PoolConnectionState, 
        project_interface::{Barangays, Categories, Incharge, ProjectScope, ProjectTypes, ReturnBarangays, ReturnCategories, ReturnIncharge, ReturnProjectScope, ReturnProjectTypes, ReturnSectors, ReturnSourceOfFunds, ReturnSustainableDevelopmentGoals, ReturnUnitOfMeasurements, Sectors, SourceOfFunds, SustainableDevelopmentGoals, UnitOfMeasurements, UpdateBarangaysById, UpdateCategoriesById, UpdateInchargeById, UpdateProjectScopeById, UpdateProjectTypesById, UpdateSectorsById, UpdateSourceOfFundsById, UpdateSustainableDevelopmentGoalsById, UpdateUnitOfMeasurementsById}}, 
    };

// Sustainable Development Goals
// POST / GET
pub async fn add_sustainable_development_goals (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<SustainableDevelopmentGoals>,
) -> impl IntoResponse {
    let find_same_sdg: Result<ReturnSustainableDevelopmentGoals, sqlx::Error> = 
        sqlx::query_as("SELECT * FROM pi_sdg where sdg_title = ?")
            .bind(&request.sdg_title)
            .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_sdg {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    match sqlx::query("INSERT INTO pi_sdg (sdg_title) VALUES (?)")
        .bind(request.sdg_title)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }
}

pub async fn update_sustainable_development_goals_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateSustainableDevelopmentGoalsById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnSustainableDevelopmentGoals, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_sdg where sdg_title = ?")
        .bind(&request.sdg_title)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("UPDATE pi_sdg SET sdg_title = ? WHERE id = ?")
        .bind(request.sdg_title)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
        }
}

pub async fn delete_sustainable_development_goals_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_sdg WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
    }

}

pub async fn all_sustainable_development_goals (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_sdg ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
        Ok(result) => {
            let result: Vec<ReturnSustainableDevelopmentGoals> = result;
            (StatusCode::OK, Json(result))
        },
        Err(_) => (StatusCode::OK, Json(Vec::default()))
    }
}

// Source of Funds
// POST / GET
pub async fn add_source_of_funds (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<SourceOfFunds>
) -> impl IntoResponse {

    let find_same_sof: Result<ReturnSourceOfFunds, sqlx::Error> = 
        sqlx::query_as("SELECT * FROM pi_sof where sof_title = ?")
            .bind(&request.sof_title)
            .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_sof {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    match sqlx::query("INSERT INTO pi_sof (sof_title) VALUES (?)")
        .bind(request.sof_title)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }
}

pub async fn update_source_of_funds_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateSourceOfFundsById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnSourceOfFunds, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_sof WHERE sof_title = ?")
        .bind(&request.sof_title)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("UPDATE pi_sof SET sof_title = ? WHERE id = ?")
        .bind(request.sof_title)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
    }

}

pub async fn delete_source_of_funds_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_sof WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
    }

}

pub async fn all_source_of_funds (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {
    
    match sqlx::query_as("SELECT * FROM pi_sof ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnSourceOfFunds> = result;

                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
        }
}

// Project Types
// POST / GET
pub async fn add_project_types (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<ProjectTypes>
) -> impl IntoResponse {

    let find_same_pt: Result<ReturnProjectTypes, sqlx::Error> =
        sqlx::query_as("SELECT * FROM pi_pt WHERE pt_title = ?")
            .bind(&request.pt_title)
            .fetch_one(&sql_pool.connection).await;
    
    if let Ok(_) = find_same_pt {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO pi_pt (pt_title) VALUES (?)")
        .bind(request.pt_title)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }

}

pub async fn update_project_types_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateProjectTypesById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnProjectTypes, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_pt WHERE pt_title = ?")
        .bind(&request.pt_title)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("UPDATE pi_pt SET pt_title = ? WHERE id = ?")
        .bind(request.pt_title)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
    }

}

pub async fn delete_project_types_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_pt WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::OK, format!("{:?}", ResponseErrorMessage::DataNotDelete))
    }

}

pub async fn all_project_types (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_pt ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnProjectTypes> = result;
                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
    }

}

// Incharge
// POST / GET
pub async fn add_incharge (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<Incharge>
) -> impl IntoResponse {

    let find_same_incharge: Result<ReturnIncharge, sqlx::Error> =
        sqlx::query_as("SELECT * FROM pi_incharge WHERE incharge = ?")
            .bind(&request.incharge)
            .fetch_one(&sql_pool.connection).await;
    
    if let Ok(_) = find_same_incharge {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO pi_incharge (incharge) VALUES (?)")
        .bind(request.incharge)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }
}

pub async fn update_incharge_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateInchargeById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnIncharge, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_incharge WHERE incharge = ?")
        .bind(&request.incharge)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("UPDATE pi_incharge SET incharge = ? WHERE id = ?")
        .bind(request.incharge)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
    }

}

pub async fn delete_incharge_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_incharge WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
        }

}

pub async fn all_incharge (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_incharge ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnIncharge> = result;

                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
        }

}

// Categories
// POST / GET
pub async fn add_categories (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<Categories>
) -> impl IntoResponse {
    
    let find_same_categories: Result<ReturnCategories, sqlx::Error> =
        sqlx::query_as("SELECT * FROM pi_categories WHERE categories = ?")
            .bind(&request.categories)
            .fetch_one(&sql_pool.connection).await;
    
    if let Ok(_) = find_same_categories {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match  sqlx::query("INSERT INTO pi_categories (categories) VALUE (?)")
        .bind(request.categories)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }
}

pub async fn update_categories_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateCategoriesById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnCategories, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_categories WHERE categories = ?")
        .bind(&request.categories)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    match sqlx::query("UPDATE pi_categories SET categories = ? WHERE id = ?")
        .bind(request.categories)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
        }

}

pub async fn delete_categories_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_categories WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
        }

}

pub async fn all_categories (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_categories ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnCategories> = result;
                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
        }

}

// Sectors
// POST / GET
pub async fn add_sectors (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<Sectors>
) -> impl IntoResponse {

    let find_same_sectors: Result<ReturnSectors, sqlx::Error> = 
        sqlx::query_as("SELECT * FROM pi_sectors WHERE sectors = ?")
            .bind(&request.sectors)
            .fetch_one(&sql_pool.connection).await;
    
    if let Ok(_) = find_same_sectors {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO pi_sectors (sectors) VALUE (?)")
        .bind(request.sectors)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }
}

pub async fn update_sectors_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateSectorsById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnSectors, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_sectors WHERE sectors = ?")
        .bind(&request.sectors)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    match sqlx::query("UPDATE pi_sectors SET sectors = ? WHERE id = ?")
        .bind(request.sectors)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
    }

}

pub async fn delete_sectors_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_sectors WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
        }

}

pub async fn all_sectors (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_sectors ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnSectors> = result;
                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
        }

}

// Barangays
// POST / GET
pub async fn add_barangays (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<Barangays>
) -> impl IntoResponse {

    let find_same_barangays: Result<ReturnBarangays, sqlx::Error> = 
        sqlx::query_as("SELECT * FROM pi_barangays WHERE barangays = ?")
            .bind(&request.barangays)
            .fetch_one(&sql_pool.connection).await;
    
    if let Ok(_) = find_same_barangays {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO pi_barangays (barangays) VALUE (?)")
        .bind(request.barangays)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
        }
}

pub async fn update_barangays_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateBarangaysById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnBarangays, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_barangays WHERE barangays = ?")
        .bind(&request.barangays)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    match sqlx::query("UPDATE pi_barangays SET barangays = ? WHERE id = ?")
        .bind(request.barangays)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
    }

}

pub async fn delete_barangays_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_barangays WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
        }

}

pub async fn all_barangays (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_barangays ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnBarangays> = result;
                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
        }
}

// Unit of Measurements
// POST / GET
pub async fn add_unit_of_measurements (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UnitOfMeasurements>
) -> impl IntoResponse {

    let find_same_um: Result<ReturnUnitOfMeasurements, sqlx::Error> = 
        sqlx::query_as("SELECT * FROM pi_um WHERE um_title = ?")
            .bind(&request.um_title)
            .fetch_one(&sql_pool.connection).await;
    
    if let Ok(_) = find_same_um {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO pi_um (um_title) VALUE (?)")
        .bind(request.um_title)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ErrorGeneratingToken))
    }
}

pub async fn update_unit_of_measurements_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateUnitOfMeasurementsById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnUnitOfMeasurements, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_um WHERE um_title = ?")
        .bind(&request.um_title)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    match sqlx::query("UPDATE pi_um SET um_title = ? WHERE id = ?")
        .bind(request.um_title)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
        }

}

pub async fn delete_unit_of_measurements_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_um WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
    }

}

pub async fn all_unit_of_measurements (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_um ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnUnitOfMeasurements> = result;
                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
        }

}

// Project Scope
// POST / GET
pub async fn add_project_scope (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<ProjectScope>
) -> impl IntoResponse {

    let find_same_ps: Result<ReturnProjectScope, sqlx::Error> = 
        sqlx::query_as("SELECT * FROM pi_ps WHERE ps_title = ?")
            .bind(&request.ps_title)
            .fetch_one(&sql_pool.connection).await;
    
    if let Ok(_) = find_same_ps {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    }

    match sqlx::query("INSERT INTO pi_ps (ps_title) VALUE (?)")
        .bind(request.ps_title)
        .execute(&sql_pool.connection).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataCreated)),
            Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", ResponseErrorMessage::ExecutingQueryError))
    }
}

pub async fn update_project_scope_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<UpdateProjectScopeById>
) -> impl IntoResponse {

    let find_same_name: Result<ReturnProjectScope, sqlx::Error> = sqlx::query_as("SELECT * FROM pi_ps WHERE ps_title = ?")
        .bind(&request.ps_title)
        .fetch_one(&sql_pool.connection).await;

    if let Ok(_) = find_same_name {
        return (StatusCode::NOT_ACCEPTABLE, format!("{:?}", ResponseErrorMessage::AlreadyExists));
    };

    match sqlx::query("UPDATE pi_ps SET ps_title = ? WHERE id = ?")
        .bind(request.ps_title)
        .bind(request.id)
        .execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::NewDataUpdate)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotModified))
    }

}

pub async fn delete_project_scope_by_id (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>,
    Json(request): Json<RequestById>
) -> impl IntoResponse {

    match sqlx::query("DELETE FROM pi_ps WHERE id = ?")
        .bind(request.id).execute(&sql_pool.connection.to_owned()).await {
            Ok(_) => (StatusCode::OK, format!("{:?}", ResponseOkMessage::DataDeleteSuccess)),
            Err(_) => (StatusCode::NOT_MODIFIED, format!("{:?}", ResponseErrorMessage::DataNotDelete))
        }

}

pub async fn all_project_scope (
    Extension(sql_pool): Extension<Arc<PoolConnectionState>>
) -> impl IntoResponse {

    match sqlx::query_as("SELECT * FROM pi_ps ORDER BY id DESC")
        .fetch_all(&sql_pool.connection.to_owned()).await {
            Ok(result) => {
                let result: Vec<ReturnProjectScope> = result;
                (StatusCode::OK, Json(result))
            },
            Err(_) => (StatusCode::OK, Json(Vec::default()))
        }

}