CREATE TABLE pacientes (
	id SERIAL PRIMARY KEY,
	nombre VARCHAR NOT NULL,
	hipertension TEXT NOT NULL,
	diabetes TEXT NOT NULL,
	cancer TEXT NOT NULL,
	alergias TEXT NOT NULL,
	cardiopatias TEXT NOT NULL,
	otros TEXT NOT NULL
);
