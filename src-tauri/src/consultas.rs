use crate::model::{NuevoPaciente, Paciente, CrearPaciente, ActualizarPaciente};
use crate::db::establish_connection;
use diesel::prelude::*;

#[tauri::command]
pub fn fetch_consultas() -> Vec<Paciente> { 
    use crate::schema::pacientes::dsl::*;
    let connection = &mut establish_connection();

    let results = pacientes
        .load::<Paciente>(connection)
        .expect("Error loading pacientes");

    results
}

#[tauri::command]
pub fn get_consulta(paciente_id: i32) -> Paciente {
    use crate::schema::pacientes::dsl::*;
    let connection = &mut establish_connection();

    let paciente = pacientes
        .find(paciente_id)
        .select(Paciente::as_select())
        .first(connection)
        .unwrap();

    paciente

}

#[tauri::command]
pub fn create_consulta(data: CrearPaciente) {
    use crate::schema::pacientes::dsl::*;
    let connection = &mut establish_connection();

    let nuevo_paciente = NuevoPaciente {
        nombre: &data.nombre,
        hipertension:  &data.hipertension,
        diabetes:  &data.diabetes,
        cancer:  &data.cancer,
        alergias:  &data.alergias,
        cardiopatias:  &data.cardiopatias,
        otros:  &data.otros,
    };

    diesel::insert_into(pacientes)
        .values(&nuevo_paciente)
        .execute(connection)
        .expect("Error al guardar el nuevo paciente");
}

#[tauri::command]
pub fn update_consulta(data: ActualizarPaciente) {
    use crate::schema::pacientes::dsl::*;
    let connection = &mut establish_connection();

    let db_paciente = Paciente {
        id: data.id,
        nombre: data.nombre,
        hipertension:  data.hipertension,
        diabetes:  data.diabetes,
        cancer:  data.cancer,
        alergias:  data.alergias,
        cardiopatias:  data.cardiopatias,
        otros:  data.otros,
    };

    diesel::update(pacientes.find(data.id))
        .set(&db_paciente)
        .execute(connection)
        .expect("Error al actualizar el paciente");
}

#[tauri::command]
pub fn delete_consulta(paciente_id: i32){
    use crate::schema::pacientes::dsl::*;

    let connection = &mut establish_connection();
    diesel::delete(pacientes.filter(id.eq(paciente_id)))
        .execute(connection)
        .expect("Error al eliminar paciente");
}
