use async_trait::async_trait;

/// The Aggregate trait is responsible for validating commands, mapping commands to events, and applying
/// events onto the state.
///
/// An Aggregate should be able to derive its own state from nothing but its initial configuration, and its
/// event stream. Applying the same events, in the same order, to the same aggregate, should always yield an
/// identical aggregate state.
///
/// The `handle_command` method is _asynchronous_ and accepts a reference to the aggregate's collaborating
/// services. Services are external dependencies (e.g. payment gateways, email providers) that the aggregate
/// can use during command handling. For aggregates that do not require any services, the `Services` associated
/// type defaults to `()`.
///
/// The `apply_event` method remains _synchronous_ and must have no side effects.
#[async_trait]
pub trait Aggregate {
    /// The `NAME` const is responsible for naming an aggregate type.
    /// Each aggregate type should have a name that is unique among all the aggregate types in your application.
    ///
    /// Aggregates are linked to their instances & events using their `NAME` and their `aggregate_id`.
    /// Be very careful when changing `NAME`, as doing so will break the link between all the aggregates
    /// of their type, and their events!
    const NAME: &'static str;

    /// Internal aggregate state. This will be wrapped in [`crate::state::AggregateState`] and could
    /// be used to validate commands.
    type State: Default;

    /// A command is an action that the caller can execute over an aggregate in order to let it emit
    /// an event.
    type Command;

    /// An event represents a fact that took place in the domain. They are the source of truth;
    /// your current state is derived from the events.
    type Event;

    /// This associated type is used to get domain errors while handling a command.
    type Error: std::error::Error;

    /// External services (collaborators) that the aggregate can use during command handling.
    /// Set to `()` for aggregates that do not require any services.
    type Services: Send + Sync;

    /// Handles, validate a command and emits events. This method can use the provided `services`
    /// to interact with external systems (e.g. payment gateways, email providers).
    ///
    /// # Errors
    ///
    /// Will return `Err` if the user of this library set up command validations. Every error here
    /// could be just a "domain error". No technical errors.
    async fn handle_command(
        state: &Self::State,
        command: Self::Command,
        services: &Self::Services,
    ) -> Result<Vec<Self::Event>, Self::Error>;

    /// Updates the aggregate state using the new event. This assumes that the event can be correctly applied
    /// to the state.
    ///
    /// If this is not the case, this function is allowed to panic.
    fn apply_event(state: Self::State, payload: Self::Event) -> Self::State;
}
