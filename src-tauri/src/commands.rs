use tauri::AppHandle;
use crate::models::{Firma, Ansprechsperson, Kontakt};
use crate::db::db_connect;
use rusqlite::params;

// Firma CRUD
#[tauri::command]
pub fn get_firmen(app_handle: AppHandle) -> Result<Vec<Firma>, String> {
    let conn = db_connect(&app_handle)?;
    let mut stmt = conn.prepare("SELECT id, name, strasse, plz, ort, land, website, mail, telefon, fax, facebook, instagram, x_twitter FROM firma").map_err(|e| e.to_string())?;
    let firmen_iter = stmt.query_map([], |row| {
        Ok(Firma {
            id: row.get(0)?,
            name: row.get(1)?,
            strasse: row.get(2)?,
            plz: row.get(3)?,
            ort: row.get(4)?,
            land: row.get(5)?,
            website: row.get(6)?,
            mail: row.get(7)?,
            telefon: row.get(8)?,
            fax: row.get(9)?,
            facebook: row.get(10)?,
            instagram: row.get(11)?,
            x_twitter: row.get(12)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut firmen = Vec::new();
    for firma in firmen_iter {
        firmen.push(firma.map_err(|e| e.to_string())?);
    }
    Ok(firmen)
}

#[tauri::command]
pub fn create_firma(app_handle: AppHandle, firma: Firma) -> Result<i32, String> {
    let conn = db_connect(&app_handle)?;
    conn.execute(
        "INSERT INTO firma (name, strasse, plz, ort, land, website, mail, telefon, fax, facebook, instagram, x_twitter) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
        params![firma.name, firma.strasse, firma.plz, firma.ort, firma.land, firma.website, firma.mail, firma.telefon, firma.fax, firma.facebook, firma.instagram, firma.x_twitter],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn update_firma(app_handle: AppHandle, firma: Firma) -> Result<(), String> {
    let conn = db_connect(&app_handle)?;
    conn.execute(
        "UPDATE firma SET name = ?1, strasse = ?2, plz = ?3, ort = ?4, land = ?5, website = ?6, mail = ?7, telefon = ?8, fax = ?9, facebook = ?10, instagram = ?11, x_twitter = ?12 WHERE id = ?13",
        params![firma.name, firma.strasse, firma.plz, firma.ort, firma.land, firma.website, firma.mail, firma.telefon, firma.fax, firma.facebook, firma.instagram, firma.x_twitter, firma.id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_firma(app_handle: AppHandle, id: i32) -> Result<(), String> {
    let conn = db_connect(&app_handle)?;
    conn.execute("DELETE FROM firma WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// Ansprechsperson CRUD
#[tauri::command]
pub fn get_ansprechspersonen(app_handle: AppHandle) -> Result<Vec<Ansprechsperson>, String> {
    let conn = db_connect(&app_handle)?;
    let mut stmt = conn.prepare("SELECT id, anrede, name, vorname, tel_mobil, tel_direkt, mail, facebook, linkedin, xing, firma_id FROM ansprechsperson").map_err(|e| e.to_string())?;
    let iter = stmt.query_map([], |row| {
        Ok(Ansprechsperson {
            id: row.get(0)?,
            anrede: row.get(1)?,
            name: row.get(2)?,
            vorname: row.get(3)?,
            tel_mobil: row.get(4)?,
            tel_direkt: row.get(5)?,
            mail: row.get(6)?,
            facebook: row.get(7)?,
            linkedin: row.get(8)?,
            xing: row.get(9)?,
            firma_id: row.get(10)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for item in iter {
        result.push(item.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn create_ansprechsperson(app_handle: AppHandle, person: Ansprechsperson) -> Result<i32, String> {
    let conn = db_connect(&app_handle)?;
    conn.execute(
        "INSERT INTO ansprechsperson (anrede, name, vorname, tel_mobil, tel_direkt, mail, facebook, linkedin, xing, firma_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![person.anrede, person.name, person.vorname, person.tel_mobil, person.tel_direkt, person.mail, person.facebook, person.linkedin, person.xing, person.firma_id],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn update_ansprechsperson(app_handle: AppHandle, person: Ansprechsperson) -> Result<(), String> {
    let conn = db_connect(&app_handle)?;
    conn.execute(
        "UPDATE ansprechsperson SET anrede = ?1, name = ?2, vorname = ?3, tel_mobil = ?4, tel_direkt = ?5, mail = ?6, facebook = ?7, linkedin = ?8, xing = ?9, firma_id = ?10 WHERE id = ?11",
        params![person.anrede, person.name, person.vorname, person.tel_mobil, person.tel_direkt, person.mail, person.facebook, person.linkedin, person.xing, person.firma_id, person.id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_ansprechsperson(app_handle: AppHandle, id: i32) -> Result<(), String> {
    let conn = db_connect(&app_handle)?;
    conn.execute("DELETE FROM ansprechsperson WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}

// Kontakt CRUD
#[tauri::command]
pub fn get_kontakte(app_handle: AppHandle) -> Result<Vec<Kontakt>, String> {
    let conn = db_connect(&app_handle)?;
    let mut stmt = conn.prepare("SELECT id, typ, firma_id, ansprechsperson_id, datum, von_zeit, bis_zeit, betreff, text, partizipierende_personen FROM kontakt").map_err(|e| e.to_string())?;
    let iter = stmt.query_map([], |row| {
        Ok(Kontakt {
            id: row.get(0)?,
            typ: row.get(1)?,
            firma_id: row.get(2)?,
            ansprechsperson_id: row.get(3)?,
            datum: row.get(4)?,
            von_zeit: row.get(5)?,
            bis_zeit: row.get(6)?,
            betreff: row.get(7)?,
            text: row.get(8)?,
            partizipierende_personen: row.get(9)?,
        })
    }).map_err(|e| e.to_string())?;

    let mut result = Vec::new();
    for item in iter {
        result.push(item.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
pub fn create_kontakt(app_handle: AppHandle, kontakt: Kontakt) -> Result<i32, String> {
    let conn = db_connect(&app_handle)?;
    conn.execute(
        "INSERT INTO kontakt (typ, firma_id, ansprechsperson_id, datum, von_zeit, bis_zeit, betreff, text, partizipierende_personen) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![kontakt.typ, kontakt.firma_id, kontakt.ansprechsperson_id, kontakt.datum, kontakt.von_zeit, kontakt.bis_zeit, kontakt.betreff, kontakt.text, kontakt.partizipierende_personen],
    ).map_err(|e| e.to_string())?;
    Ok(conn.last_insert_rowid() as i32)
}

#[tauri::command]
pub fn update_kontakt(app_handle: AppHandle, kontakt: Kontakt) -> Result<(), String> {
    let conn = db_connect(&app_handle)?;
    conn.execute(
        "UPDATE kontakt SET typ = ?1, firma_id = ?2, ansprechsperson_id = ?3, datum = ?4, von_zeit = ?5, bis_zeit = ?6, betreff = ?7, text = ?8, partizipierende_personen = ?9 WHERE id = ?10",
        params![kontakt.typ, kontakt.firma_id, kontakt.ansprechsperson_id, kontakt.datum, kontakt.von_zeit, kontakt.bis_zeit, kontakt.betreff, kontakt.text, kontakt.partizipierende_personen, kontakt.id],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_kontakt(app_handle: AppHandle, id: i32) -> Result<(), String> {
    let conn = db_connect(&app_handle)?;
    conn.execute("DELETE FROM kontakt WHERE id = ?1", params![id]).map_err(|e| e.to_string())?;
    Ok(())
}
