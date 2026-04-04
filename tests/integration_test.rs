use minikv::command::Command;
use minikv::operation::Operation;
use minikv::store::Store;

#[test]
fn integra_set_y_store() {
    let args = vec![
        "minikv".to_string(),
        "set".to_string(),
        "clave1".to_string(),
        "valor1".to_string(),
    ];

    let comando = Command::parsear(args);
    let mut store = Store::new();

    match comando {
        Ok(Command::Set { clave, valor }) => {
            let operacion = Operation::Set { clave, valor };
            operacion.ejecutar_en_store(&mut store);
        }
        _ => panic!("Se esperaba Command::Set"),
    }

    assert_eq!(store.get("clave1"), Some(&"valor1".to_string()));
    assert_eq!(store.length(), 1);
}

#[test]
fn integra_unset_y_store() {
    let args = vec![
        "minikv".to_string(),
        "set".to_string(),
        "clave1".to_string(),
    ];

    let comando = Command::parsear(args);

    let mut store = Store::new();
    store.set("clave1".to_string(), "valor1".to_string());

    match comando {
        Ok(Command::Unset { clave }) => {
            let operacion = Operation::Unset { clave };
            operacion.ejecutar_en_store(&mut store);
        }
        _ => panic!("Se esperaba Command::Unset"),
    }

    assert_eq!(store.get("clave1"), None);
    assert_eq!(store.length(), 0);
}

#[test]
fn integra_parseo_get() {
    let args = vec!["minikv".to_string(), "get".to_string(), "edad".to_string()];

    let comando = Command::parsear(args);

    match comando {
        Ok(Command::Get { clave }) => assert_eq!(clave, "edad"),
        _ => panic!("Se esperaba Command::Get"),
    }
}

#[test]
fn integra_length() {
    let args = vec!["minikv".to_string(), "length".to_string()];

    let comando = Command::parsear(args);

    assert!(matches!(comando, Ok(Command::Length)));
}

#[test]
fn integra_snapshot_parseo() {
    let args = vec!["minikv".to_string(), "snapshot".to_string()];

    let comando = Command::parsear(args);

    assert!(matches!(comando, Ok(Command::Snapshot)));
}
