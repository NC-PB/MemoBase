use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Firma {
    pub id: Option<i32>,
    pub name: String,
    pub strasse: Option<String>,
    pub plz: Option<String>,
    pub ort: Option<String>,
    pub land: Option<String>,
    pub website: Option<String>,
    pub mail: Option<String>,
    pub telefon: Option<String>,
    pub fax: Option<String>,
    pub facebook: Option<String>,
    pub instagram: Option<String>,
    pub x_twitter: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ansprechsperson {
    pub id: Option<i32>,
    pub anrede: Option<String>,
    pub name: String,
    pub vorname: Option<String>,
    pub tel_mobil: Option<String>,
    pub tel_direkt: Option<String>,
    pub mail: Option<String>,
    pub facebook: Option<String>,
    pub linkedin: Option<String>,
    pub xing: Option<String>,
    pub firma_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Kontakt {
    pub id: Option<i32>,
    pub typ: Option<String>,
    pub firma_id: Option<i32>,
    pub ansprechsperson_id: Option<i32>,
    pub datum: Option<String>,
    pub von_zeit: Option<String>,
    pub bis_zeit: Option<String>,
    pub betreff: Option<String>,
    pub text: Option<String>,
    pub partizipierende_personen: Option<String>,
}
