use crate::persona::Persona;

pub trait InformacionPersona {
    fn imprimir_informacion(&self);
    fn es_mayor_de_edad(&self) -> bool;
}

impl InformacionPersona for Persona {
    fn imprimir_informacion(&self) {
        println!("Nombre: {}", self.obtener_nombre());
        println!("Edad: {}", self.obtener_edad());
        println!("Género: {:?}", self.obtener_genero());
    }

    fn es_mayor_de_edad(&self) -> bool {
        self.obtener_edad() >= 18
    }
}
