use super::chat::handler::ChatHandler;
use super::game::handler::GameActionHandler;
use super::search::handler::UserSearchHandler;
use crate::common::{ClientRequest, GameAction};
use crate::websockets::api::challenges::handler::ChallengeHandler;
use crate::websockets::api::ping::handler::PingHandler;
use crate::websockets::api::tournaments::handler::TournamentHandler;
use crate::websockets::api::user_status::handler::UserStatusHandler;
use crate::websockets::auth_error::AuthError;
use crate::websockets::chat::Chats;
use crate::websockets::internal_server_message::InternalServerMessage;
use crate::websockets::messages::WsMessage;
use crate::websockets::tournament_game_start::TournamentGameStart;
use anyhow::Result;
use db_lib::DbPool;
use shared_types::{ChatDestination, SimpleUser};
use uuid::Uuid;

pub struct RequestHandler {
    command: ClientRequest,
    chat_storage: actix_web::web::Data<Chats>,
    game_start: actix_web::web::Data<TournamentGameStart>,
    received_from: actix::Recipient<WsMessage>, // This is the socket the message was received over
    pool: DbPool,
    user_id: Uuid,
    username: String,
    authed: bool,
    admin: bool,
}

impl RequestHandler {
    pub fn new(
        command: ClientRequest,
        chat_storage: actix_web::web::Data<Chats>,
        game_start: actix_web::web::Data<TournamentGameStart>,
        sender_addr: actix::Recipient<WsMessage>,
        user: SimpleUser,
        pool: DbPool,
    ) -> Self {
        Self {
            received_from: sender_addr,
            command,
            chat_storage,
            game_start,
            pool,
            user_id: user.user_id,
            username: user.username,
            authed: user.authed,
            admin: user.admin,
        }
    }

    fn ensure_auth(&self) -> Result<()> {
        if !self.authed {
            Err(AuthError::Unauthorized)?
        }
        Ok(())
    }

    fn ensure_admin(&self) -> Result<()> {
        if !self.admin {
            Err(AuthError::Unauthorized)?
        }
        Ok(())
    }

    pub async fn handle(&self) -> Result<Vec<InternalServerMessage>> {
        let messages = match self.command.clone() {
            ClientRequest::UserSearch(pattern) => {
                UserSearchHandler::new(self.user_id, pattern, &self.pool)
                    .handle()
                    .await?
            }
            ClientRequest::Chat(message_container) => {
                self.ensure_auth()?;
                if self.user_id != message_container.message.user_id {
                    Err(AuthError::Unauthorized)?
                }
                if message_container.destination == ChatDestination::Global {
                    self.ensure_admin()?;
                }
                ChatHandler::new(message_container, self.chat_storage.clone()).handle()
            }
            ClientRequest::Tournament(tournament_action) => {
                TournamentHandler::new(
                    tournament_action,
                    &self.username,
                    self.user_id,
                    self.chat_storage.clone(),
                    &self.pool,
                )
                .await?
                .handle()
                .await?
            }
            ClientRequest::Ping(sent) => PingHandler::new(self.user_id, sent).handle(),
            ClientRequest::Game {
                action: game_action,
                game_id,
            } => {
                match game_action {
                    GameAction::Turn(_) | GameAction::Control(_) => self.ensure_auth()?,
                    _ => {}
                };
                GameActionHandler::new(
                    &game_id,
                    game_action,
                    (&self.username,self.user_id),
                    self.received_from.clone(),
                    self.chat_storage.clone(),
                    self.game_start.clone(),
                    &self.pool,
                )
                .await?
                .handle()
                .await?
            }
            ClientRequest::Challenge(challenge_action) => {
                self.ensure_auth()?;
                ChallengeHandler::new(challenge_action, &self.username, self.user_id, &self.pool)
                    .await?
                    .handle()
                    .await?
            }
            ClientRequest::Away => UserStatusHandler::new().await?.handle().await?,
        };
        Ok(messages)
    }
}
