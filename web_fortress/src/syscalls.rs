use crate::errors::Errcode;
use libc::TIOCSTI;
use nix::sys::stat::Mode;
use nix::sched::CloneFlags;
use syscallz::{Action, Cmp, Comparator, Context, Syscall};

// Define the error code for permission denied
const EPERM: u16 = 1;
const AT_FDCWD: i32 = -100; // Value of AT_FDCWD, which indicates the current working directory
const BLOCKED_UID: u64 = 10000;
const BLOCKED_GID: u64 = 10000;

// Function to set syscall filters
pub fn setsyscalls() -> Result<(), Errcode> {
    log::debug!("Refusing / Filtering unwanted syscalls");

    // Define specific modes and flags
    let s_isuid: u64 = Mode::S_ISUID.bits().into();
    let s_isgid: u64 = Mode::S_ISGID.bits().into();
    let clone_new_user: u64 = CloneFlags::CLONE_NEWUSER.bits() as u64;

    // Define syscalls to refuse based on specific conditions
    let syscalls_refuse_ifcomp = [
        (Syscall::chmod, 1, s_isuid),
        (Syscall::chmod, 1, s_isgid),
        (Syscall::fchmod, 1, s_isuid),
        (Syscall::fchmod, 1, s_isgid),
        (Syscall::fchmodat, 2, s_isuid),
        (Syscall::fchmodat, 2, s_isgid),
        (Syscall::unshare, 0, clone_new_user),
        (Syscall::clone, 0, clone_new_user),
        (Syscall::ioctl, 1, TIOCSTI),
        (Syscall::execve, 1, BLOCKED_UID),
        (Syscall::execve, 1, BLOCKED_GID),
    ];

    // Define syscalls to refuse unconditionally
    let syscalls_refused = [
        Syscall::keyctl,
        Syscall::add_key,
        Syscall::request_key,
        Syscall::mbind,
        Syscall::migrate_pages,
        Syscall::move_pages,
        Syscall::set_mempolicy,
        Syscall::userfaultfd,
        Syscall::perf_event_open,
    ];

    // Initialize the syscall context with the default action of allowing syscalls
    if let Ok(mut ctx) = Context::init_with_action(Action::Allow) {
        // Refuse specific syscalls based on conditions
        for (sc, ind, biteq) in syscalls_refuse_ifcomp.iter() {
            refuse_if_comp(&mut ctx, *ind, sc, *biteq)?;
        }

        // Refuse specific syscalls unconditionally
        for sc in syscalls_refused.iter() {
            refuse_syscall(&mut ctx, sc)?;
        }

        // Load the context to apply the rules
        if let Err(_) = ctx.load() {
            return Err(Errcode::SyscallsError(0));
        }

        Ok(())
    } else {
        Err(Errcode::SyscallsError(1))
    }
}

// Function to refuse syscalls based on a condition
fn refuse_if_comp(ctx: &mut Context, ind: u32, sc: &Syscall, biteq: u64) -> Result<(), Errcode> {
    match ctx.set_rule_for_syscall(
        Action::Errno(EPERM),
        *sc,
        &[Comparator::new(ind, Cmp::MaskedEq, biteq, Some(biteq))]
    ) {
        Ok(_) => Ok(()),
        Err(_) => Err(Errcode::SyscallsError(2)),
    }
}

// Function to refuse syscalls unconditionally
fn refuse_syscall(ctx: &mut Context, sc: &Syscall) -> Result<(), Errcode> {
    match ctx.set_action_for_syscall(Action::Errno(EPERM), *sc) {
        Ok(_) => Ok(()),
        Err(_) => Err(Errcode::SyscallsError(3)),
    }
}
