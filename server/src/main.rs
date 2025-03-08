use actix_web::{post, web, App, HttpServer, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct PsiRequest {
    public_key: String,
    encrypted_interests: Vec<String>,
    merkle_root: String,
}

#[post("/process-psi")]
async fn process_psi(info: web::Json<PsiRequest>) -> impl Responder {
    // Here you would perform your PSI computation, verify the Merkle tree, and generate a zk proof.
    // For demonstration, we simply echo a dummy response.
    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": "PSI processed",
        "fhe_result": ["Result1", "Result2"]
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(process_psi)
            // Optionally add CORS if your front end and server are on different origins.
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
