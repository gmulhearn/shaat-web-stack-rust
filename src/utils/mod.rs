pub mod askama_to_actix_responder;

pub fn random_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
