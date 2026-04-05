use crate::command::Command;
use crate::errores::ErrorTipo;

/// Parsea una línea textual del protocolo y la convierte a un `Command`.
pub fn parsear_linea_comando(linea: &str) -> Result<Command, ErrorTipo> {
    Command::parsear_linea(linea)
}

/// Devuelve la respuesta estándar de éxito del protocolo.
pub fn respuesta_ok() -> String {
    "OK".to_string()
}

/// Formatea una respuesta de error según el protocolo.
///
/// Ejemplo: `ERROR "NOT FOUND"`.
pub fn respuesta_error(error: &ErrorTipo) -> String {
    format!("ERROR \"{}\"", motivo_error(error))
}

/// Devuelve el motivo textual de error para el protocolo.
pub fn motivo_error(error: &ErrorTipo) -> &'static str {
    match error {
        ErrorTipo::NotFound => "NOT FOUND",
        ErrorTipo::ExtraArgument => "EXTRA ARGUMENT",
        ErrorTipo::InvalidDataFile => "INVALID DATA FILE",
        ErrorTipo::InvalidLogFile => "INVALID LOG FILE",
        ErrorTipo::MissingArgument => "MISSING ARGUMENT",
        ErrorTipo::UnknownCommand => "UNKNOWN COMMAND",
        ErrorTipo::InvalidArgs => "INVALID ARGS",
        ErrorTipo::ServerSocketBinding => "SERVER SOCKET BINDING",
        ErrorTipo::ClientSocketBinding => "CLIENT SOCKET BINDING",
        ErrorTipo::Timeout => "TIMEOUT",
        ErrorTipo::ConnectionClosed => "CONNECTION CLOSED",
    }
}

#[cfg(test)]
mod tests {
    use super::{parsear_linea_comando, respuesta_error};
    use crate::command::Command;
    use crate::errores::ErrorTipo;

    #[test]
    fn parsea_comando_set_desde_linea() {
        let comando = parsear_linea_comando("set nombre joaquin");

        match comando {
            Ok(Command::Set { clave, valor }) => {
                assert_eq!(clave, "nombre");
                assert_eq!(valor, "joaquin");
            }
            _ => panic!("Se esperaba Command::Set"),
        }
    }

    #[test]
    fn formatea_error_para_protocolo() {
        let respuesta = respuesta_error(&ErrorTipo::ExtraArgument);

        assert_eq!(respuesta, "ERROR \"EXTRA ARGUMENT\"");
    }
}
