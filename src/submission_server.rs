use futures::Steam;

/// Basic http server that we use to accept new eip712 transactions and will just stream them to
/// the guard
pub struct SubmissionServer {}

// placeholder
type Tx712 = ();

impl Stream for SubmissionServer {
    type Item = Tx712;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        todo!()
    }
}
