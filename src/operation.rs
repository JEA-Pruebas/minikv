//! # Módulo de Operaciones
//!
//! Este módulo define las acciones que pueden modificarse en el almacenamiento (`Store`).
//! Sigue el patrón de comando para encapsular los cambios de estado.

use crate::store::Store;

/// Representa una acción que se puede realizar sobre el `Store`.
pub enum Operation {
    /// Guarda o actualiza un par clave-valor.
    Set {
        /// La clave a modificar o agregar.
        clave: String,
        /// El valor a asociar con la clave.
        valor: String,
    },
    /// Elimina una clave del store.
    Unset {
        /// La clave a eliminar.
        clave: String,
    },
}

impl Operation {
    /// Ejecuta la operación sobre el `Store`, modificando su estado.
    ///
    /// - Para `Set`, guarda o actualiza la clave con el valor dado.
    /// - Para `Unset`, elimina la clave del store si existe.
    pub fn ejecutar_en_store(self, store: &mut Store) {
        match self {
            Operation::Set { clave, valor } => store.set(clave, valor),
            Operation::Unset { clave } => store.unset(&clave),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Operation;
    use crate::store::Store;

    #[test]
    fn guardo_valor_en_store() {
        let mut store = Store::new();
        let operacion = Operation::Set {
            clave: "clave1".to_string(),
            valor: "valor1".to_string(),
        };

        operacion.ejecutar_en_store(&mut store);

        assert_eq!(store.get("clave1"), Some(&"valor1".to_string()));
    }

    #[test]
    fn elimino_clave_del_store() {
        let mut store = Store::new();
        store.set("clave1".to_string(), "valor1".to_string());

        let operacion = Operation::Unset {
            clave: "clave1".to_string(),
        };

        operacion.ejecutar_en_store(&mut store);

        assert_eq!(store.get("clave1"), None);
    }

    #[test]
    fn reemplazo_valor_existente() {
        let mut store = Store::new();
        store.set("clave1".to_string(), "valor1".to_string());

        let operacion = Operation::Set {
            clave: "clave1".to_string(),
            valor: "valor2".to_string(),
        };

        operacion.ejecutar_en_store(&mut store);

        assert_eq!(store.get("clave1"), Some(&"valor2".to_string()));
        assert_eq!(store.length(), 1);
    }
}
