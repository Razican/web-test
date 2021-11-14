use rocket::{fs::NamedFile, get, routes, tokio::fs, Route};
use std::{
    env, io,
    path::{Path, PathBuf},
};

/// Gets the routes for the frontend
pub fn routes() -> Vec<Route> {
    routes![frontend]
}

/// Serving the frontend files for the application.
#[get("/<route..>", rank = 100)]
pub async fn frontend(route: PathBuf) -> io::Result<NamedFile> {
    let dist_folder = env::var("DIST_FOLDER").unwrap_or_else(|_| "./dist".to_owned());
    let file_path = Path::new(&dist_folder).join(&route);

    if route == Path::new("") || {
        let metadata = fs::metadata(&file_path).await;
        match metadata {
            Ok(m) if m.is_dir() => true,
            Err(e) if e.kind() == io::ErrorKind::NotFound => true,
            Err(e) => return Err(e),
            _ => false,
        }
    } {
        let index_file = Path::new(&dist_folder).join("index.html");
        NamedFile::open(index_file).await
    } else {
        NamedFile::open(file_path).await
    }
}
