use serde::Deserialize;
use std::{collections::HashMap, convert::Infallible, sync::Arc};
use tokio::sync::Mutex;
use warp::{http::Response, http::StatusCode, Filter, Rejection, Reply};

type Result<T> = std::result::Result<T, Rejection>;
type UsersDb = Arc<Mutex<HashMap<String, UserData>>>;

#[derive(Debug, Clone, Deserialize)]
struct UserData {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Deserialize)]
struct LoginData {
    username: String,
    password: String,
}

#[tokio::main]
async fn main() {
    let users_db: UsersDb = Arc::new(Mutex::new(HashMap::new()));

    let default_route = warp::any().map(|| "Welcome");
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
            .and(warp::body::json())
            .and(with_users_db(users_db.clone()))
            .and_then(login_handler));

    let routes = register_routes
        .or(login_routes)
        .or(default_route)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 5000)).await;
}

async fn register_handler(user: UserData, users_db: UsersDb) -> Result<impl Reply> {
    println!("Received UserData: {:?}", user);
    if users_db.lock().await.contains_key(&user.username) {
        println!("User already exists");
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("User already exists"));
    }
    users_db.lock().await.insert(user.username.clone(), user);
    println!("Users database: {:?}", users_db);
    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .body("registered"))
}

async fn login_handler(login_data: LoginData, users_db: UsersDb) -> Result<impl Reply> {
    println!("Received login data: {:?}", login_data);

    let cur_user_db = users_db.lock().await;
    let user = match cur_user_db.get(&login_data.username) {
        Some(k) => k,
        None => {
            println!("User '{}' not found in database", &login_data.username);
            return Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("login failed"));
        }
    };

    if user.password != login_data.password {
        println!("Password incorrect for user: {}", &login_data.username);
        return Ok(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body("login failed"));
    }

    println!("Login ok");
    Ok(Response::builder().status(StatusCode::OK).body("login ok"))
}

fn with_users_db(
    users_db: UsersDb,
) -> impl Filter<Extract = (UsersDb,), Error = Infallible> + Clone {
    warp::any().map(move || users_db.clone())
}
