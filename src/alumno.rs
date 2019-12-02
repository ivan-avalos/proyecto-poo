use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alumno {
    pub num_control: String,
    pub nombre: String,
    pub semestre: u8,
}

impl Alumno {
    pub fn new(num_control: String, nombre: String, semestre: u8) -> Alumno {
        Alumno {
            num_control,
            nombre,
            semestre,
        }
    }

    pub fn new_from_stdin() -> Alumno {
        loop {
            println!("Introducir datos del alumno:");

            super::utils::p_flush("Número de control: ");
            let mut num_control = String::new();
            io::stdin()
                .read_line(&mut num_control)
                .expect(super::utils::RL_ERROR);
            num_control.pop();

            super::utils::p_flush("Nombre: ");
            let mut nombre = String::new();
            io::stdin()
                .read_line(&mut nombre)
                .expect(super::utils::RL_ERROR);
            nombre.pop();

            super::utils::p_flush("Semestre: ");
            let mut semestre = String::new();
            io::stdin()
                .read_line(&mut semestre)
                .expect(super::utils::RL_ERROR);
            let semestre: u8 = match semestre.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            break Alumno::new(num_control.trim().into(), nombre.trim().into(), semestre);
        }
    }

    pub fn edit (&mut self, nombre: String, semestre: u8) {
        self.nombre = nombre;
        self.semestre = semestre;
    }

    pub fn edit_from_stdin (&mut self) {
        loop {
            println!("Introducir nuevos datos del alumno:");

            super::utils::p_flush("Nombre: ");
            let mut nombre = String::new();
            io::stdin()
                .read_line(&mut nombre)
                .expect(super::utils::RL_ERROR);
            nombre.pop();

            super::utils::p_flush("Semestre: ");
            let mut semestre = String::new();
            io::stdin()
                .read_line(&mut semestre)
                .expect(super::utils::RL_ERROR);
            let semestre: u8 = match semestre.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };

            self.edit(nombre, semestre);
            break;
        }
    }

    pub fn print(&self) {
        println!("Alumno:");
        println!("  – Número de control: {}", self.num_control);
        println!("  – Nombre: {}", self.nombre);
        println!("  – Semestre: {}", self.semestre);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alumnos {
    pub alumnos: Vec<Alumno>,
}

impl Alumnos {
    pub fn new() -> Alumnos {
        Alumnos {
            alumnos: Vec::new(),
        }
    }

    pub fn push(&mut self, alumno: Alumno) {
        self.alumnos.push(alumno);
    }

    pub fn push_from_stdin(&mut self) {
        let alumno = Alumno::new_from_stdin();
        self.push(alumno);
    }

    pub fn edit_from_stdin (&mut self) {
        loop {
            super::utils::p_flush("Número de control: ");
            let mut num_control = String::new();
            io::stdin()
                .read_line(&mut num_control)
                .expect(super::utils::RL_ERROR);
            num_control.pop();

            match self.alumnos.clone()
                .into_iter()
                .find(|x| x.num_control == num_control) {
                    Some(_) => {},
                    None => {
                        println!("[!] El alumno {} no existe.", num_control);
                        continue;
                    
                }
            }

            for alumno in &mut self.alumnos {
                if alumno.num_control == num_control {
                    alumno.edit_from_stdin();
                    break;
                }
            }
            break;
        }
    }

    pub fn print(&self) {
        for alumno in &(self.alumnos) {
            alumno.print();
        }
    }
}

impl Extend<Alumno> for Alumnos {
    fn extend<T: IntoIterator<Item = Alumno>>(&mut self, iter: T) {
        for alumno in iter {
            self.alumnos.push(alumno);
        }
    }
}
