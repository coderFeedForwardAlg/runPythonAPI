#[macro_use] extern crate rocket;

#[cfg(test)]
mod tests;
mod paste_id;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};
// use rocket::http::hyper::Method;
use rocket::http::Method;

use std::io;

use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};
use std::process::Command;

use std::fs::OpenOptions;
use std::io::{BufReader, BufRead, Write};


use paste_id::PasteId;

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:8000");
const ID_LENGTH: usize = 6;

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;


    // fix webkit stuff  (bad code put in fn)
    let mut file = OpenOptions::new()
    .read(true)
    .write(true)
    .open(id.file_path().to_str().unwrap())
    .expect("file.txt doesn't exist or so");

    let mut lines = BufReader::new(file).lines().skip(3)
    .map(|x| x.unwrap())
    .collect::<Vec<String>>().split_last().unwrap().1.join("\n");

    fs::write(id.file_path().to_str().unwrap(), lines).await.expect("Can't write");


    let p1 = Command::new("python3")
        // .current_dir("../upload")
        .args([id.file_path()])
        .output();


    
        
    
    // Ok(uri!(HOST, retrieve(id)).to_string())
    let hello = String::from_utf8(p1.unwrap().stdout).unwrap(); //res.stdout;
    return Ok(hello);
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}


#[delete("/<id>")]
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

          EXAMPLE: curl --data-binary @file.txt http://localhost:8000

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}


fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::all();

    CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, upload, delete, retrieve]).attach(make_cors())
}