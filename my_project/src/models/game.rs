use crate::models::score::add_score;
use crate::models::user::User;
use rusqlite::Connection;

struct Game {
    id: i64,
    name: String,
}

pub fn add_game(conn: &Connection, game_name: &str) {
    if get_game_by_name(conn, game_name).is_none() {
        conn.execute("INSERT INTO games (name) VALUES (?1)", &[&game_name])
            .expect("Erreur lors de l'ajout du jeu");
    }
}

// Définir une fonction pour chaque jeu
pub fn play_game(
    conn: &Connection,
    user: &User,
    game_name_to_search: &str,
) -> Result<i64, &'static str> {
    // recuperer jeu dans BDD
    if let Some(game) = get_game_by_name(conn, game_name_to_search) {
        println!("Lancement du jeu pour l'utilisateur {} !", user.name);
        // Logique du jeu à lancer
        let score: i64;
        match game.name.as_str() {
            "Jeu1" => {
                score = game1::play_game();
            }
            "Jeu2" => {
                score = game2::play_game();
            }
            "Jeu3" => {
                score = game3::play_game();
            }
            _ => return Err("Game not found!"),
        }

        // Enregistrement du score dans la base de données
        add_score(conn, user.id, game.id, score)
            .map_err(|_| "Erreur lors de l'enregistrement du score")?;
        Ok(score)
    } else {
        Err("Game not found!")
    }
}

fn get_game_by_name(conn: &Connection, name: &str) -> Option<Game> {
    conn.query_row(
        "SELECT id, name FROM games WHERE name = ?1",
        &[&name],
        |row| {
            Ok(Game {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        },
    )
        .ok()
}

pub const GAMES: [&str; 3] = ["Jeu1", "Jeu2", "Jeu3"];