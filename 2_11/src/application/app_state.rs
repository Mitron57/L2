use crate::domain;
use std::error::Error;

type Repository = dyn domain::Repository<Error = Box<dyn Error>> + Send + Sync;
type EventService = dyn domain::EventService<Error = Box<dyn Error>> + Sync + Send;
pub struct AppState {
    pub repository: Box<Repository>,
    pub event_service: Box<EventService>,
}

impl AppState {
    pub fn new(repository: Box<Repository>, event_service: Box<EventService>) -> Self {
        AppState {
            repository,
            event_service,
        }
    }
}
