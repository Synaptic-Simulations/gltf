use std::error::Error as StdError;
use std::{fs, path};

const SAMPLE_MODELS_DIRECTORY_PATH: &str = "glTF-Sample-Models/2.0";

fn sanity_check(
    document: &gltf::Document,
    buffer_data: &[gltf::buffer::Data],
    image_data: &[gltf::image::Data],
) -> Result<(), ()> {
    // Check buffers.
    if document.buffers().len() != buffer_data.len() {
        return Err(());
    }
    for (buf, data) in document.buffers().zip(buffer_data.iter()) {
        if ((buf.length() + 3) & !3) != data.0.len() {
            return Err(());
        }
    }

    // Check images.
    if document.images().len() != image_data.len() {
        return Err(());
    }

    // Cool and good.
    Ok(())
}

fn run() -> Result<(), Box<dyn StdError>> {
    let sample_dir_path = path::Path::new(SAMPLE_MODELS_DIRECTORY_PATH);
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                // Import standard glTF.
                let mut gltf_path = entry_path.join("glTF").join(file_name);
                gltf_path.set_extension("gltf");
                {
                    print!("{:?}: ", gltf_path);
                    let result = gltf::import(&gltf_path)?;
                    sanity_check(&result.0, &result.1, &result.2).expect("test failure");
                    println!("ok");
                }

                // Import standard glTF with embedded buffer and image data.
                let mut gle_path = entry_path.join("glTF-Embedded").join(file_name);
                gle_path.set_extension("gltf");
                if gle_path.exists() {
                    print!("{:?}: ", gle_path);
                    let result = gltf::import(&gle_path)?;
                    sanity_check(&result.0, &result.1, &result.2).expect("test failure");
                    println!("ok");
                }

                // Import binary glTF.
                let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
                glb_path.set_extension("glb");
                if glb_path.exists() {
                    print!("{:?}: ", glb_path);
                    let result = gltf::import(&glb_path)?;
                    sanity_check(&result.0, &result.1, &result.2).expect("test failure");
                    println!("ok");
                }
            }
        }
    }

    sparse_accessor_without_buffer_view_test()
}

/// Test a file with a sparse accessor with no buffer view
fn sparse_accessor_without_buffer_view_test() -> Result<(), Box<dyn StdError>> {
    let glb_path = path::Path::new("tests/box_sparse.glb");
    print!("{:?}: ", glb_path);
    let result = gltf::import(&glb_path)?;
    sanity_check(&result.0, &result.1, &result.2).expect("test failure");
    println!("ok");

    let gltf_path = path::Path::new("tests/box_sparse.gltf");
    print!("{:?}: ", gltf_path);
    let result = gltf::import(&gltf_path)?;
    sanity_check(&result.0, &result.1, &result.2).expect("test failure");
    println!("ok");
    Ok(())
}

#[test]
fn import_sanity_check() {
    run().expect("test failure");
}
