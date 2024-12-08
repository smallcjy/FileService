use uuid::Uuid;

/// call python code for get pdf cover 
pub fn get_pdf_cover(fuuid: Uuid) -> Result<(), actix_web::Error> {
    let output = match std::process::Command::new("python3")
        .arg("parse_pdf_cover.py")
        .arg(fuuid.to_string())
        .output() {
            Ok(output) => output,
            Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string())),
        };
    
    match output.status.success() {
        true => Ok(()),
        false => return Err(actix_web::error::ErrorInternalServerError("Failed to get pdf cover!".to_string())),
    }   
}