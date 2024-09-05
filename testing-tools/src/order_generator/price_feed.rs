use std::{sync::Arc, time::Duration};

use cex_exchanges::{
    binance::ws::BinanceWsMessage,
    normalized::{
        types::RawTradingPair,
        ws::{CombinedWsMessage, NormalizedExchangeBuilder, NormalizedWsChannelKinds}
    },
    CexExchange
};
use futures::StreamExt;
use tokio::sync::{broadcast, RwLock};

#[derive(Clone, Debug)]
pub struct PriceLevel {
    pub price:    f64,
    pub quantity: f64
}

#[derive(Debug, Clone)]
pub struct DepthUpdate {
    pub last_updated_id: u64,
    pub bids:            Vec<PriceLevel>,
    pub asks:            Vec<PriceLevel>
}

#[derive(Debug, Clone)]
pub struct PriceFeed {
    symbol:    String,
    cache:     Arc<RwLock<DepthUpdate>>,
    update_tx: broadcast::Sender<DepthUpdate>
}

impl Default for PriceFeed {
    fn default() -> Self {
        let (update_tx, _) = broadcast::channel(100);
        Self {
            symbol: "ethusdc".to_string(),
            cache: Arc::new(RwLock::new(DepthUpdate {
                last_updated_id: 0,
                bids:            Vec::new(),
                asks:            Vec::new()
            })),
            update_tx
        }
    }
}

impl PriceFeed {
    pub fn new(symbol: String) -> Self {
        let (update_tx, _) = broadcast::channel(100);
        Self {
            symbol,
            cache: Arc::new(RwLock::new(DepthUpdate {
                last_updated_id: 0,
                bids:            Vec::new(),
                asks:            Vec::new()
            })),
            update_tx
        }
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
                    let depth_update = DepthUpdate {
                        last_updated_id: book_ticker.orderbook_update_id,
                        bids:            vec![PriceLevel {
                            price:    book_ticker.best_bid_price,
                            quantity: book_ticker.best_bid_amt
                        }],
                        asks:            vec![PriceLevel {
                            price:    book_ticker.best_ask_price,
                            quantity: book_ticker.best_ask_amt
                        }]
                    };
                    tracing::trace!(
                        "price update best bid: {:?} best ask: {:?}",
                        depth_update.bids.first(),
                        depth_update.asks.first()
                    );
                    let mut cache = self.cache.write().await;
                    *cache = depth_update.clone();

                    // intentionally throttled for testing
                    if last_update_time.elapsed() >= Duration::from_secs(1) {
                        self.update_tx
                            .send(depth_update)
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

    pub fn subscribe(&self) -> broadcast::Receiver<DepthUpdate> {
        self.update_tx.subscribe()
    }

    pub async fn price_cache(&self) -> (Vec<PriceLevel>, Vec<PriceLevel>) {
        let cache = self.cache.read().await;
        (cache.bids.clone(), cache.asks.clone())
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
