mod updater;

use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::collections::HashMap;
use std::fs::File;
use crate::updater::to_writer_yaml;

#[derive(Debug, Serialize, Deserialize)]
struct Service {
    image: String,
    ports: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DockerCompose {
    version: String,
    services: HashMap<String, Service>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Ouvrir le fichier yaml en mode lecture
    let file = File::open("docker-compose.yaml")?;

    // Utiliser la fonction from_reader pour désérialiser le contenu du fichier yaml
    let docker_compose: DockerCompose = from_reader(file)?;

    // Accéder aux données
    println!("Version: {}", docker_compose.version);

    for (service_name, service) in &docker_compose.services {
        println!("Service: {}", service_name);
        println!("  Image: {}", service.image);
        if let Some(ports) = &service.ports {
            println!("  Ports: {:?}", ports);
        }
    }

    to_writer_yaml()
}