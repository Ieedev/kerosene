use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Hyperliquid Network
// ---------------------------------------------------------------------------

/// Trusted Hyperliquid environments. URLs are deliberately not user-configurable
/// so a selected network always binds reads, streams, and signed actions together.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum HyperliquidNetwork {
    #[default]
    Mainnet,
    Testnet,
}

impl HyperliquidNetwork {
    pub(crate) const ALL: [Self; 2] = [Self::Mainnet, Self::Testnet];
    pub(crate) const L1_CHAIN_ID: u64 = 1337;

    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Mainnet => "Mainnet",
            Self::Testnet => "TESTNET - simulated funds",
        }
    }

    pub(crate) fn info_url(self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.hyperliquid.xyz/info",
            Self::Testnet => "https://api.hyperliquid-testnet.xyz/info",
        }
    }

    pub(crate) fn exchange_url(self) -> &'static str {
        match self {
            Self::Mainnet => "https://api.hyperliquid.xyz/exchange",
            Self::Testnet => "https://api.hyperliquid-testnet.xyz/exchange",
        }
    }

    pub(crate) fn ws_url(self) -> &'static str {
        match self {
            Self::Mainnet => "wss://api.hyperliquid.xyz/ws",
            Self::Testnet => "wss://api.hyperliquid-testnet.xyz/ws",
        }
    }

    pub(crate) fn phantom_agent_source(self) -> &'static str {
        match self {
            Self::Mainnet => "a",
            Self::Testnet => "b",
        }
    }

    pub(crate) fn is_testnet(self) -> bool {
        matches!(self, Self::Testnet)
    }
}

impl std::fmt::Display for HyperliquidNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

#[cfg(test)]
mod tests {
    use super::HyperliquidNetwork;

    #[test]
    fn networks_use_distinct_trusted_endpoints_and_sources() {
        assert_ne!(
            HyperliquidNetwork::Mainnet.info_url(),
            HyperliquidNetwork::Testnet.info_url()
        );
        assert_ne!(
            HyperliquidNetwork::Mainnet.exchange_url(),
            HyperliquidNetwork::Testnet.exchange_url()
        );
        assert_ne!(
            HyperliquidNetwork::Mainnet.ws_url(),
            HyperliquidNetwork::Testnet.ws_url()
        );
        assert_eq!(HyperliquidNetwork::Mainnet.phantom_agent_source(), "a");
        assert_eq!(HyperliquidNetwork::Testnet.phantom_agent_source(), "b");
    }
}
