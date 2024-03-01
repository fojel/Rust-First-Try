mod persona;
mod informacion_persona;

use persona::{Genero, Persona};
use informacion_persona::InformacionPersona;

fn main() {
    let persona1 = Persona::nueva("Juan", 25, Genero::Masculino);
    let persona2 = Persona::nueva("María", 17, Genero::Femenino);

    println!("Información de persona 1:");
    persona1.imprimir_informacion();
    println!("Es mayor de edad: {}", persona1.es_mayor_de_edad());

    println!();

    println!("Información de persona 2:");
    persona2.imprimir_informacion();
    println!("Es mayor de edad: {}", persona2.es_mayor_de_edad());
}
