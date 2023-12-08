/// The Peer Challenge occurs when two peers with the same id are connected.
/// This can occur when the same person runs two different clients or someone
/// tries to connect to other nodes using another peers status message before
/// the time-stamp is out of its functioning range. when this occurs. A given
/// node will send a challenge message to both peers in-which it needs to
/// sign the message with there key and return it. If both challenge messages
/// are valid, Then the first connection is kept and the new one is dropped.
#[derive(Debug)]
pub struct PeerChallenge {}
