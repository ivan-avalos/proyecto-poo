use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Write;

// Util functions
pub const RL_ERROR: &'static str = "Error al leer línea.";
pub fn p_flush(s: &'static str) {
    print!("{}", s);
    io::stdout().flush().expect("No se pudo enjuagar stdout");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AlumnoMateria {
    pub num_control: String,
    pub clave_materia: String,
}

impl AlumnoMateria {
    pub fn new(num_control: String, clave_materia: String) -> AlumnoMateria {
        AlumnoMateria {
            num_control,
            clave_materia,
        }
    }

    pub fn new_from_stdin() -> AlumnoMateria {
        println!("Introducir datos de enrolamiento:");

        p_flush("Número de control: ");
        let mut num_control = String::new();
        io::stdin().read_line(&mut num_control).expect(RL_ERROR);
        num_control.pop();

        p_flush("Clave de materia: ");
        let mut clave_materia = String::new();
        io::stdin().read_line(&mut clave_materia).expect(RL_ERROR);
        clave_materia.pop();

        AlumnoMateria::new(num_control.trim().into(), clave_materia.trim().into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Carga {
    pub relaciones: Vec<AlumnoMateria>,
}

impl Carga {
    pub fn new() -> Carga {
        Carga {
            relaciones: Vec::new(),
        }
    }

    pub fn push(&mut self, relacion: AlumnoMateria) {
        self.relaciones.push(relacion);
    }

    pub fn push_from_stdin(&mut self) {
        let relacion = AlumnoMateria::new_from_stdin();
        self.push(relacion);
    }

    pub fn get_for_alumno(
        &self,
        num_control: String,
        materias: super::materia::Materias,
    ) -> Result<Vec<super::materia::Materia>, String> {
        let mut relaciones: Vec<AlumnoMateria> = Vec::new();
        for relacion in &(self.relaciones) {
            relaciones.push(relacion.clone());
        }

        let relaciones: Vec<AlumnoMateria> = relaciones
            .into_iter()
            .filter(|x| num_control == x.num_control)
            .collect();

        if relaciones.len() == 0 {
            let msg = format!(
                "El usuario {} no existe o no tiene materias asignadas.",
                num_control
            );
            println!("[!] {}", msg);
            return Err(msg);
        }

        let mut result: Vec<super::materia::Materia> = Vec::new();
        for relacion in &relaciones {
            result.push(
                match materias
                    .materias
                    .clone()
                    .into_iter()
                    .find(|x| x.clave == relacion.clave_materia)
                {
                    Some(m) => m,
                    None => {
                        println!(
                            "[!] La materia con clave {} no existe.",
                            relacion.clave_materia
                        );
                        continue;
                    }
                },
            );
        }
        return Ok(result);
    }

    pub fn print_for_alumno(&self, num_control: String, materias: super::materia::Materias) {
        match self.get_for_alumno(num_control, materias) {
            Ok(materias) => {
                for materia in &materias {
                    materia.print();
                }
            }
            Err(_) => return,
        }
    }

    pub fn print_from_stdin(&self, materias: super::materia::Materias) {
        p_flush("Número de control del alumno: ");
        let mut num_control = String::new();
        io::stdin().read_line(&mut num_control).expect(RL_ERROR);
        self.print_for_alumno(num_control.trim().into(), materias);
    }

    pub fn export_carga(
        &self,
        alumno: super::alumno::Alumno,
        materias: super::materia::Materias,
        output_path: String,
    ) {
        let mut output = String::new();
        output.push_str("ITC\n");
        output.push_str("Boleta de carga académica\n");
        output.push_str("–––––––––––––––––––––––––––––––––––––\n");
        output.push_str(&*format!("Nombre: {}\n", alumno.nombre));
        output.push_str(&*format!("Número de control: {}\n", alumno.num_control));
        output.push_str(&*format!("Semestre: {}\n", alumno.semestre));

        let mut table = super::Table::new();
        table.add_row(row!["Clave", "Materia", "Créditos"]);

        match self.get_for_alumno(alumno.num_control, materias) {
            Ok(materias) => {
                for materia in &materias {
                    table.add_row(row![materia.clave, materia.nombre, materia.creditos]);
                }
            }
            Err(_) => return,
        };
        output.push_str(&*format!("{}\n", table.to_string()));
        
        let mut file = File::create (output_path)
            .expect("[!] No se pudo abrir/crear el archivo.");
        file.write(output.as_bytes()).expect("[!] No se pudo escribir al archivo.");
    }

    pub fn export_carga_from_stdin(
        &self,
        alumnos: super::alumno::Alumnos,
        materias: super::materia::Materias,
    ) {
        p_flush("Número de control del alumno: ");
        let mut num_control = String::new();
        io::stdin().read_line(&mut num_control).expect(RL_ERROR);
        let num_control: String = num_control.trim().into();
        let output_path = format!("carga_{}.txt", num_control);

        let alumno = match alumnos
            .alumnos
            .clone()
            .into_iter()
            .find(|x| x.num_control == num_control)
        {
            Some(alumno) => alumno,
            None => {
                println!("[!] El alumno {} no existe", num_control);
                return;
            }
        };

        self.export_carga(alumno, materias, output_path);
    }
}
