use minikv::errores::ErrorTipo;
use minikv::protocolo::respuesta_error;
use minikv::servidor::iniciar_servidor;
use std::env;

fn main() {
    let direccion = match parsear_direccion(env::args()) {
        Ok(valor) => valor,
        Err(error) => {
            println!("{}", respuesta_error(&error));
            return;
        }
    };

    if let Err(error) = iniciar_servidor(&direccion) {
        println!("{}", respuesta_error(&error));
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
