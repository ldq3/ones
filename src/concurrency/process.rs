use alloc::vec::Vec;

use crate::{
    memory::page, runtime::address_space::AddressSpace, Allocator
};

/**
# 已有
new_pid()
*/
pub trait Lib {
    /**
    The id of kernel process is 0.
    */
    fn new_kernel();
    /**
    Add init process.
    */
    fn init() -> usize;
    // /**
    // fn fork(process: &mut Process) -> usize {
    //     let id = Self::new_pid();

    //     let thread = process.0.thread.iter().map(|x| x.clone()).collect();

    //     let inner = Process {
    //         id,
    //         thread,
    //         parent: Some(process.0.id),
    //         address_space: process.0.address_space.clone(),
    //         children: Vec::new(),
    //     };
    // }
    // */
    // fn fork(process: &mut Process) -> usize;
}

pub struct Process {
    pub id: usize, // 如果没有该字段不方便实现 Drop

    pub address_space: AddressSpace,
    pub page_table: page::Table,
    pub thread: Vec<usize>, // thread id
    
    pub parent: Option<usize>,
    pub children: Vec<usize>,
}

impl Process {
    /**
    use ones::{
            memory::Address,
            concurrency::process::Process as _,
            runtime::address_space::AddressSpace as _,
            intervene::Lib
        };
        use crate::{ 
            cpu::satp,
            intervene,
        };

        let (ks_bound, ks_bottom) = self.0.alloc_kernel_stack();
        let kernel = &mut self.0.process[0];

        let mut process = Process::new(elf);

        kernel.0.address_space.0.page_table.map_area((ks_bound, ks_bottom), Flag::R | Flag::W);
        let ksp = Address::address(ks_bottom + 1) - 1;
        process.0.thread[0].0.context.sp = ksp;

        use crate::intervene::data::Data;
        use riscv::register::sstatus::{ self, SPP };

        let (page_number, _) = AddressSpace::intervene_data(0);
        let (frame_number, _) = process.0.address_space.0.page_table.get(page_number);
        let data = Data::get_mut(frame_number);

        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        data.status = sstatus.bits();
        
        data.kernel_info.addr_trans = satp(kernel.0.address_space.0.page_table.root());
        data.kernel_info.dist = intervene::Handler::service_user as usize;
        data.kernel_info.sp = ksp;

        let pid = process.id();
        self.0.process.insert(pid, process);
        self.0.ready.push_back((pid, 0));
    */
    pub fn new(parent: Option<usize>, address_space: AddressSpace, page_table: page::Table) -> usize { 
        let id = access(|manager| {
            let id = manager.allocator.alloc().unwrap();
            
            let process = Process {
                id,
                address_space,
                thread: Vec::new(),
                parent,
                children: Vec::new(),
                page_table
            };
        
            manager.process[id] = Some(process);

            id
        }); 

        id
    }
}

// impl Drop for Process {
//     #[inline]
//     fn drop(&mut self) {


//         access(|manager| {
//             manager.allocator.dealloc(self.id);
//         })
//     }
// }

pub struct Manager {
    pub allocator: Allocator,
    pub process: Vec<Option<Process>>,
}

use spin::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref MANAGER : Mutex<Manager> = {
        let mut process = Vec::new();
        for _ in 0..16 {
            process.push(None);
        }

        Mutex::new(
            Manager {
                allocator: Allocator::new(0, 15).unwrap(),
                process,
            }
        )
    };
}
/**
Access process manager.
*/
#[inline]
pub fn access<F, V>(f: F) -> V
where
    F: FnOnce(&mut Manager) -> V,
{
    let mut mutex = MANAGER.lock();
    f(&mut mutex)
}
