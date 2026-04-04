//! # Módulo Store
//!
//! Este módulo proporciona una abstracción sobre un `HashMap` para gestionar
//! el almacenamiento de pares clave-valor de forma estructurada.

use std::collections::HashMap;
use std::collections::hash_map::Iter;

/// Es la estructura central sobre la cual se aplican las operaciones del sistema.
pub struct Store {
    datos: HashMap<String, String>,
}

impl Store {
    /// Crea un nuevo store vacío.
    pub fn new() -> Self {
        Self {
            datos: HashMap::new(),
        }
    }

    /// Inserta o actualiza el valor asociado a una clave.
    ///
    /// Si la clave ya existe, su valor es reemplazado.
    pub fn set(&mut self, clave: String, valor: String) {
        self.datos.insert(clave, valor);
    }

    /// Elimina una clave del store.
    ///
    /// Si la clave no existe, no realiza ninguna acción.
    pub fn unset(&mut self, clave: &str) {
        self.datos.remove(clave);
    }

    /// Obtiene una referencia al valor asociado a una clave.
    ///
    /// Devuelve `Some(&String)` si la clave existe o `None` en caso contrario.
    pub fn get(&self, clave: &str) -> Option<&String> {
        self.datos.get(clave)
    }

    /// Devuelve la cantidad de claves almacenadas en el store.
    pub fn length(&self) -> usize {
        self.datos.len()
    }

    /// Permite iterar sobre todos los pares clave-valor del store.
    ///
    /// Devuelve un iterador sobre referencias a las claves y valores.
    pub fn iter(&self) -> Iter<'_, String, String> {
        self.datos.iter()
    }
}
/// Implementación del trait `Default` para `Store`.
///
/// Permite crear un store vacío utilizando `Store::default()`.
impl Default for Store {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Store;

    #[test]
    fn store_nuevo_empieza_vacio() {
        let store = Store::new();

        assert_eq!(store.length(), 0);
        assert_eq!(store.get("clave1"), None);
    }

    #[test]
    fn set_guarda_una_clave_y_get_la_recupera() {
        let mut store = Store::new();

        store.set("clave1".to_string(), "valor1".to_string());

        assert_eq!(store.get("clave1"), Some(&"valor1".to_string()));
    }

    #[test]
    fn set_reemplaza_el_valor_si_la_clave_ya_existia() {
        let mut store = Store::new();

        store.set("clave1".to_string(), "valor1".to_string());
        store.set("clave1".to_string(), "valor2".to_string());

        assert_eq!(store.get("clave1"), Some(&"valor2".to_string()));
        assert_eq!(store.length(), 1);
    }

    #[test]
    fn unset_elimina_una_clave_existente() {
        let mut store = Store::new();

        store.set("clave1".to_string(), "valor1".to_string());
        store.unset("clave1");

        assert_eq!(store.get("clave1"), None);
        assert_eq!(store.length(), 0);
    }

    #[test]
    fn unset_de_una_clave_inexistente_no_rompe() {
        let mut store = Store::new();

        store.unset("inexistente");

        assert_eq!(store.length(), 0);
    }

    #[test]
    fn cuenta_cuantas_claves_hay() {
        let mut store = Store::new();

        store.set("clave1".to_string(), "valor1".to_string());
        store.set("clave2".to_string(), "valor2".to_string());

        assert_eq!(store.length(), 2);
    }
}
