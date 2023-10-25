//! A validation service for transactions.

use std::{future::Future, pin::Pin, sync::Arc};

use futures_util::{lock::Mutex, StreamExt};
use reth_primitives::{ChainSpec, SealedBlock};
use reth_tasks::TaskSpawner;
use tokio::{
    sync,
    sync::{mpsc, oneshot}
};
use tokio_stream::wrappers::ReceiverStream;

use crate::{
    validate::{AngstromOrderValidatorBuilder, OrderValidatorError},
    AngstromOrderValidator, OrderOrigin, OrderValidator, PoolOrder, TransactionValidationOutcome
};

/// A service that performs validation jobs.
#[derive(Clone)]
pub struct ValidationTask {
    #[allow(clippy::type_complexity)]
    validation_jobs: Arc<Mutex<ReceiverStream<Pin<Box<dyn Future<Output = ()> + Send>>>>>
}

impl ValidationTask {
    /// Creates a new clonable task pair
    pub fn new() -> (ValidationJobSender, Self) {
        let (tx, rx) = mpsc::channel(1);
        (ValidationJobSender { tx }, Self::with_receiver(rx))
    }

    /// Creates a new task with the given receiver.
    pub fn with_receiver(jobs: mpsc::Receiver<Pin<Box<dyn Future<Output = ()> + Send>>>) -> Self {
        ValidationTask { validation_jobs: Arc::new(Mutex::new(ReceiverStream::new(jobs))) }
    }

    /// Executes all new validation jobs that come in.
    ///
    /// This will run as long as the channel is alive and is expected to be
    /// spawned as a task.
    pub async fn run(self) {
        loop {
            let task = self.validation_jobs.lock().await.next().await;
            match task {
                None => return,
                Some(task) => task.await
            }
        }
    }
}

impl std::fmt::Debug for ValidationTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ValidationTask")
            .field("validation_jobs", &"...")
            .finish()
    }
}

/// A sender new type for sending validation jobs to [ValidationTask].
#[derive(Debug)]
pub struct ValidationJobSender {
    tx: mpsc::Sender<Pin<Box<dyn Future<Output = ()> + Send>>>
}

impl ValidationJobSender {
    /// Sends the given job to the validation task.
    pub async fn send(
        &self,
        job: Pin<Box<dyn Future<Output = ()> + Send>>
    ) -> Result<(), OrderValidatorError> {
        self.tx
            .send(job)
            .await
            .map_err(|_| OrderValidatorError::ValidationServiceUnreachable)
    }
}

/// A [OrderValidator] implementation that validates ethereum transaction.
///
/// This validator is non-blocking, all validation work is done in a separate
/// task.
#[derive(Debug, Clone)]
pub struct TransactionValidationTaskExecutor<V> {
    /// The validator that will validate transactions on a separate task.
    pub validator:          V,
    /// The sender half to validation tasks that perform the actual validation.
    pub to_validation_task: Arc<sync::Mutex<ValidationJobSender>>
}

// === impl TransactionValidationTaskExecutor ===

impl TransactionValidationTaskExecutor<()> {
    /// Convenience method to create a [AngstromOrderValidatorBuilder]
    pub fn eth_builder(chain_spec: Arc<ChainSpec>) -> AngstromOrderValidatorBuilder {
        AngstromOrderValidatorBuilder::new(chain_spec)
    }
}

impl<Client, Tx> TransactionValidationTaskExecutor<AngstromOrderValidator<Client, Tx>> {
    /// Creates a new instance for the given [ChainSpec]
    ///
    /// This will spawn a single validation tasks that performs the actual
    /// validation.
    /// See [TransactionValidationTaskExecutor::eth_with_additional_tasks]
    pub fn eth<T>(client: Client, chain_spec: Arc<ChainSpec>, tasks: T) -> Self
    where
        T: TaskSpawner
    {
        Self::eth_with_additional_tasks(client, chain_spec, tasks, 0)
    }

    /// Creates a new instance for the given [ChainSpec]
    ///
    /// By default this will enable support for:
    ///   - shanghai
    ///   - eip1559
    ///   - eip2930
    ///
    /// This will always spawn a validation task that performs the actual
    /// validation. It will spawn `num_additional_tasks` additional tasks.
    pub fn eth_with_additional_tasks<T>(
        client: Client,
        chain_spec: Arc<ChainSpec>,
        tasks: T,
        num_additional_tasks: usize
    ) -> Self
    where
        T: TaskSpawner
    {
        AngstromOrderValidatorBuilder::new(chain_spec)
            .with_additional_tasks(num_additional_tasks)
            .build_with_tasks::<Client, Tx, T>(client, tasks)
    }
}

impl<V> TransactionValidationTaskExecutor<V> {
    /// Creates a new executor instance with the given validator for transaction
    /// validation.
    ///
    /// Initializes the executor with the provided validator and sets up
    /// communication for validation tasks.
    pub fn new(validator: V) -> Self {
        let (tx, _) = ValidationTask::new();
        Self { validator, to_validation_task: Arc::new(sync::Mutex::new(tx)) }
    }
}

#[async_trait::async_trait]
impl<V> OrderValidator for TransactionValidationTaskExecutor<V>
where
    V: OrderValidator + Clone + 'static
{
    type Order = <V as OrderValidator>::Order;

    async fn validate_transaction(
        &self,
        origin: OrderOrigin,
        transaction: Self::Order
    ) -> TransactionValidationOutcome<Self::Order> {
        let hash = *transaction.hash();
        let (tx, rx) = oneshot::channel();
        {
            let res = {
                let to_validation_task = self.to_validation_task.clone();
                let to_validation_task = to_validation_task.lock().await;
                let validator = self.validator.clone();
                to_validation_task
                    .send(Box::pin(async move {
                        let res = validator.validate_transaction(origin, transaction).await;
                        let _ = tx.send(res);
                    }))
                    .await
            };
            if res.is_err() {
                return TransactionValidationOutcome::Error(
                    hash,
                    Box::new(OrderValidatorError::ValidationServiceUnreachable)
                )
            }
        }

        match rx.await {
            Ok(res) => res,
            Err(_) => TransactionValidationOutcome::Error(
                hash,
                Box::new(OrderValidatorError::ValidationServiceUnreachable)
            )
        }
    }

    fn on_new_head_block(&self, new_tip_block: &SealedBlock) {
        self.validator.on_new_head_block(new_tip_block)
    }
}
