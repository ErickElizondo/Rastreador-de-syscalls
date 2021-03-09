use libc::{c_void, user_regs_struct, PT_NULL};
use nix::sys::ptrace;
use nix::sys::ptrace::*;
use nix::sys::wait::waitpid;
use std::collections::HashMap;
use std::mem;
use std::os::unix::process::CommandExt;
use std::process::Command;
use std::ptr;
use std::io::{stdin, stdout, Read, Write};
use std::time::Instant;

mod system_call_names;

fn pause() {//Done by user u/K900_ in reddit https://www.reddit.com/r/rust/comments/8tfyof/noob_question_pause/
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn traceme() -> std::io::Result<(())> {
    match ptrace::traceme() {
        Ok(()) => Ok(()),
        Err(::nix::Error::Sys(errno)) => Err(std::io::Error::from_raw_os_error(errno as i32)),
        Err(e) => Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
    }
}

pub fn get_regs(pid: nix::unistd::Pid, arg: &str, exit: bool) -> Result<user_regs_struct, nix::Error> {
    unsafe {//Changed to fulfill the requierements of -v and -V
        let mut regs: user_regs_struct = mem::uninitialized();
        let start = Instant::now();

        #[allow(deprecated)]
        let res = ptrace::ptrace(
            Request::PTRACE_GETREGS,
            pid,
            PT_NULL as *mut c_void,
            &mut regs as *mut _ as *mut c_void,
        );
        let elapsed = start.elapsed();

        res.map(|_| regs);
        let mut syscallName = system_call_names::SYSTEM_CALL_NAMES[(regs.orig_rax) as usize];
        if (arg == "-V" && exit){
            println!("{}", &syscallName);
            pause();
        }else if (arg == "-v" && exit){ //Data obtained from sys/user.h https://code.woboq.org/qt5/include/sys/user.h.html
            println!("========================================");
            println!("syscall name: {}", &syscallName);
            println!("Register content (some of them):");
            println!("r15: {}", regs.r15);
            println!("r14: {}", regs.r14);
            println!("regs.orig_rax: {}", regs.orig_rax);
            println!("rcx: {}", regs.rcx);
            println!("rdi: {}", regs.rdi);
            println!("Time_elapsed: {} ms", elapsed.as_secs_f64());
        }
        res.map(|_| regs)
    }
}

fn strace(option: &str, argv: &mut [std::string::String]){//Moved main function to an auxiliar one so I can iterate through the arguments
    let mut cmd = Command::new(&argv[1]);
    for arg in argv {
        if (arg == "-v" || arg == "-V"){
            continue;            
        }else{//I wanna add as arguments any value except -v or -V
            cmd.arg(arg);
        }
    }
    let mut map = HashMap::new();

    //allow the child to be traced
    let output = cmd.before_exec(traceme);

    let mut child = cmd.spawn().expect("child process failed");

    let pid = nix::unistd::Pid::from_raw(child.id() as libc::pid_t);

    //allow parent to be stopped everytime there is a SIGTRAP sent because a syscall happened.
    ptrace::setoptions(
        pid,
        Options::PTRACE_O_TRACESYSGOOD | Options::PTRACE_O_TRACEEXEC,
    )
    .unwrap();

    waitpid(pid, None);

    /// Whether we are exiting (rather than entering) a syscall.
    /// ptrace is stopped both times when exiting and entering a syscall, we only
    /// need to stop once.  
    let mut exit = true;

    loop {
        //get the registers from the address where ptrace is stopped.
        let regs = match get_regs(pid, option, exit) {
            Ok(x) => x,
            Err(err) => {
                eprintln!("End of ptrace {:?}", err);
                break;
            }
        };
        
        if exit {
            /// syscall number is stored inside orig_rax register. Transalte from number
            /// to syscall name using an array that stores all syscalls.  
            let mut syscallName = system_call_names::SYSTEM_CALL_NAMES[(regs.orig_rax) as usize];

            match map.get(&syscallName) {
                Some(&number) => map.insert(syscallName, number + 1),
                _ => map.insert(syscallName, 1),
            };
        }

        unsafe {
            ptrace(
                Request::PTRACE_SYSCALL,
                pid,
                ptr::null_mut(),
                ptr::null_mut(),
            );
        }

        waitpid(pid, None);
        exit = !exit;
    }
    println!("=======displaying all syscalls=======");
    for (syscall, &number) in map.iter() {
        println!("{}: {}", syscall, number);
    }
}

fn main() {
    let argv: Vec<std::string::String> = std::env::args().collect();
    println!("{:?}", argv);
    let mut newvec = argv.to_vec();//Since I can't pass a vector as a parameter on a loop, I had to copy that vector in a weird way
    for arg in argv {
        if (arg == "-v" || arg == "-V"){
            strace(&arg, &mut newvec);
            println!("==========================");
        }
    }    
}