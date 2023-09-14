use axum::extract::{Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};

use crate::ctx::Ctx;
use crate::models::{ModelController, Ticket, TicketContent};
use crate::Result;

pub fn routes(mc: ModelController) -> Router {
    return Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc);
}

async fn create_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Json(ticket_content): Json<TicketContent>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket", "HANDLER");

    return Ok(Json(mc.create_ticket(ctx, ticket_content).await?));
}

async fn list_tickets(State(mc): State<ModelController>, ctx: Ctx) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    return Ok(Json(mc.list_tickets(ctx).await?));
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    ctx: Ctx,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - delete_ticket", "HANDLER");

    return Ok(Json(mc.delete_ticket(ctx, id).await?));
}
