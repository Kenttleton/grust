use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};

// Group -> User -> Repository
#[post("/{group}/{user}/{repository}")]
async fn create_group_user_repository(web::Path((group, user, repository)): web::Path<(String, String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("CREATE\nGroup: {}\nUser: {}\nRepository: {}", group, user, repository))
}

#[get("/{group}/{user}/{repository}")]
async fn read_group_user_repository(web::Path((group, user, repository)): web::Path<(String, String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("READ\nGroup: {}\nUser: {}\nRepository: {}", group, user, repository))
}

#[put("/{group}/{user}/{repository}")]
async fn update_group_user_repository(web::Path((group, user, repository)): web::Path<(String, String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("UPDATE\nGroup: {}\nUser: {}\nRepository: {}", group, user, repository))
}

#[delete("/{group}/{user}/{repository}")]
async fn delete_group_user_repository(web::Path((group, user, repository)): web::Path<(String, String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("DELETE\nGroup: {}\nUser: {}\nRepository: {}", group, user, repository))
}

// User -> Repository
#[post("/{user}/{repository}")]
async fn create_user_repository(web::Path((user, repository)): web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("CREATE\nUser: {}\nRepository: {}", user, repository))
}

#[get("/{user}/{repository}")]
async fn read_user_repository(web::Path((user, repository)): web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("READ\nUser: {}\nRepository: {}", user, repository))
}

#[put("/{user}/{repository}")]
async fn update_user_repository(web::Path((user, repository)): web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("UPDATE\nUser: {}\nRepository: {}", user, repository))
}

#[delete("/{user}/{repository}")]
async fn delete_user_repository(web::Path((user, repository)): web::Path<(String, String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("DELETE\nUser: {}\nRepository: {}", user, repository))
}

// User interactions
#[post("/{user}")]
async fn create_user_information(web::Path(user): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("CREATE\nUser: {}", user))
}

#[get("/{user}")]
async fn read_user_information(web::Path(user): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("READ\nUser: {}", user))
}

#[put("/{user}")]
async fn update_user_information(web::Path(user): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("UPDATE\nUser: {}", user))
}

#[delete("/{user}")]
async fn delete_user_information(web::Path(user): web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("DELETE\nUser: {}", user))
}

// Root
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("git server")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new()
        .service(create_group_user_repository)
        .service(read_group_user_repository)
        .service(update_group_user_repository)
        .service(delete_group_user_repository)
        .service(create_user_repository)
        .service(read_user_repository)
        .service(update_user_repository)
        .service(delete_user_repository)
        .service(create_user_information)
        .service(read_user_information)
        .service(update_user_information)
        .service(delete_user_information)
        .service(index))
            .bind("0.0.0.0:8080")?
            .run()
            .await
}
