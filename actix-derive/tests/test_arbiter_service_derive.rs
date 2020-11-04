use actix::{ArbiterService, Handler, System};
use actix_derive::Message;

#[derive(Message)]
#[rtype(result = "()")]
struct Sum;

#[derive(
    actix_derive::Actor, actix_derive::Supervised, actix_derive::ArbiterService, Default,
)]
#[actor(context = "::actix::SyncContext")]
struct Adder;

impl Handler<Sum> for Adder {
    type Result = <Sum as actix::Message>::Result;
    fn handle(&mut self, _: Sum, _: &mut Self::Context) -> () {
        ()
    }
}

#[test]
fn test_message() {
    let mut sys = System::new("actix-test-runtime");
    let addr = Adder::from_registry();
    let _ = sys.block_on(addr.send(Sum)).unwrap();
}
