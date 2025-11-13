use serde::{Deserialize, Serialize};
use serde_yaml::{from_reader, to_writer};
use std::collections::HashMap;
use std::fs::File;

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

pub fn to_writer_yaml() -> Result<(), Box<dyn std::error::Error>> {
    let mut services = HashMap::new();
    services.insert(
        "nginx".to_string(),
        Service {
            image: "nginx:latest".to_string(),
            ports: Some(vec!["8080:80".to_string()]),
        },
    );

    let docker_compose = DockerCompose {
        version: "3.8".to_string(),
        services,
    };

    // Ouvrir le fichier yaml en mode écriture
    let file = File::create("docker-compose.yaml")?;

    // Utiliser la fonction to_writer pour écrire dans le fichier yaml
    to_writer(file, &docker_compose)?;

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

    Ok(())
}