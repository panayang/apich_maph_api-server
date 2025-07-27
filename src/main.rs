use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use core_engine::{CoreEngine, ProblemDefinition, Solution};

/// The main handler for the /simulation endpoint.
///
/// This function receives a JSON ProblemDefinition, runs the simulation using the
/// core engine, and returns the Solution as JSON.
async fn run_simulation_handler(problem_def: web::Json<ProblemDefinition>) -> impl Responder {
    println!("Received simulation request: {}", problem_def.id);

    // Create a new instance of our core engine.
    let engine = CoreEngine::new();

    // Run the simulation.
    match engine.run_simulation(problem_def.into_inner()).await {
        Ok(solution) => HttpResponse::Ok().json(solution),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting API server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .route("/simulation", web::post().to(run_simulation_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

