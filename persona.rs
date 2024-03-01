#[derive(Debug)]
pub struct Persona {
    nombre: String,
    edad: u8,
    genero: Genero,
}

#[derive(Debug)]
pub enum Genero {
    Masculino,
    Femenino,
    Otro,
}

impl Persona {
    pub fn nueva(nombre: &str, edad: u8, genero: Genero) -> Self {
        Self {
            nombre: nombre.to_string(),
            edad,
            genero,
        }
    }

    pub fn obtener_nombre(&self) -> &str {
        &self.nombre
    }

    pub fn obtener_edad(&self) -> u8 {
        self.edad
    }

    pub fn obtener_genero(&self) -> &Genero {
        &self.genero
    }
}
