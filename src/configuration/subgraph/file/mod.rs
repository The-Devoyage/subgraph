use serde::{Deserialize, Serialize};

/// Configuration for the file upload and serving capabilities of the server.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FileDirectory {
    /// The path to the directory where files are stored.
    pub path: String,
    /// The route to serve files from.
    pub route: String,
}

impl FileDirectory {
    /// Warp Middleware to determine if the file should be served.
    pub async fn check_file_perm(
        file: warp::fs::File,
        access: bool,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        if access {
            Ok(file)
        } else {
            Err(warp::reject::not_found())
        }
    }
}
