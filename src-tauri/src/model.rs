use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::{consultas, pacientes};

#[derive(Deserialize, Insertable)]
#[diesel(table_name = pacientes)]
pub struct NuevoPaciente<'a> {
    pub nombre: &'a str,
    pub hipertension:  &'a str,
    pub diabetes:  &'a str,
    pub cancer:  &'a str,
    pub alergias:  &'a str,
    pub cardiopatias:  &'a str,
    pub otros:  &'a str,
}

#[derive(Deserialize, Serialize)]
pub struct CrearPaciente {
    pub nombre: String,
    pub hipertension:  String,
    pub diabetes:  String,
    pub cancer:  String,
    pub alergias:  String,
    pub cardiopatias:  String,
    pub otros:  String,
}

#[derive(Deserialize, Serialize)]
pub struct ActualizarPaciente {
    pub id: i32,
    pub nombre: String,
    pub hipertension:  String,
    pub diabetes:  String,
    pub cancer:  String,
    pub alergias:  String,
    pub cardiopatias:  String,
    pub otros:  String,
}

#[derive(Queryable, Identifiable, AsChangeset, Selectable, Serialize, Debug)]
#[diesel(table_name = pacientes)]
pub struct Paciente {
    pub id: i32,
    pub nombre: String,
    pub hipertension:  String,
    pub diabetes:  String,
    pub cancer:  String,
    pub alergias:  String,
    pub cardiopatias:  String,
    pub otros:  String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = consultas)]
pub struct NuevaConsulta<'a> {
    pub talla: &'a str,
    pub peso: &'a str,
    pub presion: &'a str,
    pub temperatura: &'a str,
    pub oxigenacion: &'a str,
    pub diagnostico: &'a str,
    pub tratamiento: &'a str,
    pub estudios: bool,
    pub estudios_desc: &'a str,
    pub paciente_id: i32,
}

#[derive(Queryable, Identifiable, AsChangeset, Selectable, Serialize, Associations, Debug)]
#[diesel(belongs_to(Paciente))]
#[diesel(table_name = consultas)]
pub struct Consulta {
    pub id: i32,
    pub talla: String,
    pub peso: String,
    pub presion: String,
    pub temperatura: String,
    pub oxigenacion: String,
    pub diagnostico: String,
    pub tratamiento: String,
    pub estudios: bool,
    pub estudios_desc: String,
    pub paciente_id: i32,
}
