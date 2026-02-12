use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;
use super::helpers::{rpc_ok, rpc_created};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/v1/groups/{number}", get(list_groups).post(create_group))
        .route(
            "/v1/groups/{number}/{groupid}",
            get(get_group).put(update_group).delete(delete_group),
        )
        .route(
            "/v1/groups/{number}/{groupid}/members",
            post(add_members).delete(remove_members),
        )
        .route(
            "/v1/groups/{number}/{groupid}/admins",
            post(add_admins).delete(remove_admins),
        )
        .route("/v1/groups/{number}/{groupid}/avatar", get(get_avatar))
        .route("/v1/groups/{number}/{groupid}/join", post(join_group))
        .route("/v1/groups/{number}/{groupid}/quit", post(quit_group))
        .route("/v1/groups/{number}/{groupid}/block", post(block_group))
}

// ---- List / Get -----------------------------------------------------------

async fn list_groups(
    State(st): State<AppState>,
    Path(number): Path<String>,
) -> Response {
    rpc_ok(&st, "listGroups", json!({ "account": number })).await
}

async fn get_group(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
) -> Response {
    rpc_ok(&st, "listGroups", json!({ "account": number, "group-id": groupid })).await
}

// ---- Create / Update / Delete ---------------------------------------------

#[derive(Deserialize)]
struct CreateGroupBody {
    name: String,
    members: Vec<String>,
    description: Option<String>,
    #[serde(default)]
    permissions: Option<GroupPermissions>,
}

#[derive(Deserialize)]
struct GroupPermissions {
    add_members: Option<String>,
    edit_details: Option<String>,
}

async fn create_group(
    State(st): State<AppState>,
    Path(number): Path<String>,
    Json(body): Json<CreateGroupBody>,
) -> Response {
    let mut params = json!({
        "account": number,
        "name": body.name,
        "member": body.members,
    });
    if let Some(desc) = &body.description {
        params["description"] = json!(desc);
    }
    if let Some(perms) = &body.permissions {
        if let Some(add) = &perms.add_members {
            params["set-permission-add-member"] = json!(add);
        }
        if let Some(edit) = &perms.edit_details {
            params["set-permission-edit-details"] = json!(edit);
        }
    }
    rpc_created(&st, "updateGroup", params).await
}

#[derive(Deserialize)]
struct UpdateGroupBody {
    name: Option<String>,
    description: Option<String>,
    base64_avatar: Option<String>,
    expiration: Option<u64>,
    #[serde(default)]
    permissions: Option<GroupPermissions>,
}

async fn update_group(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
    Json(body): Json<UpdateGroupBody>,
) -> Response {
    let mut params = json!({
        "account": number,
        "group-id": groupid,
    });
    if let Some(name) = &body.name {
        params["name"] = json!(name);
    }
    if let Some(desc) = &body.description {
        params["description"] = json!(desc);
    }
    if let Some(avatar) = &body.base64_avatar {
        params["avatar"] = json!(avatar);
    }
    if let Some(exp) = body.expiration {
        params["expiration"] = json!(exp);
    }
    if let Some(perms) = &body.permissions {
        if let Some(add) = &perms.add_members {
            params["set-permission-add-member"] = json!(add);
        }
        if let Some(edit) = &perms.edit_details {
            params["set-permission-edit-details"] = json!(edit);
        }
    }
    rpc_ok(&st, "updateGroup", params).await
}

async fn delete_group(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
) -> Response {
    rpc_ok(&st, "quitGroup", json!({ "account": number, "group-id": groupid, "delete": true })).await
}

// ---- Members / Admins -----------------------------------------------------

#[derive(Deserialize)]
struct MembersBody {
    members: Vec<String>,
}

async fn add_members(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
    Json(body): Json<MembersBody>,
) -> Response {
    rpc_ok(&st, "updateGroup", json!({
        "account": number,
        "group-id": groupid,
        "addMember": body.members,
    })).await
}

async fn remove_members(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
    Json(body): Json<MembersBody>,
) -> Response {
    rpc_ok(&st, "updateGroup", json!({
        "account": number,
        "group-id": groupid,
        "removeMember": body.members,
    })).await
}

#[derive(Deserialize)]
struct AdminsBody {
    admins: Vec<String>,
}

async fn add_admins(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
    Json(body): Json<AdminsBody>,
) -> Response {
    rpc_ok(&st, "updateGroup", json!({
        "account": number,
        "group-id": groupid,
        "addAdmin": body.admins,
    })).await
}

async fn remove_admins(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
    Json(body): Json<AdminsBody>,
) -> Response {
    rpc_ok(&st, "updateGroup", json!({
        "account": number,
        "group-id": groupid,
        "removeAdmin": body.admins,
    })).await
}

// ---- Avatar / Join / Quit / Block -----------------------------------------

async fn get_avatar(
    Path((_number, _groupid)): Path<(String, String)>,
) -> Response {
    (StatusCode::NOT_IMPLEMENTED, Json(json!({ "error": "Group avatar retrieval not yet implemented" }))).into_response()
}

async fn join_group(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
) -> Response {
    rpc_ok(&st, "joinGroup", json!({ "account": number, "group-id": groupid })).await
}

async fn quit_group(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
) -> Response {
    rpc_ok(&st, "quitGroup", json!({ "account": number, "group-id": groupid })).await
}

async fn block_group(
    State(st): State<AppState>,
    Path((number, groupid)): Path<(String, String)>,
) -> Response {
    rpc_ok(&st, "block", json!({ "account": number, "group-id": groupid })).await
}
