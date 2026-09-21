use data_model::{NewUser, User};
use log::info;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    info!("worker started");

    // Fila simulada de eventos NewUser.
    let mut queue = vec![
        NewUser { name: "Bob".into() },
        NewUser { name: "Carol".into() },
        NewUser { name: "Dave".into() },
    ];

    let mut next_id = 1000u64;

    while let Some(ev) = queue.pop() {
        // "Processa" evento convertendo em User persistido
        let user = User::from_new(ev, next_id)?;
        next_id += 1;
        info!("processed user: id={} name={}", user.id, user.name);

        // Simula latência/espera de fila
        sleep(Duration::from_millis(500)).await;
    }

    info!("worker finished");
    Ok(())
}
