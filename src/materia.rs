use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Materia {
    pub clave: String,
    pub nombre: String,
    pub creditos: u8,
}

impl Materia {
    pub fn new(clave: String, nombre: String, creditos: u8) -> Materia {
        Materia {
            clave: clave,
            nombre: nombre,
            creditos: creditos,
        }
    }

    pub fn new_from_stdin() -> Materia {
        return loop {
            println!("Introducir datos de la materia:");

            super::utils::p_flush("Clave: ");
            let mut clave = String::new();
            io::stdin()
                .read_line(&mut clave)
                .expect(super::utils::RL_ERROR);
            clave.pop();

            super::utils::p_flush("Nombre: ");
            let mut nombre = String::new();
            io::stdin()
                .read_line(&mut nombre)
                .expect(super::utils::RL_ERROR);
            nombre.pop();

            super::utils::p_flush("Créditos: ");
            let mut creditos = String::new();
            io::stdin()
                .read_line(&mut creditos)
                .expect(super::utils::RL_ERROR);
            let creditos: u8 = match creditos.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            break Materia::new(clave.trim().into(), nombre.trim().into(), creditos);
        };
    }

    pub fn edit (&mut self, nombre: String, creditos: u8) {
        self.nombre = nombre;
        self.creditos = creditos;
    }

    pub fn edit_from_stdin (&mut self) {
        return loop {
            println!("Introducir datos de la materia:");

            super::utils::p_flush("Nombre: ");
            let mut nombre = String::new();
            io::stdin()
                .read_line(&mut nombre)
                .expect(super::utils::RL_ERROR);
            nombre.pop();

            super::utils::p_flush("Créditos: ");
            let mut creditos = String::new();
            io::stdin()
                .read_line(&mut creditos)
                .expect(super::utils::RL_ERROR);
            let creditos: u8 = match creditos.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
            self.edit(nombre, creditos);
            break;
        };
    }

    pub fn print(&self) {
        println!("Materia:");
        println!("  – Clave: {}", self.clave);
        println!("  – Nombre: {}", self.nombre);
        println!("  – Créditos: {}", self.creditos);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Materias {
    pub materias: Vec<Materia>,
}

impl Materias {
    pub fn new() -> Materias {
        Materias {
            materias: Vec::new(),
        }
    }

    pub fn push(&mut self, materia: Materia) {
        self.materias.push(materia);
    }

    pub fn push_from_stdin(&mut self) {
        let materia = Materia::new_from_stdin();
        self.push(materia);
    }

    pub fn edit_from_stdin(&mut self) {
        loop {
            super::utils::p_flush("Número de control: ");
            let mut clave = String::new();
            io::stdin().read_line(&mut clave)
                .expect(super::utils::RL_ERROR);
            clave.pop();

            match self.materias.clone()
                .into_iter()
                .find(|x| x.clave == clave) {
                    Some(_) => {},
                    None => {
                        println!("[!] La materia {} no existe", clave);
                        continue;
                    }
                }
            
            for materia in &mut self.materias {
                if materia.clave == clave {
                    materia.edit_from_stdin();
                    break;
                }
            }
            break;
        }
    }

    pub fn print(&self) {
        for materia in &(self.materias) {
            materia.print();
        }
    }
}

impl Extend<Materia> for Materias {
    fn extend<T: IntoIterator<Item = Materia>>(&mut self, iter: T) {
        for elem in iter {
            self.materias.push(elem);
        }
    }
}
