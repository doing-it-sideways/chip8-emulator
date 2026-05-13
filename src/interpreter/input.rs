use super::{
    ProgramStatus,
};

pub trait InputHandler {
    fn handle(&mut self, cur_input: &mut u16) -> ProgramStatus;
}
