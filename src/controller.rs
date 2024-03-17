use actix_web::{web, HttpResponse, Responder, get};
use crate::models::{BlackjackGame, Info};
use askama::Template; // bring trait in scope

#[derive(Template)]
#[template(path = "template.html")]
struct Home<'a> {
    name: &'a str,
}

pub async fn index() -> impl Responder {
    // Create a new Blackjack game instance
    let game = BlackjackGame::new(52);
    println!("Cards: {}",game.get_cards());

    // Render the game HTML template
    let home = Home {name: "Mr John Doe"};
    let html = home.render().unwrap();

    // Return HTML response
    HttpResponse::Ok().body(html)
}

async fn get_value(info: web::Query<Info>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", info.name))
}

