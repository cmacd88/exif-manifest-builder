use exif::{self, Exif, Reader};
use serde_json;
use std::fs::{self, File};
use std::io::BufReader;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() -> std::io::Result<()> {
    let directory = Path::new("/path/to/your/folder");
    let mut manifest = Vec::new();

    for entry in WalkDir::new(directory) {
        let entry = entry?;

        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();

        let file = File::open(path)?;
        let reader = BufReader::new(&file);

        let exif_reader = Reader::new();
        let exif = exif_reader.read_from_container(&mut reader).ok().and_then(|result| result);

        if let Some(exif_data) = exif {
            let json_data = exif_data.to_json();
            let manifest_entry = json!({
                "path": path.to_str().unwrap(),
                "exif_data": json_data,
            });

            manifest.push(manifest_entry);
        }
    }

    let json_manifest = serde_json::to_string(&manifest)?;

    fs::write("manifest.json", json_manifest)?;

    Ok(())
}
