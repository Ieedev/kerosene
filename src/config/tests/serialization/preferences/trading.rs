use super::{
    KeroseneConfig, default_config_value, default_market_slippage_pct, json_string, remove_field,
    value_from_json, value_from_str,
};
use crate::config::HyperliquidNetwork;

#[test]
fn legacy_config_without_market_slippage_uses_default() {
    let mut value = default_config_value();
    remove_field(
        &mut value,
        "market_slippage_pct",
        "config should serialize to object",
    );

    let config: KeroseneConfig = value_from_json(value, "legacy config should deserialize");

    assert_eq!(config.market_slippage_pct, default_market_slippage_pct());
}

#[test]
fn network_and_local_ai_service_preferences_round_trip_with_safe_legacy_defaults() {
    let config = KeroseneConfig {
        hyperliquid_network: HyperliquidNetwork::Testnet,
        ai_overlay_service_url: "http://localhost:9000".to_string(),
        ..KeroseneConfig::default()
    };

    let json = json_string(&config, "config should serialize");
    let decoded: KeroseneConfig = value_from_str(&json, "config should deserialize");
    assert_eq!(decoded.hyperliquid_network, HyperliquidNetwork::Testnet);
    assert_eq!(decoded.ai_overlay_service_url, "http://localhost:9000");

    let mut legacy = default_config_value();
    remove_field(
        &mut legacy,
        "hyperliquid_network",
        "config should serialize to object",
    );
    remove_field(
        &mut legacy,
        "ai_overlay_service_url",
        "config should serialize to object",
    );
    let decoded_legacy: KeroseneConfig =
        value_from_json(legacy, "legacy config should deserialize");
    assert_eq!(
        decoded_legacy.hyperliquid_network,
        HyperliquidNetwork::Mainnet
    );
    assert_eq!(
        decoded_legacy.ai_overlay_service_url,
        crate::ai_overlay_state::DEFAULT_AI_OVERLAY_SERVICE_URL
    );
}

#[test]
fn order_quantity_denomination_round_trips_and_legacy_defaults_coin() {
    let config = KeroseneConfig {
        order_quantity_is_usd: true,
        ..KeroseneConfig::default()
    };

    let json = json_string(&config, "config should serialize");
    let decoded: KeroseneConfig = value_from_str(&json, "config should deserialize");
    assert!(decoded.order_quantity_is_usd);

    let mut legacy = default_config_value();
    remove_field(
        &mut legacy,
        "order_quantity_is_usd",
        "config should serialize to object",
    );
    let decoded_legacy: KeroseneConfig =
        value_from_json(legacy, "legacy config should deserialize");
    assert!(!decoded_legacy.order_quantity_is_usd);
}

#[test]
fn optimistic_account_updates_round_trips_and_legacy_defaults_off() {
    let config = KeroseneConfig {
        optimistic_account_updates: true,
        ..KeroseneConfig::default()
    };

    let json = json_string(&config, "config should serialize");
    let decoded: KeroseneConfig = value_from_str(&json, "config should deserialize");
    assert!(decoded.optimistic_account_updates);

    let mut legacy = default_config_value();
    remove_field(
        &mut legacy,
        "optimistic_account_updates",
        "config should serialize to object",
    );
    let decoded_legacy: KeroseneConfig =
        value_from_json(legacy, "legacy config should deserialize");
    assert!(!decoded_legacy.optimistic_account_updates);
}

#[test]
fn hydromancer_realtime_position_pnl_round_trips_and_legacy_defaults_off() {
    let config = KeroseneConfig {
        hydromancer_realtime_position_pnl_enabled: true,
        ..KeroseneConfig::default()
    };

    let json = json_string(&config, "config should serialize");
    let decoded: KeroseneConfig = value_from_str(&json, "config should deserialize");
    assert!(decoded.hydromancer_realtime_position_pnl_enabled);

    let mut legacy = default_config_value();
    remove_field(
        &mut legacy,
        "hydromancer_realtime_position_pnl_enabled",
        "config should serialize to object",
    );
    let decoded_legacy: KeroseneConfig =
        value_from_json(legacy, "legacy config should deserialize");
    assert!(!decoded_legacy.hydromancer_realtime_position_pnl_enabled);
}
