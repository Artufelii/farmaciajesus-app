CREATE TABLE consultas (
	id SERIAL PRIMARY KEY,
	talla VARCHAR NOT NULL,
	peso VARCHAR NOT NULL,
	presion VARCHAR NOT NULL,
	temperatura VARCHAR NOT NULL,
	oxigenacion VARCHAR NOT NULL,
	diagnostico TEXT NOT NULL,
	tratamiento TEXT NOT NULL,
	estudios BOOLEAN NOT NULL DEFAULT FALSE,
	estudios_desc TEXT NOT NULL,
	paciente_id SERIAL REFERENCES pacientes(id)
);
