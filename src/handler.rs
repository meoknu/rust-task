use crate::{
    model::{NodeModel, NodeModelResponse},
    schema::{CreateNodeSchema},
    AppState,
};
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/nodes")]
pub async fn note_list_handler(
    data: web::Data<AppState>,
) -> impl Responder {

    let nodes: Vec<NodeModel> = sqlx::query_as!(
        NodeModel,
        r#"SELECT * FROM nodes"#
    )
    .fetch_all(&data.db)
    .await
    .unwrap();

    let note_responses = nodes
        .into_iter()
        .map(|note| filter_db_record(&note))
        .collect::<Vec<NodeModelResponse>>();

    let json_response = serde_json::json!({
        "status": "success",
        "results": note_responses.len(),
        "nodes": note_responses
    });
    HttpResponse::Ok().json(json_response)
}

#[post("/nodes/")]
async fn create_note_handler(
    body: web::Json<CreateNodeSchema>,
    data: web::Data<AppState>,
) -> impl Responder {

    let query_result = sqlx::query_as!(NodeModel, r#"SELECT * FROM nodes WHERE id = ?"#, body.parent_id)
        .fetch_one(&data.db)
        .await;

    match query_result {
        Ok(_note) => {
            let insert_result =
                sqlx::query(r#"INSERT INTO nodes (parent_id) VALUES (?)"#)
                    .bind(body.parent_id)
                    .execute(&data.db)
                    .await
                    .map_err(|err: sqlx::Error| err.to_string());

            match insert_result {
                Ok(result) => {
                    let last_insert_id = result.last_insert_id();
                    let note_response = serde_json::json!({"status": "success","id": last_insert_id});
                    return HttpResponse::Ok().json(note_response);
                },
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
                }
            }
        }
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", e)}));
        }
    }
}

fn filter_db_record(note: &NodeModel) -> NodeModelResponse {
    NodeModelResponse {
        id: note.id.to_owned(),
        parent_id: note.parent_id.to_owned(),
    }
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(note_list_handler)
        .service(create_note_handler);

    conf.service(scope);
}
