use crate::errors::Errcode;

const EPERM: u16 = 1;

use syscallz::{Context, Action, Syscall};

pub fn setsyscalls() -> Result<(), Errcode> {

    log::debug!("Refusing / Filtering unwanted syscalls");

    let syscalls_to_allow = [
        Syscall::execve, Syscall::brk, Syscall::arch_prctl, Syscall::mmap, Syscall::access,
        Syscall::openat, Syscall::newfstatat, Syscall::close, Syscall::read, Syscall::mprotect,
        Syscall::pread64, Syscall::set_tid_address, Syscall::set_robust_list, Syscall::rseq,
        Syscall::prlimit64, Syscall::munmap, Syscall::poll, Syscall::rt_sigaction, Syscall::sigaltstack,
        Syscall::getrandom, Syscall::sched_getaffinity, Syscall::statx, Syscall::ioctl, Syscall::write,
        Syscall::uname, Syscall::readlink, Syscall::socketpair, Syscall::clone, Syscall::sethostname,
        Syscall::mount, Syscall::mkdir, Syscall::getdents64, Syscall::pivot_root, Syscall::chdir,
        Syscall::umount2, Syscall::rmdir, Syscall::unshare, Syscall::sendto, Syscall::recvfrom,
        Syscall::lseek, Syscall::rt_sigprocmask, Syscall::clone3, Syscall::wait4, Syscall::prctl,
        Syscall::getuid, Syscall::socket, Syscall::setsockopt, Syscall::bind, Syscall::getsockname,
        Syscall::recvmsg, Syscall::sendmsg, Syscall::exit_group, Syscall::setns, Syscall::setgroups,
        Syscall::setresgid, Syscall::setresuid, Syscall::capget, Syscall::seccomp, Syscall::futex,
        Syscall::sysinfo, Syscall::fcntl, Syscall::dup, Syscall::geteuid, Syscall::getegid,
        Syscall::getgid, Syscall::connect, Syscall::getcwd, Syscall::epoll_create1, Syscall::gettid,
        Syscall::sendmmsg, Syscall::listen, Syscall::accept4, Syscall::shutdown, Syscall::madvise,
        Syscall::exit, Syscall::rt_sigreturn
    ];

    if let Ok(mut ctx) = Context::init_with_action(Action::Errno(EPERM)){
        for &syscall in &syscalls_to_allow {
            allow_syscall(&mut ctx, syscall)?;
        }    

        if ctx.load().is_err() {
            return Err(Errcode::SyscallsError(0));
        }

        Ok(())
    } else {
        Err(Errcode::SyscallsError(1))
    }
}

fn allow_syscall(ctx: &mut Context, sc: Syscall) -> Result<(), Errcode> {
    match ctx.set_action_for_syscall(Action::Allow, sc) {
        Ok(_) => Ok(()),
        Err(_) => Err(Errcode::SyscallsError(2)),
    }
}
