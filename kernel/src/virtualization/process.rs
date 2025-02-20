use ones::virtualization::process::ModelProcess;

use crate::{ exception, virtualization::memory::page };

pub type _Process = ModelProcess<page::Table, exception::_Stack>;
