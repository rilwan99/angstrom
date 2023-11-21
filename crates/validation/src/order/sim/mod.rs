/// The SimValidation module is responsable for dealing with orders that
/// need to be partly simulated to prove valid.
/// These are composable orders where pre and post hooks add the uncertainty.
/// 1) This will Sim the pre-hook, do state validation with the updated pre-hook
/// data (Approvals transfers etc)
/// 2) Then normal State sim will occur
/// 3) post-hook will be simmed with updated data to ensure success
pub struct SimValidation {

}

