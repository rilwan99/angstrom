/// the representation of these represents time in seconds
/// for the current cutoff period
#[derive(Debug, Default)]
pub enum ConsensusStage {
    // TODO: not 100% sure on this
    #[default]
    DataPropagation     = 0,
    BundleSigningCutoff = 8,
    /// TODO: not sure if we need this or not. we will leave it
    /// in for now tho
    Bundle23PropCutoff  = 9,
    LeaderProposeCutoff = 10
}

impl ConsensusStage {
    pub fn get_current_period(time: Time) -> Self {
        todo!()
    }
}
