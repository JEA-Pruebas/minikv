use crate::errores::ErrorTipo;
use crate::persistence::cargar_store;
use crate::protocolo::{parsear_linea_comando, respuesta_error};
use crate::servicio::ejecutar_comando_en_store;
use crate::store::Store;
use std::io::{BufRead, BufReader, ErrorKind, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

const TIMEOUT_MS: u64 = 5000;

/// Inicia el servidor TCP de minikv y atiende conexiones entrantes.
pub fn iniciar_servidor(direccion: &str) -> Result<(), ErrorTipo> {
    let store = cargar_store()?;
    let listener = TcpListener::bind(direccion).map_err(|_| ErrorTipo::ServerSocketBinding)?;
    let estado = Arc::new(Mutex::new(store));

    for stream in listener.incoming() {
        match stream {
            Ok(stream_ok) => lanzar_hilo_cliente(stream_ok, Arc::clone(&estado)),
            Err(_) => eprintln!("{}", respuesta_error(&ErrorTipo::ConnectionClosed)),
        }
    }

    Ok(())
}

fn lanzar_hilo_cliente(stream: TcpStream, estado: Arc<Mutex<Store>>) {
    let _ = thread::Builder::new()
        .name("minikv-cliente".to_string())
        .spawn(move || manejar_cliente(stream, estado));
}

fn manejar_cliente(mut stream: TcpStream, estado: Arc<Mutex<Store>>) {
    if configurar_timeout(&stream).is_err() {
        eprintln!("{}", respuesta_error(&ErrorTipo::Timeout));
        return;
    }

    let mut lector = match crear_lector_stream(&stream) {
        Ok(lector_ok) => lector_ok,
        Err(error) => {
            eprintln!("{}", respuesta_error(&error));
            return;
        }
    };

    loop {
        let linea = match leer_linea_cliente(&mut lector) {
            Ok(Some(linea_ok)) => linea_ok,
            Ok(None) => return,
            Err(error) => {
                eprintln!("{}", respuesta_error(&error));
                return;
            }
        };

        if let Err(error) = responder_linea(&mut stream, &estado, &linea) {
            eprintln!("{}", respuesta_error(&error));
            return;
        }
    }
}

fn configurar_timeout(stream: &TcpStream) -> Result<(), ErrorTipo> {
    let timeout_escritura = Some(Duration::from_millis(TIMEOUT_MS));

    stream
        .set_write_timeout(timeout_escritura)
        .map_err(|_| ErrorTipo::Timeout)?;

    Ok(())
}

fn procesar_linea_recibida(linea: &str, estado: &Arc<Mutex<Store>>) -> String {
    let comando = match parsear_linea_comando(linea.trim()) {
        Ok(comando_ok) => comando_ok,
        Err(error) => return respuesta_error(&error),
    };

    let mut store = match estado.lock() {
        Ok(guardia) => guardia,
        Err(_) => return respuesta_error(&ErrorTipo::ConnectionClosed),
    };

    match ejecutar_comando_en_store(comando, &mut store) {
        Ok(respuesta) => respuesta,
        Err(error) => respuesta_error(&error),
    }
}

fn enviar_respuesta(stream: &mut TcpStream, respuesta: &str) -> Result<(), ErrorTipo> {
    writeln!(stream, "{}", respuesta).map_err(mapear_error_io)
}

fn responder_linea(
    stream: &mut TcpStream,
    estado: &Arc<Mutex<Store>>,
    linea: &str,
) -> Result<(), ErrorTipo> {
    let respuesta = procesar_linea_recibida(linea, estado);
    enviar_respuesta(stream, &respuesta)
}

fn crear_lector_stream(stream: &TcpStream) -> Result<BufReader<TcpStream>, ErrorTipo> {
    stream
        .try_clone()
        .map(BufReader::new)
        .map_err(|_| ErrorTipo::ConnectionClosed)
}

fn leer_linea_cliente(lector: &mut BufReader<TcpStream>) -> Result<Option<String>, ErrorTipo> {
    let mut linea = String::new();
    let leidos = lector.read_line(&mut linea).map_err(mapear_error_io)?;

    if leidos == 0 {
        return Ok(None);
    }

    Ok(Some(linea))
}

fn mapear_error_io(error: std::io::Error) -> ErrorTipo {
    match error.kind() {
        ErrorKind::WouldBlock | ErrorKind::TimedOut => ErrorTipo::Timeout,
        _ => ErrorTipo::ConnectionClosed,
    }
}
