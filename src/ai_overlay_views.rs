use crate::ai_overlay_state::AiOverlayConnection;
use crate::app_state::TradingTerminal;
use crate::message::Message;
use iced::widget::{button, column, container, text};
use iced::{Color, Element, Length, Theme};

impl TradingTerminal {
    pub(crate) fn view_ai_overlay(&self) -> Element<'_, Message> {
        let theme = self.theme();
        let color = match self.ai_overlay.connection {
            AiOverlayConnection::NoDecision => theme.palette().success,
            AiOverlayConnection::DecisionAvailable => theme.palette().warning,
            AiOverlayConnection::Checking => theme.palette().primary,
            AiOverlayConnection::Disconnected | AiOverlayConnection::Error => {
                theme.palette().danger
            }
        };
        let detail = self
            .ai_overlay
            .detail
            .as_deref()
            .unwrap_or("No decision received. This panel cannot place trades.");

        container(
            column![
                text("LOCAL AI - READ ONLY")
                    .size(11)
                    .color(theme.palette().text),
                text(self.ai_overlay.connection.label())
                    .size(12)
                    .color(color),
                text(detail)
                    .size(11)
                    .color(theme.extended_palette().background.weak.text),
                button(text("Refresh").size(11))
                    .padding([4, 8])
                    .on_press(Message::AiOverlayRefresh),
            ]
            .spacing(5)
            .width(Length::Fixed(260.0)),
        )
        .padding(10)
        .style(move |theme: &Theme| container::Style {
            background: Some(
                Color {
                    a: 0.96,
                    ..theme.extended_palette().background.strong.color
                }
                .into(),
            ),
            border: iced::Border {
                radius: 5.0.into(),
                width: 1.0,
                color,
            },
            ..Default::default()
        })
        .into()
    }
}
