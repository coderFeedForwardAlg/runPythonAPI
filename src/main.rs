use std::path::Path;


#[macro_use] extern crate rocket;
mod paste_id;

use std::io;

use rocket::data::{Data, ToByteUnit};
use rocket::http::uri::Absolute;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};

use paste_id::PasteId;

// In a real application, these would be retrieved dynamically from a config.
const HOST: Absolute<'static> = uri!("http://localhost:8000");
const ID_LENGTH: usize = 3;

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[get("/<id>")]
async fn retrieve(id: &str) -> Option<File> {
    let upload_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
    let filename = Path::new(upload_dir).join(id);
    File::open(&filename).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, retrieve])
}