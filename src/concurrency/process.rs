/*!
The id of kernel process is 0.
*/

use alloc::vec::Vec;

use crate::{
    memory::page::{ self, Table },
    runtime::address_space::AddressSpace,
    Allocator
};

/**
# 已有
new_pid()
*/
pub trait Lib: Hal {
    fn new(parent: Option<usize>, address_space: AddressSpace) -> usize;
    /**
    Add init process.
    */
    fn from_elf(parent: Option<usize>, elf: &[u8]) -> usize {
        let (address_space, data_offset) = AddressSpace::from_elf(&elf);
        let pid = Self::new(parent, address_space);

        access(|manager| {
            let process = manager.process[pid].as_mut().unwrap();
            for i in 0..data_offset.len() { 
                let segement= process.address_space.segement[i];

                Self::copy_data(&mut process.page_table, segement.range, &elf[data_offset[i].0..data_offset[i].1]);
            }
        });
 
        pid
    }

    fn new_kernel(address_space: AddressSpace) -> usize;
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

pub trait Hal {
    fn copy_data(table: &mut Table, range: (usize, usize), data: &[u8]);
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
