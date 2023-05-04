use crear_logs_con_rust::mostrar_un_info;
use log::{debug, error, info, trace, warn};

fn main() {
    env_logger::init();

    error!("Este es un error");

    warn!("Hola soy un warning!");

    info!("Yo soy un info");

    debug!("Mensaje de debug");

    trace!("El de prioridad bajita");

    mostrar_un_info();
}
