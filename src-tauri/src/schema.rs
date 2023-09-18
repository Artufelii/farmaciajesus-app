// @generated automatically by Diesel CLI.

diesel::table! {
    consultas (id) {
        id -> Int4,
        talla -> Varchar,
        peso -> Varchar,
        presion -> Varchar,
        temperatura -> Varchar,
        oxigenacion -> Varchar,
        diagnostico -> Text,
        tratamiento -> Text,
        estudios -> Bool,
        estudios_desc -> Text,
        paciente_id -> Int4,
    }
}

diesel::table! {
    pacientes (id) {
        id -> Int4,
        nombre -> Varchar,
        hipertension -> Text,
        diabetes -> Text,
        cancer -> Text,
        alergias -> Text,
        cardiopatias -> Text,
        otros -> Text,
    }
}

diesel::joinable!(consultas -> pacientes (paciente_id));

diesel::allow_tables_to_appear_in_same_query!(
    consultas,
    pacientes,
);
