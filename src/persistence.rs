//! # Módulo de Persistencia
//!
//! Este módulo gestiona el ciclo de vida de los datos en disco utilizando dos estrategias:
//! 1. Registra cada operación en `.minikv.log`.
//! 2. Guarda el estado actual del `Store` en `.minikv.data` para un inicio rápido.

use crate::errores::ErrorTipo;
use crate::operation::Operation;
use crate::store::Store;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};

/// Ruta al archivo de registro de operaciones (Log).
const LOG_PATH: &str = ".minikv.log";
/// Ruta al archivo de snapshot persistido (Data).
const DATA_PATH: &str = ".minikv.data";

/// Agrega una operación al archivo de log.
///
/// La operación se serializa en el formato definido por el sistema y se escribe
/// al final de `.minikv.log`.
///
/// # Errores
///
/// Devuelve `InvalidLogFile` si no se puede escribir el archivo de log.
pub fn append_operacion_log(operacion: &Operation) -> Result<(), ErrorTipo> {
    let linea = formatear_operacion_log(operacion);
    append_linea(LOG_PATH, &linea).map_err(|_| ErrorTipo::InvalidLogFile)
}

/// Carga el estado actual del store a partir de los archivos persistidos.
///
/// Primero intenta reconstruir el estado base desde `.minikv.data` y luego
/// aplica las operaciones registradas en `.minikv.log`.
///
/// Si alguno de los archivos contiene contenido inválido o no puede leerse,
/// devuelve el error correspondiente.
pub fn cargar_store() -> Result<Store, ErrorTipo> {
    let mut store = Store::new();

    let data_valida = cargar_desde_archivo(&mut store, DATA_PATH, parsear_linea_data)
        .map_err(|_| ErrorTipo::InvalidDataFile)?;

    if !data_valida {
        return Err(ErrorTipo::InvalidDataFile);
    }

    let log_valido = cargar_desde_archivo(&mut store, LOG_PATH, parsear_linea_log)
        .map_err(|_| ErrorTipo::InvalidLogFile)?;

    if !log_valido {
        return Err(ErrorTipo::InvalidLogFile);
    }

    Ok(store)
}

/// Genera un snapshot del estado actual del store.
///
/// Escribe todo el contenido del store en `.minikv.data` y luego vacía
/// el archivo `.minikv.log`.
///
/// # Errores
///
/// - `InvalidDataFile` si no puede escribirse el snapshot.
/// - `InvalidLogFile` si no puede vaciarse el log.
pub fn snapshot(store: &Store) -> Result<(), ErrorTipo> {
    escribir_snapshot(store).map_err(|_| ErrorTipo::InvalidDataFile)?;
    truncar_archivo(LOG_PATH).map_err(|_| ErrorTipo::InvalidLogFile)?;
    Ok(())
}

fn append_linea(path: &str, linea: &str) -> std::io::Result<()> {
    let mut archivo = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(archivo, "{}", linea)?;

    Ok(())
}

fn cargar_desde_archivo(
    store: &mut Store,
    path: &str,
    parser: fn(&str) -> Option<Operation>,
) -> std::io::Result<bool> {
    let Some(archivo) = abrir_si_existe(path)? else {
        return Ok(true);
    };

    let reader = BufReader::new(archivo);

    for linea in reader.lines() {
        let contenido = linea?;

        let Some(operacion) = parser(&contenido) else {
            return Ok(false);
        };

        operacion.ejecutar_en_store(store);
    }

    Ok(true)
}

fn abrir_si_existe(path: &str) -> std::io::Result<Option<File>> {
    match File::open(path) {
        Ok(archivo) => Ok(Some(archivo)),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error),
    }
}

fn parsear_linea_log(linea: &str) -> Option<Operation> {
    let mut campos = separar_campos(linea).into_iter();

    let comando = campos.next()?;
    if comando != "set" {
        return None;
    }

    let clave = campos.next()?;

    match campos.next() {
        Some(valor) => {
            if campos.next().is_some() {
                return None;
            }

            Some(Operation::Set { clave, valor })
        }
        None => Some(Operation::Unset { clave }),
    }
}

fn parsear_linea_data(linea: &str) -> Option<Operation> {
    let mut campos = separar_campos(linea).into_iter();

    let clave = campos.next()?;
    let valor = campos.next()?;

    if campos.next().is_some() {
        return None;
    }

    Some(Operation::Set { clave, valor })
}

fn escribir_snapshot(store: &Store) -> std::io::Result<()> {
    let mut archivo = File::create(DATA_PATH)?;

    for (clave, valor) in store.iter() {
        let linea = formatear_linea_data(clave, valor);
        writeln!(archivo, "{}", linea)?;
    }

    Ok(())
}

fn truncar_archivo(path: &str) -> std::io::Result<()> {
    File::create(path)?;
    Ok(())
}

fn escapar_string(texto: &str) -> String {
    texto.replace('\\', "\\\\").replace('"', "\\\"")
}

fn formatear_operacion_log(operacion: &Operation) -> String {
    match operacion {
        Operation::Set { clave, valor } => {
            let clave = escapar_string(clave);
            let valor = escapar_string(valor);
            format!("set \"{}\" \"{}\"", clave, valor)
        }
        Operation::Unset { clave } => {
            let clave = escapar_string(clave);
            format!("set \"{}\"", clave)
        }
    }
}

fn formatear_linea_data(clave: &str, valor: &str) -> String {
    let clave = escapar_string(clave);
    let valor = escapar_string(valor);

    format!("\"{}\" \"{}\"", clave, valor)
}

fn procesar_caracter(
    caracter: char,
    actual: &mut String,
    en_comillas: &mut bool,
    escape: &mut bool,
) -> bool {
    match caracter {
        _ if *escape => {
            actual.push(caracter);
            *escape = false;
            false
        }
        '\\' => {
            *escape = true;
            false
        }
        '"' => {
            *en_comillas = !*en_comillas;
            false
        }
        _ if caracter.is_whitespace() && !*en_comillas => !actual.is_empty(),
        _ => {
            actual.push(caracter);
            false
        }
    }
}

fn separar_campos(linea: &str) -> Vec<String> {
    let mut campos = Vec::new();
    let mut actual = String::new();
    let mut en_comillas = false;
    let mut escape = false;

    for caracter in linea.chars() {
        if procesar_caracter(caracter, &mut actual, &mut en_comillas, &mut escape) {
            campos.push(actual);
            actual = String::new();
        }
    }

    if escape || en_comillas {
        return Vec::new();
    }
    if !actual.is_empty() {
        campos.push(actual);
    }
    campos
}

#[cfg(test)]
mod tests {
    use super::parsear_linea_data;
    use super::parsear_linea_log;
    use super::separar_campos;
    use crate::operation::Operation;

    #[test]
    fn parsea_campos_con_espacios() {
        let campos = separar_campos("set \"hola mundo\" \"chau mundo\"");

        assert_eq!(
            campos,
            vec![
                "set".to_string(),
                "hola mundo".to_string(),
                "chau mundo".to_string()
            ]
        );
    }

    #[test]
    fn parsea_campos_con_comillas_escapadas() {
        let campos = separar_campos("set \"clave1 \\\"A\\\"\" \"valor \\\"A\\\"\"");

        assert_eq!(
            campos,
            vec![
                "set".to_string(),
                "clave1 \"A\"".to_string(),
                "valor \"A\"".to_string()
            ]
        );
    }

    #[test]
    fn parsea_linea_log_set() {
        let operacion = parsear_linea_log("set \"clave1\" \"valor1\"");

        match operacion {
            Some(Operation::Set { clave, valor }) => {
                assert_eq!(clave, "clave1");
                assert_eq!(valor, "valor1");
            }
            _ => panic!("Se esperaba Operation::Set"),
        }
    }

    #[test]
    fn parsea_linea_log_unset() {
        let operacion = parsear_linea_log("set \"clave1\"");

        match operacion {
            Some(Operation::Unset { clave }) => assert_eq!(clave, "clave1"),
            _ => panic!("Se esperaba Operation::Unset"),
        }
    }

    #[test]
    fn parsea_linea_data_como_set() {
        let operacion = parsear_linea_data("\"edad\" \"25\"");

        match operacion {
            Some(Operation::Set { clave, valor }) => {
                assert_eq!(clave, "edad");
                assert_eq!(valor, "25");
            }
            _ => panic!("Se esperaba Operation::Set"),
        }
    }
}
