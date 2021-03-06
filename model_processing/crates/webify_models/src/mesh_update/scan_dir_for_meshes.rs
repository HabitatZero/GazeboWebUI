//! Scan all files recursively in the specified path for DAE meshes

use std::path::Path;
use std::{fs, path::PathBuf};

use console::style;

/// Orchestrator to scan the specified directory for meshes
pub fn scan_dir_for_meshes(dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    println!("\nScanning for meshes to webify...");
    let meshes = match recursive_scan(dir, Vec::new()) {
        Ok(mesh_list) => mesh_list,
        Err(error) => panic!("Failed to scan all directories for meshes: {:?}", error),
    };

    println!("Meshes found: {}\n", style(meshes.len()).bold().blue());

    Ok(meshes)
}

/// Recursively scan the specified path and return only DAE files
fn recursive_scan(dir: &Path, mut meshes: Vec<PathBuf>) -> std::io::Result<Vec<PathBuf>> {
    // TODO: Dry this up with the image_processing one
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let e = entry?;
            let path = e.path();

            if path.is_dir() {
                meshes = recursive_scan(&path, meshes.clone())?;
            } else {
                let extension = match path.extension() {
                    Some(ext) => ext.to_str().unwrap(),
                    _ => "",
                };

                if extension == "dae" {
                    meshes.push(path);
                };
            }
        }
    }

    Ok(meshes)
}

#[cfg(test)]
mod scan_dir_for_meshes_tests {
    use super::*;

    #[test]
    fn it_returns_meshes() {
        let dir = &Path::new("tests").join("mesh_update").join("test");
        let results = match scan_dir_for_meshes(dir) {
            Ok(mesh_list) => mesh_list,
            Err(error) => panic!("Failed to scan all directories for meshes: {:?}", error),
        };

        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].to_str(),
            Some("tests/mesh_update/test/meshes/test.dae")
        );
    }
}

#[cfg(test)]
mod recursive_scan_tests {
    use super::*;

    #[test]
    fn it_recursively_scans_the_dir() {
        let dir = &Path::new("tests").join("mesh_update").join("test");
        let results = match recursive_scan(dir, Vec::new()) {
            Ok(mesh_list) => mesh_list,
            Err(error) => panic!("Failed to scan all directories for meshes: {:?}", error),
        };

        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].to_str(),
            Some("tests/mesh_update/test/meshes/test.dae")
        );
    }
}
