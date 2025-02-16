// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use image::{imageops::FilterType, io::Reader as ImageReader, ImageOutputFormat};
use std::fs;
use std::io::Cursor;
use tokio::task;

#[tauri::command]
async fn preview(image_blob: Vec<u8>, width: u32, height: u32, quality: u8) -> Result<(String, String), String> {
    let image_data =
        task::spawn_blocking(move || process_image(image_blob, width, height, quality))
            .await
            .map_err(|e| format!("Failed to process image: {}", e))??;

    let image_size = format_bytes(image_data.len());
    let image_base64 = format!(
        "data:image/jpeg;base64,{}",
        general_purpose::STANDARD.encode(&image_data)
    );

    Ok((image_base64, image_size))
}

#[tauri::command]
async fn convert(image_blob: Vec<u8>, width: u32, height: u32, quality: u8, save_path: String) -> Result<String, String> {
    let image_data =
        task::spawn_blocking(move || process_image(image_blob, width, height, quality))
            .await
            .map_err(|e| format!("Failed to process image: {}", e))??;

    fs::write(&save_path, &image_data).map_err(|e| format!("Failed to save image: {}", e))?;

    Ok(format_bytes(image_data.len()))
}

fn process_image(image_blob: Vec<u8>, width: u32, height: u32, quality: u8) -> Result<Vec<u8>, String> {
    let image_cursor = Cursor::new(image_blob);
    let img = ImageReader::new(image_cursor)
        .with_guessed_format()
        .expect("Cursor io never fails")
        .decode()
        .map_err(|e| format!("Failed to load image: {}", e))?;

    let resized = img.resize(width, height, FilterType::Lanczos3);

    let mut output = Cursor::new(Vec::new());
    resized
        .write_to(&mut output, ImageOutputFormat::Jpeg(quality))
        .map_err(|e| format!("Failed to write image: {}", e))?;

    Ok(output.into_inner())
}

fn format_bytes(size: usize) -> String {
    let size = size as f64;
    const SIZE_UNITS: [&str; 5] = ["B", "kB", "MB", "GB", "TB"];

    if size == 0.0 {
        return "0 B".to_string();
    }

    let digit_groups = (size.log10() / 1024f64.log10()).floor() as i32;

    format!(
        "{:.2} {}",
        size / 1024f64.powi(digit_groups),
        SIZE_UNITS[digit_groups as usize]
    )
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![preview, convert])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
