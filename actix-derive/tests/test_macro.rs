use actix::{Actor, Handler, System};
use actix_derive::{Message, MessageResponse};

#[derive(MessageResponse)]
struct Added(usize);

#[derive(Message)]
#[rtype(result = "Added")]
struct Sum(usize, usize);

#[derive(actix_derive::Actor, Default)]
struct Adder;

impl Handler<Sum> for Adder {
    type Result = <Sum as actix::Message>::Result;
    fn handle(&mut self, msg: Sum, _: &mut Self::Context) -> Added {
        Added(msg.0 + msg.1)
    }
}

#[test]
fn test_message() {
    System::new("actix-test-runtime").block_on(async {
        let addr = Adder::start_default();
        let res = addr.send(Sum(3, 5)).await.unwrap();
        assert_eq!(res.0, 8);
    });
}
