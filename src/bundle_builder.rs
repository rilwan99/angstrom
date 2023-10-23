use std::{
    pin::Pin,
    task::{Context, Poll}
};

use common::PollExt;
use futures::{stream::FuturesUnordered, Future, Stream};
use futures_util::StreamExt;
use sim::Simulator;
use tokio::task::JoinError;
use tracing::warn;

pub enum BuildingResults {}

pub struct BundleBuilder<S: Simulator + Unpin + 'static> {
    sim:          S,
    construction:
        FuturesUnordered<Pin<Box<dyn Future<Output = Result<BuildingResults, JoinError>>>>>
}

impl<S: Simulator + Unpin + 'static> BundleBuilder<S> {
    pub fn new(sim: S) -> Self {
        Self { sim, construction: FuturesUnordered::default() }
    }
}

impl<S: Simulator + Unpin + 'static> Stream for BundleBuilder<S> {
    type Item = BuildingResults;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.construction.poll_next_unpin(cx).filter_map(|res| {
            res.transpose()
                .inspect_err(|e| warn!(?e, "failed to build bundle"))
                .ok()
        })
    }
}
