use rusqlite::Connection;
use std::fs;
use tauri::Manager;

pub fn db_connect(app_handle: &tauri::AppHandle) -> Result<Connection, String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_data_dir.join("memobase.sqlite");
    Connection::open(db_path).map_err(|e| e.to_string())
}

pub fn initialize_database(app_handle: &tauri::AppHandle) -> Result<Connection, String> {
    let app_data_dir = app_handle.path().app_data_dir().map_err(|e| e.to_string())?;
    
    // Create the directory if it doesn't exist
    if !app_data_dir.exists() {
        fs::create_dir_all(&app_data_dir).map_err(|e| e.to_string())?;
    }
    
    let db_path = app_data_dir.join("memobase.sqlite");
    
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    
    // Initialize tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS firma (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            strasse TEXT,
            plz TEXT,
            ort TEXT,
            land TEXT,
            website TEXT,
            mail TEXT,
            telefon TEXT,
            fax TEXT,
            facebook TEXT,
            instagram TEXT,
            x_twitter TEXT
        )",
        (),
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS ansprechsperson (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            anrede TEXT,
            name TEXT NOT NULL,
            vorname TEXT,
            tel_mobil TEXT,
            tel_direkt TEXT,
            mail TEXT,
            facebook TEXT,
            linkedin TEXT,
            xing TEXT,
            firma_id INTEGER REFERENCES firma(id) ON DELETE SET NULL
        )",
        (),
    ).map_err(|e| e.to_string())?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS kontakt (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            typ TEXT,
            firma_id INTEGER REFERENCES firma(id) ON DELETE CASCADE,
            ansprechsperson_id INTEGER REFERENCES ansprechsperson(id) ON DELETE SET NULL,
            datum TEXT,
            von_zeit TEXT,
            bis_zeit TEXT,
            betreff TEXT,
            text TEXT,
            partizipierende_personen TEXT
        )",
        (),
    ).map_err(|e| e.to_string())?;

    Ok(conn)
}
