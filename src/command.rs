//! # CLIs del Sistema
//!
//! Representa un comando válido ingresado por el usuario a través de la línea de comandos.

use crate::errores::ErrorTipo;

/// Cada variante corresponde a una operación que puede realizar el sistema.
pub enum Command {
    /// Guarda o actualiza una clave con un valor.
    Set { clave: String, valor: String },

    /// Elimina una clave.
    Unset { clave: String },

    /// Consulta el valor de una clave.
    Get { clave: String },

    /// Devuelve la cantidad de claves almacenadas.
    Length,

    /// Genera un snapshot del estado actual y vacía el log.
    Snapshot,
}

impl Command {
    /// Parsea los parámetros de la línea de comandos y construye un `Command`.
    ///
    /// # Errores
    ///
    /// - `MissingArgument` si faltan argumentos obligatorios.
    /// - `ExtraArgument` si hay argumentos de más.
    /// - `UnknownCommand` si el comando no es reconocido.
    pub fn parsear<I>(args: I) -> Result<Self, ErrorTipo>
    where
        I: IntoIterator<Item = String>,
    {
        let mut iter = args.into_iter();

        let _programa = iter.next();

        let Some(nombre) = iter.next() else {
            return Err(ErrorTipo::MissingArgument);
        };

        parsear_nombre_comando(nombre, iter)
    }

    /// Parsea un comando textual recibido por red.
    pub fn parsear_linea(linea: &str) -> Result<Self, ErrorTipo> {
        let partes = linea
            .split_whitespace()
            .map(|parte| parte.to_string())
            .collect::<Vec<String>>();

        if partes.is_empty() {
            return Err(ErrorTipo::MissingArgument);
        }

        let nombre = partes[0].clone();
        let iter = partes.into_iter().skip(1);

        parsear_nombre_comando(nombre, iter)
    }
}

fn parsear_nombre_comando<I>(nombre: String, iter: I) -> Result<Command, ErrorTipo>
where
    I: Iterator<Item = String>,
{
    match nombre.as_str() {
        "set" => parsear_set(iter),
        "get" => parsear_get(iter),
        "length" => parsear_sin_args(iter, Command::Length),
        "snapshot" => parsear_sin_args(iter, Command::Snapshot),
        _ => Err(ErrorTipo::UnknownCommand),
    }
}

fn parsear_set<I>(mut iter: I) -> Result<Command, ErrorTipo>
where
    I: Iterator<Item = String>,
{
    let Some(clave) = iter.next() else {
        return Err(ErrorTipo::MissingArgument);
    };

    match iter.next() {
        Some(valor) => {
            if iter.next().is_some() {
                return Err(ErrorTipo::ExtraArgument);
            }

            Ok(Command::Set { clave, valor })
        }
        None => Ok(Command::Unset { clave }),
    }
}

fn parsear_get<I>(mut iter: I) -> Result<Command, ErrorTipo>
where
    I: Iterator<Item = String>,
{
    let Some(clave) = iter.next() else {
        return Err(ErrorTipo::MissingArgument);
    };

    if iter.next().is_some() {
        return Err(ErrorTipo::ExtraArgument);
    }

    Ok(Command::Get { clave })
}

fn parsear_sin_args<I>(mut iter: I, comando: Command) -> Result<Command, ErrorTipo>
where
    I: Iterator<Item = String>,
{
    if iter.next().is_some() {
        return Err(ErrorTipo::ExtraArgument);
    }

    Ok(comando)
}

#[cfg(test)]
mod tests {
    use super::Command;

    #[test]
    fn parsea_set_con_valor() {
        let args = vec![
            "minikv".to_string(),
            "set".to_string(),
            "clave1".to_string(),
            "valor1".to_string(),
        ];

        let comando = Command::parsear(args);

        match comando {
            Ok(Command::Set { clave, valor }) => {
                assert_eq!(clave, "clave1");
                assert_eq!(valor, "valor1");
            }
            _ => panic!("Se esperaba Command::Set"),
        }
    }

    #[test]
    fn parsea_unset() {
        let args = vec![
            "minikv".to_string(),
            "set".to_string(),
            "clave1".to_string(),
        ];

        let comando = Command::parsear(args);

        match comando {
            Ok(Command::Unset { clave }) => assert_eq!(clave, "clave1"),
            _ => panic!("Se esperaba Command::Unset"),
        }
    }

    #[test]
    fn parsea_get() {
        let args = vec![
            "minikv".to_string(),
            "get".to_string(),
            "clave1".to_string(),
        ];

        let comando = Command::parsear(args);

        match comando {
            Ok(Command::Get { clave }) => assert_eq!(clave, "clave1"),
            _ => panic!("Se esperaba Command::Get"),
        }
    }

    #[test]
    fn parsea_length() {
        let args = vec!["minikv".to_string(), "length".to_string()];

        let comando = Command::parsear(args);

        assert!(matches!(comando, Ok(Command::Length)));
    }

    #[test]
    fn parsea_snapshot() {
        let args = vec!["minikv".to_string(), "snapshot".to_string()];

        let comando = Command::parsear(args);

        assert!(matches!(comando, Ok(Command::Snapshot)));
    }

    #[test]
    fn parsea_linea_get() {
        let comando = Command::parsear_linea("get edad");

        match comando {
            Ok(Command::Get { clave }) => assert_eq!(clave, "edad"),
            _ => panic!("Se esperaba Command::Get"),
        }
    }
}
