use std::io::{BufRead, BufReader, ErrorKind, Read, Write};
use mio::{Poll, Token};
use mio::net::TcpStream;
use std::io::{ Result as IoResult};

use crate::error::LogError;

#[derive(Debug)]
pub struct ConnectionHandler {
    pub stream: TcpStream,
    pub token: Token,
}

impl ConnectionHandler {
    pub fn new(stream: TcpStream, token: Token) -> Self {
        ConnectionHandler { stream, token }
    }

    pub fn handle_event(&mut self, poll: &mut Poll, event: &mio::event::Event) {
        if event.is_readable() {
            self.read_event();
        }
        if event.is_writable() {
            self.write_event();
        }
    }

    pub fn read_event(&mut self) {
        let mut buffer = [0; 1024];
        let mut head = String::new();
        let mut body = Vec::new();
        let mut is_body = false;
        let mut buf_reader = BufReader::new(&mut self.stream);

        loop {
            // Lire les données depuis le stream
            match buf_reader.read(&mut buffer) {
                Ok(bytes_read) => {
                    // Fin de la connexion si aucun octet n'a été lu
                    if bytes_read == 0 {
                        if is_body {
                            break;
                        } else {
                            return;
                        }
                    }

                    // Convertir le tampon en chaîne
                    let chunk = match String::from_utf8(buffer[..bytes_read].to_vec()) {
                        Ok(chunk) => chunk,
                        Err(_) => {
                            // Gestion des données non UTF-8, traiter les données brutes
                            String::from_utf8_lossy(&buffer[..bytes_read]).into()
                        },
                    };

                    // Vérifier la présence de la fin de l'en-tête
                    if let Some(index) = chunk.find("\r\n\r\n") {
                        if !is_body {
                            // Extraire l'en-tête et le corps
                            head.push_str(&chunk[..index]);
                            body.extend_from_slice(&buffer[index + 4..bytes_read]);
                            is_body = true;
                        } else {
                            body.extend_from_slice(&buffer[..bytes_read]);
                        }
                        break;
                    } else {
                        if is_body {
                            // Ajouter au corps si déjà en mode corps
                            body.extend_from_slice(&buffer[..bytes_read]);
                        } else {
                            // Ajouter à l'en-tête si en mode en-tête
                            head.push_str(&chunk);
                        }
                    }
                },
                Err(err) => match err.kind() {
                    ErrorKind::ConnectionReset => {
                        LogError::new(format!("Connection reset by peer: {:?}", err)).log();
                        return;
                    },
                    _ => {
                        LogError::new(format!("Error reading from connection: {:?}", err)).log();
                        return;
                    }
                },
            }
        }

        // Lire le reste du corps s'il y en a
        while let Ok(bytes_read) = buf_reader.read(&mut buffer) {
            if bytes_read == 0 {
                break;
            }
            body.extend_from_slice(&buffer[..bytes_read]);
        }

        // Afficher la requête reçue
        println!("Request Header: {:#?}", head);
        println!("Request Body: {:#?}", body);
    }

    pub fn write_event(&mut self) {
        // Gérer l'écriture de données sur le stream
        // Placeholder pour l'écriture, peut-être une réponse HTTP
        let response = b"HTTP/1.1 200 OK\r\nContent-Length: 10\r\n\r\nalpapierer";
        match self.stream.write_all(response) {
            Ok(_) => println!("Response sent successfully"),
            Err(err) => {
                LogError::new(format!("Error writing response: {:?}", err)).log();
            }
        }
    }
}
