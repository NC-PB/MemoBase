export type Firma = {
  id?: number;
  name: string;
  strasse?: string;
  plz?: string;
  ort?: string;
  land?: string;
  website?: string;
  mail?: string;
  telefon?: string;
  fax?: string;
  facebook?: string;
  instagram?: string;
  x_twitter?: string;
};

export type Ansprechsperson = {
  id?: number;
  anrede?: string;
  name: string;
  vorname?: string;
  tel_mobil?: string;
  tel_direkt?: string;
  mail?: string;
  facebook?: string;
  linkedin?: string;
  xing?: string;
  firma_id?: number;
};

export type Kontakt = {
  id?: number;
  typ?: string;
  firma_id?: number;
  ansprechsperson_id?: number;
  datum?: string;
  von_zeit?: string;
  bis_zeit?: string;
  betreff?: string;
  text?: string;
  partizipierende_personen?: string;
};
