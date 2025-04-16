/*!
将指定目录打包为一个文件系统镜像

# 用法
s
o
*/

extern crate ones;
extern crate alloc;

use clap::{ App, Arg };
use std::{ 
    fs::{ read_dir, File, OpenOptions },
    io::{ Read, Seek, SeekFrom, Write },
};
use spin::Mutex;
use alloc::sync::Arc;
use ones::file_system::{ BlockDevice, EasyFileSystem};

fn main() {
    let (source_path, target_path) = match_args();

    let image: Arc<Mutex<dyn BlockDevice>> = Arc::new(Mutex::new(BlockFile(Mutex::new({
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(format!("{}{}", target_path, "fs.img")).unwrap();

        f.set_len((config::SYS_SIZE * config::BLOCK_SIZE) as u64).unwrap();

        f
    }))));

    // 32MiB, at most 4095 files
    let efs = EasyFileSystem::new(image, config::SYS_SIZE as u32, 1);
    let root_inode = Arc::new(EasyFileSystem::root(&efs));

    let files: Vec<_> = read_dir(&source_path)
        .unwrap()
        .into_iter()
        .map(|dir_entry| {
            let name = dir_entry.unwrap().file_name().into_string().unwrap();
            
            name
        })
        .collect();

    for file in files {
        let mut host_file = File::open(format!("{}{}", &source_path, file)).unwrap();
        let mut all_data: Vec<u8> = Vec::new();
        host_file.read_to_end(&mut all_data).unwrap();
        // create a file in easy-fs
        let inode = root_inode.create(file.as_str()).unwrap();
        // write data to easy-fs
        inode.write_at(0, all_data.as_slice());
    }

    // list apps
    for app in root_inode.ls() {
        println!("{}", app);
    }
}

fn match_args() -> (String, String) {
    let matches = App::new("FileSystem packer")
        .arg(
            Arg::with_name("target")
                .short("t")
                .long("target")
                .takes_value(true)
                .help("Target dir(with backslash)."),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Output dir(with backslash)."),
        ).get_matches();
    
    let target_path = matches.value_of("target").expect("Error: Traget path is required.");
    let output_path = matches.value_of("output").expect("Error: Output path is required.");

    (target_path.to_string(), output_path.to_string())
}

struct BlockFile(Mutex<File>);

impl BlockDevice for BlockFile {
    fn read(&mut self, block_id: usize, buf: &mut [u8]) {
        let mut file = self.0.lock();
        file.seek(SeekFrom::Start((block_id * config::BLOCK_SIZE) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.read(buf).unwrap(), config::BLOCK_SIZE, "Not a complete block!");
    }

    fn write(&mut self, block_id: usize, buf: &[u8]) {
        let mut file = self.0.lock();
        file.seek(SeekFrom::Start((block_id * config::BLOCK_SIZE) as u64))
            .expect("Error when seeking!");
        assert_eq!(file.write(buf).unwrap(), config::BLOCK_SIZE, "Not a complete block!");
    }
}

mod config {
    /// 单位：字节（byte）
    pub const BLOCK_SIZE: usize = 512;
    /// 单位：块（block）
    pub const SYS_SIZE: usize = 32 * 2048;
}
