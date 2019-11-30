use std::io;
use std::io::Write;

pub const RL_ERROR: &'static str = "Error al leer línea.";

pub fn p_flush(s: &'static str) {
    print!("{}", s);
    io::stdout().flush().expect("No se pudo enjuagar stdout");
}