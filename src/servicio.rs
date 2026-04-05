use crate::command::Command;
use crate::errores::ErrorTipo;
use crate::operation::Operation;
use crate::persistence::{append_operacion_log, snapshot};
use crate::protocolo::respuesta_ok;
use crate::store::Store;

pub fn ejecutar_comando_en_store(comando: Command, store: &mut Store) -> Result<String, ErrorTipo> {
    match comando {
        Command::Set { clave, valor } => ejecutar_operacion(Operation::Set { clave, valor }, store),
        Command::Unset { clave } => ejecutar_operacion(Operation::Unset { clave }, store),
        Command::Get { clave } => leer_valor(store, &clave),
        Command::Length => Ok(store.length().to_string()),
        Command::Snapshot => {
            snapshot(store)?;
            Ok(respuesta_ok())
        }
    }
}

fn ejecutar_operacion(operacion: Operation, store: &mut Store) -> Result<String, ErrorTipo> {
    append_operacion_log(&operacion)?;
    operacion.ejecutar_en_store(store);
    Ok(respuesta_ok())
}

fn leer_valor(store: &Store, clave: &str) -> Result<String, ErrorTipo> {
    match store.get(clave) {
        Some(valor) => Ok(valor.to_string()),
        None => Err(ErrorTipo::NotFound),
    }
}
