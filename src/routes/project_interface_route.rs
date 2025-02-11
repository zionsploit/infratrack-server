use axum::{routing::{get, post}, Router};

use crate::services::project_interface_service::{add_barangays, add_categories, add_incharge, add_project_scope, add_project_types, add_sectors, add_source_of_funds, add_sustainable_development_goals, add_unit_of_measurements, all_barangays, all_categories, all_incharge, all_project_scope, all_project_types, all_sectors, all_source_of_funds, all_sustainable_development_goals, all_unit_of_measurements, delete_barangays_by_id, delete_categories_by_id, delete_incharge_by_id, delete_project_scope_by_id, delete_project_types_by_id, delete_sectors_by_id, delete_source_of_funds_by_id, delete_sustainable_development_goals_by_id, delete_unit_of_measurements_by_id, update_barangays_by_id, update_categories_by_id, update_incharge_by_id, update_project_scope_by_id, update_project_types_by_id, update_sectors_by_id, update_source_of_funds_by_id, update_sustainable_development_goals_by_id, update_unit_of_measurements_by_id};

pub fn project_interface_route () -> Router {
    Router::new()
        .nest("/sustainable-development-goals", 
            Router::new()
                .route("/add", post(add_sustainable_development_goals))
                .route("/update-by-id", post(update_sustainable_development_goals_by_id))
                .route("/delete-by-id", post(delete_sustainable_development_goals_by_id))
                .route("/get-all", get(all_sustainable_development_goals))
        )
        .nest("/source-of-funds", 
        Router::new()
                .route("/add", post(add_source_of_funds))
                .route("/update-by-id", post(update_source_of_funds_by_id))
                .route("/delete-by-id", post(delete_source_of_funds_by_id))
                .route("/get-all", get(all_source_of_funds))
        )
        .nest("/project-types", 
        Router::new()
                .route("/add", post(add_project_types))
                .route("/update-by-id", post(update_project_types_by_id))
                .route("/delete-by-id", post(delete_project_types_by_id))
                .route("/get-all", get(all_project_types))
        )
        .nest("/incharge", 
         Router::new()
                .route("/add", post(add_incharge))
                .route("/update-by-id", post(update_incharge_by_id))
                .route("/delete-by-id", post(delete_incharge_by_id))
                .route("/get-all", get(all_incharge))
        )
        .nest("/categories", 
        Router::new()
                .route("/add", post(add_categories))
                .route("/update-by-id", post(update_categories_by_id))
                .route("/delete-by-id", post(delete_categories_by_id))
                .route("/get-all", get(all_categories))
        )
        .nest("/sectors",
        Router::new()
                .route("/add", post( add_sectors))
                .route("/update-by-id", post(update_sectors_by_id))
                .route("/delete-by-id", post(delete_sectors_by_id))
                .route("/get-all", get(all_sectors))   
        )
        .nest("/barangays", 
        Router::new()
                .route("/add", post(add_barangays))
                .route("/update-by-id", post(update_barangays_by_id))
                .route("/delete-by-id", post(delete_barangays_by_id))
                .route("/get-all", get (all_barangays))
        )
        .nest("/unit-of-measurements", 
        Router::new()
                .route("/add", post(add_unit_of_measurements))
                .route("/update-by-id", post(update_unit_of_measurements_by_id))
                .route("/delete-by-id", post(delete_unit_of_measurements_by_id))
                .route("/get-all", get(all_unit_of_measurements))
        )
        .nest("/project-scope", 
        Router::new()
                .route("/add", post(add_project_scope))
                .route("/update-by-id", post(update_project_scope_by_id))
                .route("/delete-by-id", post(delete_project_scope_by_id))
                .route("/get-all", get(all_project_scope))
        )
}