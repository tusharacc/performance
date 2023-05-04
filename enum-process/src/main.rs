use windows::Win32::System::ProcessStatus::*;
use windows::Win32::System::Threading::*;
use windows::core::*;
use windows::Win32::Foundation::*;

fn main() {
    unsafe{
        let mut cb_needed: u32 = 0;
        let mut a_processes: Vec<u32> = Vec::with_capacity(1024);
        let result: windows::Win32::Foundation::BOOL = EnumProcesses(a_processes.as_mut_ptr(), 1024*4, &mut cb_needed);
        let mut _c_processes: u32 = 0;
        
        _c_processes = cb_needed / 4;
        a_processes.set_len(_c_processes as usize);
        println!("{:?}",_c_processes);
        let mut count: u32 = 0;
        println!("{:?}",a_processes.len());
        for process_id in a_processes.iter(){
            println!("{:?}",process_id);

            let open_process_result = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ
                , false, 
                *process_id);

            if open_process_result.is_err(){
                println!("OpenProcess failed");
                continue;
            }

            let mut lphmodule: Vec<HMODULE> = Vec::with_capacity(1024);
            let mut cb: u32 = 1024;
            let mut lpcbneeded: u32 = 0;
            let process_modules_return = EnumProcessModules(open_process_result.clone().unwrap(), lphmodule.as_mut_ptr() , cb, &mut lpcbneeded);

            if process_modules_return != false {
                lphmodule.set_len(lpcbneeded as usize / 4);
                let mut module_name: [u16; 1024] = [0; 1024];
                let mut module_name_size: u32 = 1024;
                let get_module_base_name_return = GetModuleBaseNameW(open_process_result.unwrap(),
                                                 lphmodule[0], &mut module_name);
                if get_module_base_name_return == 0 {
                    println!("GetModuleBaseNameW failed");
                    continue;
                }
                let module_name_string = String::from_utf16_lossy(&module_name);
                println!("{:?}",module_name_string);
            }

        }
    }
}
