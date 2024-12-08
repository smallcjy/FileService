use crate::controller::v1::file::file::FileType;

pub fn parse_filetype(fname: &str) -> FileType {
    let ext = fname.rsplit('.').next().unwrap_or_default();
    FileType::from_str(ext)
}