use warp::{Filter, Rejection, Reply, http::Response, http::StatusCode};
use serde::{Deserialize};
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::{Mutex};

type Result<T> = std::result::Result<T, Rejection>;
type UsersDb = Arc<Mutex<HashMap<String, UserData>>>;

#[derive(Debug, Clone, Deserialize)]
struct UserData {
    username: String,
    password: String
}

#[tokio::main]
async fn main() {
    let users_db: UsersDb = Arc::new(Mutex::new(HashMap::new()));

    let default_route = warp::any().map(|| { "Welcome"});
    let register_path = warp::path("register");
    let register_routes = register_path
    .and(warp::get())
    .map(|| "Please use a HTTP POST request to register")
    .or(register_path
        .and(warp::post())
        .and(warp::body::json())
        .and(with_users_db(users_db.clone()))
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

async fn register_handler(user: UserData, users_db: UsersDb) -> Result<impl Reply> {
    println!("Received UserData: {:?}", user);
    if users_db.lock().await.contains_key(&user.username) {
        println!("User already exists");
        return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body("User already exists"));
    }
    users_db.lock().await.insert(user.username.clone(), user);
    println!("Users database: {:?}", users_db);
    Ok(Response::builder().status(StatusCode::CREATED).body("registered"))
}

async fn login_handler() -> Result<impl Reply> {
    Ok(Response::builder().body("logged in"))
}

fn with_users_db(users_db: UsersDb) -> impl Filter<Extract = (UsersDb,), Error = Infallible> + Clone {
    warp::any().map(move || users_db.clone())
}
