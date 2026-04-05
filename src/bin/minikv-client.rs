use minikv::cliente::{ejecutar_cliente, imprimir_error_cliente};
use minikv::errores::ErrorTipo;
use std::env;

fn main() {
    let direccion = match parsear_direccion(env::args()) {
        Ok(valor) => valor,
        Err(error) => {
            imprimir_error_cliente(&error);
            return;
        }
    };

    if let Err(error) = ejecutar_cliente(&direccion) {
        imprimir_error_cliente(&error);
    }
}

fn parsear_direccion<I>(args: I) -> Result<String, ErrorTipo>
where
    I: IntoIterator<Item = String>,
{
    let mut iter = args.into_iter();
    let _programa = iter.next();

    let Some(direccion) = iter.next() else {
        return Err(ErrorTipo::InvalidArgs);
    };

    if iter.next().is_some() {
        return Err(ErrorTipo::InvalidArgs);
    }

    Ok(direccion)
}
