use super::*;
use crate::hyperliquid_network::HyperliquidNetwork;
use serde_json::json;
use std::collections::HashMap;
use std::time::Duration;

mod reconnect;
mod stale_read;
mod timeout;

const DEBUG_ADDRESS: &str = "0xabc0000000000000000000000000000000000000";

fn test_manager() -> WsManager {
    let (cmd_tx, _) = mpsc::unbounded_channel();
    let (msg_tx, _) = broadcast::channel(1);
    WsManager {
        cmd_tx: WsCommandSender::new_for_test(cmd_tx),
        msg_tx,
    }
}

#[test]
fn managers_are_isolated_by_network() {
    let mut managers = HashMap::new();
    manager_for_network(&mut managers, HyperliquidNetwork::Mainnet, test_manager);
    manager_for_network(&mut managers, HyperliquidNetwork::Testnet, test_manager);

    assert_eq!(managers.len(), 2);
    assert!(managers.contains_key(&HyperliquidNetwork::Mainnet));
    assert!(managers.contains_key(&HyperliquidNetwork::Testnet));
}

#[test]
fn manager_and_latency_probe_urls_follow_network() {
    assert_eq!(
        ws_manager_url(HyperliquidNetwork::Testnet),
        HyperliquidNetwork::Testnet.ws_url()
    );
    assert_eq!(
        api_latency_probe_url(HyperliquidNetwork::Testnet),
        HyperliquidNetwork::Testnet.info_url()
    );
    assert_ne!(
        ws_manager_url(HyperliquidNetwork::Mainnet),
        ws_manager_url(HyperliquidNetwork::Testnet)
    );
    assert_ne!(
        api_latency_probe_url(HyperliquidNetwork::Mainnet),
        api_latency_probe_url(HyperliquidNetwork::Testnet)
    );
}

#[test]
fn ws_command_debug_redacts_user_subscription_payload() {
    let command = WsCommand::Subscribe {
        topic: format!("userFills:{DEBUG_ADDRESS}"),
        payload: json!({
            "method": "subscribe",
            "subscription": {
                "type": "userFills",
                "user": DEBUG_ADDRESS,
                "token": "payload-token"
            }
        }),
    };

    let rendered = format!("{command:?}");

    assert!(rendered.contains("<redacted>"));
    assert!(rendered.contains("subscription_type: Some(\"userFills\")"));
    assert!(!rendered.contains(DEBUG_ADDRESS));
    assert!(!rendered.contains("payload-token"));
}

#[test]
fn ws_routed_message_debug_redacts_raw_data() {
    let message = WsRoutedMessage {
        channel: "userFills".to_string(),
        data: Arc::new(json!({
            "user": DEBUG_ADDRESS,
            "fills": [{ "hash": "fill-secret" }]
        })),
    };

    let rendered = format!("{message:?}");

    assert!(rendered.contains("<redacted>"));
    assert!(!rendered.contains(DEBUG_ADDRESS));
    assert!(!rendered.contains("fill-secret"));
}

struct PendingWriteSink;

impl futures::Sink<WsMsg> for PendingWriteSink {
    type Error = ();

    fn poll_ready(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Pending
    }

    fn start_send(self: std::pin::Pin<&mut Self>, _item: WsMsg) -> Result<(), Self::Error> {
        Ok(())
    }

    fn poll_flush(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Pending
    }

    fn poll_close(
        self: std::pin::Pin<&mut Self>,
        _cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        std::task::Poll::Pending
    }
}

#[tokio::test]
async fn ws_text_send_times_out_for_pending_sink() {
    let mut sink = PendingWriteSink;

    assert!(!send_ws_text_with_timeout(&mut sink, "{}".to_string()).await);
}
