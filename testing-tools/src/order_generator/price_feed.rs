use std::{sync::Arc, time::Duration};

use cex_exchanges::{
    binance::ws::{channels::BinanceBookTicker, BinanceWsMessage},
    normalized::{
        types::RawTradingPair,
        ws::{CombinedWsMessage, NormalizedExchangeBuilder, NormalizedWsChannelKinds}
    },
    CexExchange
};
use futures::StreamExt;
use tokio::sync::{broadcast, RwLock};

#[derive(Debug, Clone)]
pub struct PriceFeed {
    symbol:    String,
    update_tx: broadcast::Sender<BinanceBookTicker>
}

impl Default for PriceFeed {
    fn default() -> Self {
        let (update_tx, _) = broadcast::channel(100);
        Self { symbol: "ethusdc".to_string(), update_tx }
    }
}

impl PriceFeed {
    pub fn new(symbol: String) -> Self {
        let (update_tx, _) = broadcast::channel(100);
        Self { symbol, update_tx }
    }

    pub async fn start(&self) -> Result<(), PriceFeedError> {
        let mut builder = NormalizedExchangeBuilder::new();
        builder.add_pairs_single_channel_all_exchanges(
            &[CexExchange::Binance],
            NormalizedWsChannelKinds::Quotes,
            &[RawTradingPair::RawNoDelim { pair: self.symbol.clone() }]
        );
        let mut stream = builder
            .build_all_multistream(Some(3), Some(1))
            .unwrap()
            .unwrap();

        let mut last_update_time = tokio::time::Instant::now();

        while let Some(message) = stream.next().await {
            match message {
                CombinedWsMessage::Binance(BinanceWsMessage::BookTicker(book_ticker)) => {
                    // intentionally throttled for testing
                    if last_update_time.elapsed() >= Duration::from_secs(1) {
                        self.update_tx
                            .send(book_ticker)
                            .map_err(|_| PriceFeedError::UpdateSendError)?;
                        last_update_time = tokio::time::Instant::now();
                    }
                }
                e => {
                    tracing::error!("unhandled message {:?}", e);
                }
            }
        }
        Ok(())
    }

    pub fn subscribe(&self) -> broadcast::Receiver<BinanceBookTicker> {
        self.update_tx.subscribe()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum PriceFeedError {
    #[error("Synchronization has already been started")]
    SyncAlreadyStarted,
    #[error("Synchronization has not been started")]
    SyncNotStarted,
    #[error("Failed to send update")]
    UpdateSendError
}
