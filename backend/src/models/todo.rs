// 1. Makro Serde: Serialize, Deserialize
// Serialize: Mengubah struktur Rust (ingatan) menjadi JSON String agar bisa dikirim ke Browser/Mobile.
// Deserialize: Mengubah JSON String dari request user menjadi struktur Rust agar bisa diproses oleh kode.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    //2. Definisi Field & Tipe Data Todo
    pub id: Option<i32>,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// 3. Create: Field wajib ada (kecuali description yang memang boleh kosong)
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTodoRequest {
    // 4. Update: Field boleh kosong (kecuali title yang wajib ada)
    pub title: Option<String>,
    pub description: Option<String>,
    pub completed: Option<bool>,
}
