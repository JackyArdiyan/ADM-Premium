// Mencegah console tambahan muncul di Windows saat aplikasi dijalankan (mode release)
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        // Wajib untuk fitur "Cetak PDF" & "Export Excel" (dialog Save bawaan OS + tulis file
        // ke disk) yang dipanggil dari frontend lewat window.__TAURI__.dialog / .fs
        // (lihat simpanFileUniversal() di app guru_kelas & guru_mapel).
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .run(tauri::generate_context!())
        .expect("error saat menjalankan aplikasi Tauri");
}
