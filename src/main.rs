use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use warp::Filter;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u64,
    name: String,
}

type Users = Arc<Mutex<Vec<User>>>;

#[tokio::main]
async fn main() {
    let users = Users::default();

    let routes = warp::path("users")
        .and(
            warp::post()
                .and(warp::body::json())
                .and(with_users(users.clone()))
                .and_then(create_user),
        )
        .or(warp::get()
            .and(with_users(users.clone()))
            .and_then(list_users));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn create_user(new_user: User, users: Users) -> Result<impl warp::Reply, warp::Rejection> {
    let mut users_locked = users.lock().unwrap();
    users_locked.push(new_user);
    let users_cloned = users_locked.clone();
    Ok(warp::reply::json(&users_cloned))
}

async fn list_users(users: Users) -> Result<impl warp::Reply, warp::Rejection> {
    let users = users.lock().unwrap().clone();
    Ok(warp::reply::json(&users))
}

fn with_users(users: Users) -> impl Filter<Extract = (Users,), Error = Infallible> + Clone {
    warp::any().map(move || users.clone())
}
