mod unreal_struct;

pub mod route {
    use std::path::Path;

    use axum::{routing::post, Router};
    use tower_http::services::ServeFile;

    use crate::AppResult;

    pub fn new_router(base_path: impl AsRef<Path>) -> Router<()> {
        let base_path = base_path.as_ref();
        let default_path = base_path.join("DefaultPalWorldSettings.ini");
        let current_path = base_path.join("Pal/Saved/Config/LinuxServer/PalWorldSettings.ini");
        Router::new()
            .route_service("/default", ServeFile::new(&default_path))
            .route_service("/current", ServeFile::new(&current_path))
            .route("/save", post(|body| save(body, current_path)))
    }

    async fn save(body: String, path: impl AsRef<Path>) -> AppResult<()> {
        tokio::fs::create_dir_all(path.as_ref().parent().unwrap()).await?;
        tokio::fs::write(path, body).await?;
        Ok(())
    }
}

// fn parse_ini() {
//     let i = Ini::load_from_file("/Users/shiroki/Downloads/DefaultPalWorldSettings.ini").unwrap();
//     for (sec, prop) in i.iter() {
//         println!("Section: {:?}", sec);
//         for (k, v) in prop.iter() {
//             // println!("{}:{}", k, v);
//             if v.starts_with('(') && v.ends_with(')') {
//                 // assume as Unreal config struct
//                 println!("{k}: {:?}", unreal_struct::parse_struct(v));
//             } else {
//                 println!("{k}: {v}");
//             }
//         }
//     }
// }
