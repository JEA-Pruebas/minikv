use minikv::command::Command;
use minikv::errores::ErrorTipo;
use minikv::operation::Operation;
use minikv::persistence::{append_operacion_log, cargar_store, snapshot};
use minikv::store::Store;
use std::env;

fn main() {
    let comando = match Command::parsear(env::args()) {
        Ok(comando) => comando,
        Err(error) => {
            println!("{}", error.mensaje());
            return;
        }
    };

    let mut store = match cargar_store() {
        Ok(store) => store,
        Err(error) => {
            println!("{}", error.mensaje());
            return;
        }
    };

    ejecutar_comando(comando, &mut store);
}

fn ejecutar_comando(comando: Command, store: &mut Store) {
    match comando {
        Command::Set { clave, valor } => {
            ejecutar_operacion(Operation::Set { clave, valor }, store);
        }
        Command::Unset { clave } => {
            ejecutar_operacion(Operation::Unset { clave }, store);
        }
        Command::Get { clave } => {
            imprimir_valor(store, &clave);
        }
        Command::Length => {
            println!("{}", store.length());
        }
        Command::Snapshot => match snapshot(store) {
            Ok(()) => println!("OK"),
            Err(error) => println!("{}", error.mensaje()),
        },
    }
}

fn ejecutar_operacion(operacion: Operation, store: &mut Store) {
    match append_operacion_log(&operacion) {
        Ok(()) => {
            operacion.ejecutar_en_store(store);
            println!("OK");
        }
        Err(error) => println!("{}", error.mensaje()),
    }
}

fn imprimir_valor(store: &Store, clave: &str) {
    match store.get(clave) {
        Some(valor) => println!("{}", valor),
        None => println!("{}", ErrorTipo::NotFound.mensaje()),
    }
}
