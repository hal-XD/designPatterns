/*
 Context
    StateA
    StateB

 Client
     Context
        swithcState
            StateA <-> StateB
*/

use std::collections::HashMap;

enum States {
    StateA,
    StateB,
}

trait State {
    fn switchState(&self, context: &mut Context);
}

struct ConcreateStateA {}
impl State for ConcreateStateA {
    fn switchState(&self, context: &mut Context) {}
}

struct ConcreateStateB {}
impl State for ConcreateStateB {
    fn switchState(&self, context: &mut Context) {}
}

struct Context {
    state : Box<dyn State>
}

impl Context {
    fn switchState(&mut self) {
        self.state.switchState(self)
    }
}

struct Client {
    context : Box<Context>
}

fn main() {
}