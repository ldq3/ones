use alloc::{ 
    collections::vec_deque::VecDeque, vec::Vec
};

use ones::runtime::UserAddressSpace;
use crate::{ exception, virtualization::memory::page };

struct Process{
    id: usize,

    exception_stack: exception::Stack,
    address_space: UserAddressSpace,
    page_table: page::LocalTable,
    
    parent: Option<usize>,
    children: Vec<usize>,
}

impl Drop for Process {
    fn drop(&mut self) {
        MANAGER.lock().dealloc(self.id);
    }
}

impl Process {
    fn new(elf: &[u8]) -> Self {
        let id = MANAGER.lock().alloc();

        let exception_stack = exception::Stack::new(id);

        // map program headers of elf, with U flag
        let elf = xmas_elf::ElfFile::new(elf).unwrap();
        let magic = elf.header.pt1.magic;
        assert_eq!(magic, [0x7f, 0x45, 0x4c, 0x46], "invalid elf!");
        let address_space = UserAddressSpace::new(&elf);

        let mut page_table = page::LocalTable::new();
        for (segement, _) in address_space.clone() {
            page_table.insert_area(segement.range, segement.flag);
        }

        Self {
            id,

            exception_stack,
            address_space,
            page_table,

            parent: None,
            children: Vec::new(),
        }
    }
}

struct Manager {
    current: usize,
    recycled: Vec<usize>,
}

impl Manager {
    fn new() -> Self {
        Self {
            current: 0,
            recycled: Vec::new(),
        }
    }

    fn alloc(&mut self) -> usize {
        if let Some(id) = self.recycled.pop() {
                id
        } else {
            self.current += 1;
 
            self.current - 1
        }
    }

    fn dealloc(&mut self, pid: usize) {
        assert!(pid < self.current);
        assert!(
            self.recycled.iter().find(|ppid| **ppid == pid).is_none(),
            "pid {} has been deallocated!", pid
        );
        self.recycled.push(pid);
    }
}

use spin::Mutex;
use lazy_static::*;
lazy_static! {
    static ref MANAGER : Mutex<Manager> = Mutex::new(Manager::new());
}

struct Scheduler {
    process: Vec<Process>,
    ready: VecDeque<usize>,
}
