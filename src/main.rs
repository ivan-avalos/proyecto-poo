mod alumno;
mod carga;
mod materia;
mod utils;
use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn existing_warn(filename: &'static str) {
    println!("[!] El archivo {} no existe", filename);
    println!("[!] Se ha creado una colección vacía.");
    println!("[!] Se ha creado un nuevo archivo `{}'", filename);
}

fn main() -> Result<(), Box<dyn Error>> {
    struct DataFiles {
        alumnos: &'static str,
        materias: &'static str,
        carga: &'static str,
    };
    let data_files = DataFiles {
        alumnos: "alumnos.cbor",
        materias: "materias.cbor",
        carga: "carga.cbor",
    };

    // Load data from files
    let mut alumnos: alumno::Alumnos = if Path::new(&*(data_files.materias)).exists() {
        let file = File::open(data_files.alumnos)?;
        serde_cbor::from_reader(file)?
    } else {
        existing_warn(data_files.alumnos);
        alumno::Alumnos::new()
    };

    let mut materias: materia::Materias = if Path::new(&*(data_files.materias)).exists() {
        let file = File::open(data_files.materias)?;
        serde_cbor::from_reader(file)?
    } else {
        existing_warn(data_files.materias);
        materia::Materias::new()
    };

    let mut carga: carga::Carga = if Path::new(&*(data_files.carga)).exists() {
        let file = File::open(data_files.carga)?;
        serde_cbor::from_reader(file)?
    } else {
        existing_warn(data_files.carga);
        carga::Carga::new()
    };

    loop {
        utils::p_flush(
            "Menú:
    1.  Mostrar alumnos.
    2.  Buscar alumno.
    3.  Añadir alumno.
    4.  Modificar alumno.
    5.  Mostrar materias.
    6.  Buscar materia.
    7.  Añadir materia.
    8.  Modificar materia.
    9.  Mostrar carga académica de alumno.
    10. Enrolar alumno a materia.
    11. Exportar carga académica a archivo.
    12. Salir sin guardar.
    13. Guardar y salir.
? "
        );

        // Leer opción de stdin
        let mut opcion = String::new();
        io::stdin()
            .read_line(&mut opcion)
            .expect("Error al leer la línea.");

        let opcion: u32 = match opcion.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Ejecutar acción correspondiente
        match opcion {
            // Mostrar alumnos
            1 => alumnos.print(),
            // Buscar alumno
            2 => alumnos.search_print_from_stdin(),
            // Añadir alumno
            3 => alumnos.push_from_stdin(),
            // Modificar alumno
            4 => alumnos.edit_from_stdin(),
            // Mostrar materias
            5 => materias.print(),
            // Buscar materia
            6 => materias.search_print_from_stdin(),
            // Añadir materia
            7 => materias.push_from_stdin(),
            // Modificar materia
            8 => materias.edit_from_stdin(),
            // Mostrar carga académica de alumno.
            9 => carga.print_from_stdin(materias.clone()),
            // Enrolar alumno a materia
            10 => carga.push_from_stdin(alumnos.clone(), materias.clone()),
            // Exportar carga a txt
            11 => carga.export_carga_from_stdin(alumnos.clone(), materias.clone()),
            // Salir sin guardar
            12 => return Ok(()),
            // Guardar y salir
            13 => {
                let alumnos_output_file = File::create(data_files.alumnos)?;
                let materias_output_file = File::create(data_files.materias)?;
                let carga_output_file = File::create(data_files.carga)?;

                serde_cbor::to_writer(alumnos_output_file, &alumnos)?;
                serde_cbor::to_writer(materias_output_file, &materias)?;
                serde_cbor::to_writer(carga_output_file, &carga)?;

                println!("[!] Se han guardado los datos.");
                return Ok(());
            }
            _ => break,
        }
    }

    Ok(())
}
