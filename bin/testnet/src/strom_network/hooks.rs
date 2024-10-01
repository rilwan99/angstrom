use super::events::HookEvents;

pub struct StateMachineHook {
    /// the `expected` event starts being searched for
    pub trigger:    HookEvents,
    /// event expected to happen
    pub expected:   HookEvents,
    /// if `expected` doesn't happen by this event
    /// then failure
    pub kill_event: HookEvents
}
