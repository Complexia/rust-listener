use std::fs;
use actix_web::{post, web, Error, App, HttpResponse, HttpServer};
use futures_util::stream::StreamExt;

#[post("/upload")]
async fn upload(req: web::HttpRequest, mut payload: web::Payload) -> Result<HttpResponse, Error> {
    println!("Upload execution start");

    for (name, value) in req.headers().iter() {
        println!("Header {}: {:?}", name, value);
    }

    // Read the entire payload into a buffer
    let mut payload_data = web::BytesMut::new();
    let mut stream = payload.into_inner();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        payload_data.extend_from_slice(&chunk);
    }

    // Convert payload to string and print
    if let Ok(payload_str) = std::str::from_utf8(&payload_data) {
        println!("Payload: {}", payload_str);
    } else {
        println!("Failed to convert payload to string");
    }

    println!("File upload success");
    Ok(HttpResponse::Ok().body("200"))
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server");
    fs::create_dir_all("./uploads")?;

    HttpServer::new(|| {
        App::new()
            .service(upload)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}