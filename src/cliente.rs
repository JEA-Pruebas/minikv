use crate::errores::ErrorTipo;
use crate::protocolo::respuesta_error;
use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::time::Duration;

const TIMEOUT_MS: u64 = 5000;

pub fn ejecutar_cliente(direccion: &str) -> Result<(), ErrorTipo> {
    let mut stream = TcpStream::connect(direccion).map_err(|_| ErrorTipo::ClientSocketBinding)?;
    configurar_timeout(&stream)?;

    let stream_lectura = stream
        .try_clone()
        .map_err(|_| ErrorTipo::ConnectionClosed)?;

    let mut lector_red = BufReader::new(stream_lectura);
    let stdin = io::stdin();
    let mut lector_stdin = stdin.lock();

    loop {
        let mut linea_entrada = String::new();
        let leidos = lector_stdin
            .read_line(&mut linea_entrada)
            .map_err(|_| ErrorTipo::ConnectionClosed)?;

        if leidos == 0 {
            return Ok(());
        }

        enviar_operacion(&mut stream, linea_entrada.trim())?;
        let respuesta = recibir_respuesta(&mut lector_red)?;
        println!("{}", respuesta.trim_end());
    }
}

fn configurar_timeout(stream: &TcpStream) -> Result<(), ErrorTipo> {
    let timeout = Some(Duration::from_millis(TIMEOUT_MS));

    stream
        .set_read_timeout(timeout)
        .map_err(|_| ErrorTipo::Timeout)?;
    stream
        .set_write_timeout(timeout)
        .map_err(|_| ErrorTipo::Timeout)?;

    Ok(())
}

fn enviar_operacion(stream: &mut TcpStream, operacion: &str) -> Result<(), ErrorTipo> {
    writeln!(stream, "{}", operacion).map_err(|_| ErrorTipo::ConnectionClosed)
}

fn recibir_respuesta(lector_red: &mut BufReader<TcpStream>) -> Result<String, ErrorTipo> {
    let mut respuesta = String::new();

    let leidos = lector_red
        .read_line(&mut respuesta)
        .map_err(|_| ErrorTipo::ConnectionClosed)?;

    if leidos == 0 {
        return Err(ErrorTipo::ConnectionClosed);
    }

    Ok(respuesta)
}

pub fn imprimir_error_cliente(error: &ErrorTipo) {
    println!("{}", respuesta_error(error));
}
