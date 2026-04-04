//! # Manejo de Errores del Sistema
//!
//! Se definen los tipos de errores especificados que pueden ocurrir
//! durante la ejecución de comandos y el procesamiento y persistencia de archivos.

/// Estos errores cubren tanto problemas de entrada del usuario como
/// inconsistencias en los archivos persistidos.
pub enum ErrorTipo {
    /// La clave solicitada no existe en el store.
    NotFound,
    /// Se recibió una cantidad de argumentos mayor a la esperada.
    ExtraArgument,
    /// El archivo de datos es inválido o no se pudo leer correctamente.
    InvalidDataFile,
    /// El archivo de registro es inválido o no se pudo leer correctamente.
    InvalidLogFile,
    /// Faltan parámetros obligatorios requeridos para ejecutar el comando.
    MissingArgument,
    /// Comando desconocido.
    UnknownCommand,
}

impl ErrorTipo {
    /// Retorna una cadena de texto con el mensaje de error formateado.
    ///
    /// Este mensaje es el que se debe mostrar al usuario por consola (stdout/stderr).
    pub fn mensaje(&self) -> &'static str {
        match self {
            ErrorTipo::NotFound => "ERROR: NOT FOUND",
            ErrorTipo::ExtraArgument => "ERROR: EXTRA ARGUMENT",
            ErrorTipo::InvalidDataFile => "ERROR: INVALID DATA FILE",
            ErrorTipo::InvalidLogFile => "ERROR: INVALID LOG FILE",
            ErrorTipo::MissingArgument => "ERROR: MISSING ARGUMENT",
            ErrorTipo::UnknownCommand => "ERROR: UNKNOWN COMMAND",
        }
    }
}
