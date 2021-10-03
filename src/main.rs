use warp::{Filter, Rejection, Reply, http::Response};

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    let default_route = warp::any().map(|| { "Welcome"});
    let register_path = warp::path("register");
    let register_routes = register_path
    .and(warp::get())
    .map(|| "Please use a HTTP POST request to register")
    .or(register_path
        .and(warp::post())
        .and_then(register_handler));
    
    let login_path = warp::path("login");
    let login_routes = login_path
    .and(warp::get())
    .map(|| "Please use a HTTP POST request to login")
    .or(login_path
        .and(warp::post())
        .and_then(login_handler));
    
    let routes = register_routes.or(login_routes).or(default_route).with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127,0,0,1], 5000)).await;

}

async fn register_handler() -> Result<impl Reply> {
    Ok(Response::builder().body("registered"))
}

async fn login_handler() -> Result<impl Reply> {
    Ok(Response::builder().body("logged in"))
}

