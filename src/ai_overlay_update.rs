use crate::ai_overlay_state::{
    AiOverlayConnection, fetch_ai_overlay_status, normalize_ai_overlay_service_url,
};
use crate::app_state::TradingTerminal;
use crate::message::Message;
use iced::Task;

impl TradingTerminal {
    pub(crate) fn update_ai_overlay(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::AiOverlayServiceUrlChanged(value) => {
                self.ai_overlay_service_url_input = value;
            }
            Message::SaveAiOverlayServiceUrl => {
                match normalize_ai_overlay_service_url(&self.ai_overlay_service_url_input) {
                    Ok(url) => {
                        self.ai_overlay_service_url = url;
                        self.ai_overlay.request_generation =
                            self.ai_overlay.request_generation.wrapping_add(1);
                        self.ai_overlay.connection = AiOverlayConnection::Disconnected;
                        self.ai_overlay.detail = None;
                        self.persist_config();
                        return self.refresh_ai_overlay();
                    }
                    Err(error) => {
                        self.ai_overlay.connection = AiOverlayConnection::Error;
                        self.ai_overlay.detail = Some(error);
                    }
                }
            }
            Message::AiOverlayRefresh => return self.refresh_ai_overlay(),
            Message::AiOverlayLoaded(generation, result) => {
                if generation != self.ai_overlay.request_generation {
                    return Task::none();
                }
                match result {
                    Ok(Some(decision)) => {
                        self.ai_overlay.connection = AiOverlayConnection::DecisionAvailable;
                        self.ai_overlay.detail = Some(decision);
                    }
                    Ok(None) => {
                        self.ai_overlay.connection = AiOverlayConnection::NoDecision;
                        self.ai_overlay.detail = None;
                    }
                    Err(error) => {
                        self.ai_overlay.connection = AiOverlayConnection::Disconnected;
                        self.ai_overlay.detail = Some(error);
                    }
                }
            }
            _ => {}
        }
        Task::none()
    }

    pub(crate) fn refresh_ai_overlay(&mut self) -> Task<Message> {
        let generation = self.ai_overlay.request_generation.wrapping_add(1);
        self.ai_overlay.request_generation = generation;
        self.ai_overlay.connection = AiOverlayConnection::Checking;
        let service_url = self.ai_overlay_service_url.clone();
        Task::perform(fetch_ai_overlay_status(service_url), move |result| {
            Message::AiOverlayLoaded(generation, result)
        })
    }
}
