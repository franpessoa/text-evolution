pub mod calc;

use axum::{
    Router,
    Json,
    routing::get,
    response::IntoResponse,
    handler::HandlerWithoutStateExt, 
};
use hyper::StatusCode;
use tower_http::services::ServeDir;
use serde_derive::{Serialize, Deserialize};
use nanoid::nanoid;

use crate::calc::{
    Individual,
    Simulation
};

#[derive(Deserialize)]
struct ApiRequest {
    target: String,
    clones_per_ind: u32,
    survivors: u32,
    mut_count: u16,
    ind_count: u32
}

#[derive(Serialize)]
struct ResponseGeneration {
    best: String,
    proximity: u16
}

#[derive(Serialize)]
struct ApiResponse {
    results: Vec<ResponseGeneration>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let addr = "0.0.0.0:3000";
    let server = axum::Server::bind(&addr.parse().unwrap());  
    
    tracing::debug!("Listening on http://localhost:3000");
    
    let router = Router::new()
        .route("/api/render", get(handler))
        .nest_service(
            "/static", 
            ServeDir::new("static")
                .not_found_service(file_not_found.into_service()
            )
        );
    
    server
        .serve(router.into_make_service())
        .await
        .unwrap();
    
}

async fn file_not_found() -> (StatusCode, &'static str){
    return (StatusCode::NOT_FOUND, "File not found")
}

fn sanitize(arg: &ApiRequest) -> Result<(), &'static str> {    
    if arg.survivors > arg.clones_per_ind * arg.ind_count {
        return Err("Numero de sobreviventes deve ser menor do que o numero de individuos criador a cada geraçao")
    } else if arg.target.chars().count() <= 2 {
        return Err("Forneça pelo menos dois caracteres como entrada")
    } else if arg.mut_count <= 0 {
        return Err("Deve haver mais de uma mutaçao por geraçao")
    } else if arg.ind_count <= 0 || arg.clones_per_ind <= 0 {
        return Err("O numero de individuos deve ser no minimo 1")
    } else {
        return Ok(())
    }
}

async fn handler(Json(body): Json<ApiRequest>) -> impl IntoResponse {
    let sanitized = sanitize(&body);
    if sanitized.is_err() {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            sanitized.err().unwrap()
        ).into_response()
    }
    
    let mut individuals = Vec::new();
    let start_val = "x".repeat(body.target.chars().count());
    
    for _ in 0..body.ind_count {
        individuals.push(
            Individual {
                id: nanoid!(5),
                content: start_val.clone(),
                parent: None
            }
        )
    }
    

    let mut simulation = Simulation {
        target: body.target,
        individuals,
        num_clones: body.clones_per_ind,
        mut_count: body.mut_count,
        survivors: body.survivors,
        gen_count: 0
    };
    let mut results: Vec<ResponseGeneration> = Vec::new();
    
    'simulation_loop: loop {
        simulation = simulation.advance_gen();
        
        let best = simulation.individuals.get(0).unwrap();
        tracing::info!("{:?} -> {:?}", best.content, simulation.target);
        
        results.push(
            ResponseGeneration {
                best: best.content.clone(),
                proximity: best.compare(&simulation.target).unwrap()
            }
        );
        
        for ind in &simulation.individuals {
            if ind.is_target(&simulation.target) {
                break 'simulation_loop;
            }
        }
    }
        
    return Json(ApiResponse {
        results
    }).into_response()
}