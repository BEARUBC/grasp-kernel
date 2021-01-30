use actix::{
    prelude::*,
    dev::{MessageResponse, ResponseChannel},
};

#[derive(Message)]
#[rtype(result = "Result<bool, std::io::Error>")]
pub(crate) enum PingResponse {
    RA,
    RB,
}

impl<A, M> MessageResponse<A, M> for PingResponse where
    A: Actor,
    M: Message<Result = PingResponse>,
{
    fn handle<R: ResponseChannel<M>>(self, _: &mut A::Context, tx: Option<R>) {
        if let Some(tx) = tx {
            tx.send(self);
        }
    }
}
