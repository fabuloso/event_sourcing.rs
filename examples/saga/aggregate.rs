use serde::{Deserialize, Serialize};

use async_trait::async_trait;
use esrs::Aggregate;

use crate::common::CommonError;

#[derive(Clone)]
pub struct SagaAggregate;

#[async_trait]
impl Aggregate for SagaAggregate {
    const NAME: &'static str = "saga";
    type State = ();
    type Command = SagaCommand;
    type Event = SagaEvent;
    type Error = CommonError;
    type Services = ();

    async fn handle_command(_state: &Self::State, command: Self::Command, _services: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            SagaCommand::RequestMutation => Ok(vec![SagaEvent::MutationRequested]),
            SagaCommand::RegisterMutation => Ok(vec![SagaEvent::MutationRegistered]),
        }
    }

    fn apply_event(_state: Self::State, _payload: Self::Event) -> Self::State {}
}

pub enum SagaCommand {
    RequestMutation,
    RegisterMutation,
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum SagaEvent {
    MutationRequested,
    MutationRegistered,
}

#[cfg(feature = "upcasting")]
impl esrs::event::Upcaster for SagaEvent {}
