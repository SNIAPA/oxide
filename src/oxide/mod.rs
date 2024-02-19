

use crate::*;

use self::{hooks::Hooks};



pub mod menu;
pub mod hooks;


mea!(interfaces);

#[derive(Debug, Clone, Copy)]
pub struct Oxide {
    pub interfaces: Interfaces,
    pub hooks: Hooks,
}


impl Oxide {
    pub unsafe fn init() -> Result<Oxide, Box<dyn Error>> {
        let interfaces = Interfaces::init()?;
        let hooks = Hooks::init(&interfaces)?;
        let oxide = Oxide {
            interfaces,
            hooks,
        };


        Ok(oxide)
    }
    pub unsafe fn unload(self) {
        self.interfaces.restore();
        self.hooks.restore();
    }
}