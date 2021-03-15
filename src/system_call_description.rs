pub static SYSTEM_CALL_NAMES: [&str; 332] = [
    "read() attempts to read up to count bytes from file descriptor fd
    into the buffer starting at buf.",
    "write() writes up to count bytes from the buffer starting at buf
    to the file referred to by the file descriptor fd.",
    "The open() system call opens the file specified by pathname.  If
    the specified file does not exist, it may optionally (if O_CREAT
    is specified in flags) be created by open().",
    "close() closes a file descriptor, so that it no longer refers to
    any file and may be reused.",
    "stat() retrieve information about the file pointed
    to by pathname.",
    "fstat() is identical to stat(), except that the file about which
    information is to be retrieved is specified by the file
    descriptor fd.",
    "lstat() is identical to stat(), except that if pathname is a
    symbolic link, then it returns information about the link itself,
    not the file that the link refers to.",
    "poll() performs a similar task to select(2): it waits for one of
    a set of file descriptors to become ready to perform I/O.",
    "lseek() repositions the file offset of the open file description
    associated with the file descriptor fd to the argument offset
    according to the directive whence.",
    "mmap() creates a new mapping in the virtual address space of the
    calling process.  The starting address for the new mapping is
    specified in addr.  The length argument specifies the length of
    the mapping (which must be greater than 0).",
    "mprotect() changes the access protections for the calling
    process's memory pages containing any part of the address range
    in the interval [addr, addr+len-1].  addr must be aligned to a
    page boundary.",
    "The munmap() system call deletes the mappings for the specified
    address range, and causes further references to addresses within
    the range to generate invalid memory references.",
    "brk() sets the end of the data segment to the value specified by
    addr, when that value is reasonable, the system has enough
    memory, and the process does not exceed its maximum data size",
    "The sigaction() system call is used to change the action taken by
    a process on receipt of a specific signal.",
    "sigprocmask() is used to fetch and/or change the signal mask of
    the calling thread.  The signal mask is the set of signals whose
    delivery is currently blocked for the caller.",
    "This sigreturn() call undoes everything that was done—changing
    the process's signal mask, switching signal stacks in order to invoke 
    the signal handler.",
    "The ioctl() system call manipulates the underlying device
    parameters of special files.  In particular, many operating
    characteristics of character special files (e.g., terminals) may
    be controlled with ioctl() requests.  The argument fd must be an
    open file descriptor.",
    "pread() reads up to count bytes from file descriptor fd at offset
    offset (from the start of the file) into the buffer starting at
    buf.  The file offset is not changed.",
    "pwrite() writes up to count bytes from the buffer starting at buf
    to the file descriptor fd at offset offset.  The file offset is
    not changed.",
    "The readv() system call reads iovcnt buffers from the file
    associated with the file descriptor fd into the buffers described
    by iov (scatter input)",
    "The writev() system call writes iovcnt buffers of data described
    by iov to the file associated with the file descriptor fd
    (gather output).",
    "access() checks whether the calling process can access the file
    pathname.  If pathname is a symbolic link, it is dereferenced.",
    "pipe() creates a pipe, a unidirectional data channel that can be
    used for interprocess communication.  The array pipefd is used to
    return two file descriptors referring to the ends of the pipe.
    pipefd[0] refers to the read end of the pipe.  pipefd[1] refers
    to the write end of the pipe.  Data written to the write end of
    the pipe is buffered by the kernel until it is read from the read
    end of the pipe.",
    "select() allows a program to monitor multiple file descriptors,
    waiting until one or more of the file descriptors become ready
    for some class of I/O operation (e.g., input possible).",
    "sched_yield() causes the calling thread to relinquish the CPU.
    The thread is moved to the end of the queue for its static
    priority and a new thread gets to run.",
    "mremap() expands (or shrinks) an existing memory mapping,
    potentially moving it at the same time (controlled by the flags
    argument and the available virtual address space).",
    "msync() flushes changes made to the in-core copy of a file that
    was mapped into memory using mmap(2) back to the filesystem.",
    "mincore() returns a vector that indicates whether pages of the
    calling process's virtual memory are resident in core (RAM), and
    so will not cause a disk access (page fault) if referenced.",
    "The madvise() system call is used to give advice or directions to
    the kernel about the address range beginning at address addr and
    with size length bytes In most cases, the goal of such advice is
    to improve system or application performance.",
    "shmget() returns the identifier of the System V shared memory
    segment associated with the value of the argument key.  It may be
    used either to obtain the identifier of a previously created
    shared memory segment (when shmflg is zero and key does not have
    the value IPC_PRIVATE), or to create a new set.",
    "shmat() attaches the System V shared memory segment identified by
    shmid to the address space of the calling process.",
    "shmctl() performs the control operation specified by cmd on the
    System V shared memory segment whose identifier is given in
    shmid.",
    "The dup() system call creates a copy of the file descriptor
    oldfd, using the lowest-numbered unused file descriptor for the
    new descriptor.",
    "The dup2() system call performs the same task as dup(), but
    instead of using the lowest-numbered unused file descriptor, it
    uses the file descriptor number specified in newfd.  If the file
    descriptor newfd was previously open, it is silently closed
    before being reused.",
    "pause() causes the calling process (or thread) to sleep until a
    signal is delivered that either terminates the process or causes
    the invocation of a signal-catching function.",
    "nanosleep() suspends the execution of the calling thread until
    either at least the time specified in *req has elapsed, or the
    delivery of a signal that triggers the invocation of a handler in
    the calling thread or that terminates the process.",
    "These system calls provide access to interval timers, that is,
    timers that initially expire at some point in the future, and
    (optionally) at regular intervals after that.  When a timer
    expires, a signal is generated for the calling process, and the
    timer is reset to the specified interval (if the interval is
    nonzero).",
    "alarm() arranges for a SIGALRM signal to be delivered to the
    calling process in seconds seconds.
    If seconds is zero, any pending alarm is canceled.
    In any event any previously set alarm() is canceled.",
    "These system calls provide access to interval timers, that is,
    timers that initially expire at some point in the future, and
    (optionally) at regular intervals after that.  When a timer
    expires, a signal is generated for the calling process, and the
    timer is reset to the specified interval (if the interval is
    nonzero).",
    "getpid() returns the process ID (PID) of the calling process.
    (This is often used by routines that generate unique temporary
    filenames.)",
    "sendfile() copies data between one file descriptor and another.
    Because this copying is done within the kernel, sendfile() is
    more efficient than the combination of read(2) and write(2),
    which would require transferring data to and from user space.",
    "socket() creates an endpoint for communication and returns a file
    descriptor that refers to that endpoint.  The file descriptor
    returned by a successful call will be the lowest-numbered file
    descriptor not currently open for the process.",
    "The connect() system call connects the socket referred to by the
    file descriptor sockfd to the address specified by addr.  The
    addrlen argument specifies the size of addr.  The format of the
    address in addr is determined by the address space of the socket
    sockfd; see socket(2) for further details.",
    "The accept() system call is used with connection-based socket
    types (SOCK_STREAM, SOCK_SEQPACKET).  It extracts the first
    connection request on the queue of pending connections for the
    listening socket, sockfd, creates a new connected socket, and
    returns a new file descriptor referring to that socket.  The
    newly created socket is not in the listening state.  The original
    socket sockfd is unaffected by this call.",
    "The system calls send(), sendto(), and sendmsg() are used to
    transmit a message to another socket.
    If sendto() is used on a connection-mode (SOCK_STREAM,
    SOCK_SEQPACKET) socket, the arguments dest_addr and addrlen are
    ignored (and the error EISCONN may be returned when they are not
    NULL and 0), and the error ENOTCONN is returned when the socket
    was not actually connected. ",
    "recvfrom() places the received message into the buffer buf.  The
    caller must specify the size of the buffer in len.",
    "The system calls send(), sendto(), and sendmsg() are used to
    transmit a message to another socket.",
    "The recvmsg() call uses a msghdr structure to minimize the number
    of directly supplied arguments.  This structure is defined as
    follows in <sys/socket.h>:",
    "The shutdown() call causes all or part of a full-duplex
    connection on the socket associated with sockfd to be shut down.
    If how is SHUT_RD, further receptions will be disallowed.  If how
    is SHUT_WR, further transmissions will be disallowed.  If how is
    SHUT_RDWR, further receptions and transmissions will be
    disallowed.",
    "When a socket is created with socket(2), it exists in a name
    space (address family) but has no address assigned to it.  bind()
    assigns the address specified by addr to the socket referred to
    by the file descriptor sockfd.  addrlen specifies the size, in
    bytes, of the address structure pointed to by addr.
    Traditionally, this operation is called “assigning a name to a
    socket”.",
    "listen() marks the socket referred to by sockfd as a passive
    socket, that is, as a socket that will be used to accept incoming
    connection requests using accept(2).",
    "getsockname() returns the current address to which the socket
    sockfd is bound, in the buffer pointed to by addr.  The addrlen
    argument should be initialized to indicate the amount of space
    (in bytes) pointed to by addr.  On return it contains the actual
    size of the socket address.",
    "getpeername() returns the address of the peer connected to the
    socket sockfd, in the buffer pointed to by addr.  The addrlen
    argument should be initialized to indicate the amount of space
    pointed to by addr.  On return it contains the actual size of the
    name returned (in bytes).  The name is truncated if the buffer
    provided is too small.",
    "The socketpair() call creates an unnamed pair of connected
    sockets in the specified domain, of the specified type, and using
    the optionally specified protocol.",
    "setsockopt() manipulate options for the socket
    referred to by the file descriptor sockfd.  Options may exist at
    multiple protocol levels; they are always present at the
    uppermost socket level.",
    "getsockopt() manipulate options for the socket
    referred to by the file descriptor sockfd.  Options may exist at
    multiple protocol levels; they are always present at the
    uppermost socket level.",
    "These system calls create a new (child) process, in a manner
    similar to fork(2).
    By contrast with fork(2), these system calls provide more precise
    control over what pieces of execution context are shared between
    the calling process and the child process.",
    "fork() creates a new process by duplicating the calling process.
    The new process is referred to as the child process.  The calling
    process is referred to as the parent process.",
    "vfork(), just like fork(2), creates a child process of the
    calling process.  For details and return value and errors, see
    fork(2).",
    "execve() executes the program referred to by pathname.  This
    causes the program that is currently being run by the calling
    process to be replaced with a new program, with newly initialized
    stack, heap, and (initialized and uninitialized) data segments.",
    "_exit() terminates the calling process immediately.  Any open
    file descriptors belonging to the process are closed.  Any
    children of the process are inherited by init(1) (or by the
    nearest subreaper process as defined through the use of the
    prctl(2) PR_SET_CHILD_SUBREAPER operation).  The process's parent
    is sent a SIGCHLD signal.",
    "wait4() system call is similar to waitpid(2),
    but additionally return resource usage information about the
    child in the structure pointed to by rusage.",
    "The kill() system call can be used to send any signal to any
    process group or process.",
    "uname() returns system information in the structure pointed to by
    buf.",
    "The semget() system call returns the System V semaphore set
    identifier associated with the argument key.  It may be used
    either to obtain the identifier of a previously created semaphore
    set (when semflg is zero and key does not have the value
    IPC_PRIVATE), or to create a new set.",
    "semop() performs operations on selected semaphores in the set
    indicated by semid.  Each of the nsops elements in the array
    pointed to by sops is a structure that specifies an operation to
    be performed on a single semaphore.",
    "semctl() performs the control operation specified by cmd on the
    System V semaphore set identified by semid, or on the semnum-th
    semaphore of that set.",
    "shmdt() detaches the shared memory segment located at the address
    specified by shmaddr from the address space of the calling
    process.  The to-be-detached segment must be currently attached
    with shmaddr equal to the value returned by the attaching shmat()
    call.",
    "The msgget() system call returns the System V message queue
    identifier associated with the value of the key argument.  It may
    be used either to obtain the identifier of a previously created
    message queue (when msgflg is zero and key does not have the
    value IPC_PRIVATE), or to create a new set.",
    "The msgsnd() system call appends a copy of the message pointed to
    by msgp to the message queue whose identifier is specified by
    msqid.",
    "The msgrcv() system call removes a message from the queue
    specified by msqid and places it in the buffer pointed to by
    msgp",
    "msgctl() performs the control operation specified by cmd on the
    System V message queue with identifier msqid.",
    "fcntl() performs one of the operations described below on the
    open file descriptor fd.  The operation is determined by cmd.",
    "flock - apply or remove an advisory lock on an open file",
    "fsync() transfers (flushes) all modified in-core data of (i.e.,
    modified buffer cache pages for) the file referred to by the file
    descriptor fd to the disk device (or other permanent storage
    device) so that all changed information can be retrieved even if
    the system crashes or is rebooted.",
    "fdatasync() is similar to fsync(), but does not flush modified
    metadata unless that metadata is needed in order to allow a
    subsequent data retrieval to be correctly handled.",
    "The truncate() and ftruncate() functions cause the regular file
    named by path or referenced by fd to be truncated to a size of
    precisely length bytes. With truncate(), the file must be writable.",
    "The truncate() and ftruncate() functions cause the regular file
    named by path or referenced by fd to be truncated to a size of
    precisely length bytes. With ftruncate(), the file must be open for writing.",
    "The system call getdents() reads several linux_dirent structures
    from the directory referred to by the open file descriptor fd
    into the buffer pointed to by dirp.  The argument count specifies
    the size of that buffer.",
    "The getcwd() function copies an absolute pathname of the current
    working directory to the array pointed to by buf, which is of
    length size.",
    "chdir() changes the current working directory of the calling
    process to the directory specified in path.",
    "fchdir() is identical to chdir(); the only difference is that the
    directory is given as an open file descriptor.",
    "rename() renames a file, moving it between directories if
    required.  Any other hard links to the file (as created using
    link(2)) are unaffected.  Open file descriptors for oldpath are
    also unaffected.",
    "mkdir() attempts to create a directory named pathname.",
    "rmdir() deletes a directory, which must be empty.",
    "The open() system call opens the file specified by pathname.  If
    the specified file does not exist, it may optionally (if O_CREAT
    is specified in flags) be created by open().",
    "link() creates a new link (also known as a hard link) to an
    existing file.",
    "unlink() deletes a name from the filesystem.  If that name was
    the last link to a file and no processes have the file open, the
    file is deleted and the space it was using is made available for
    reuse.",
    "symlink() creates a symbolic link named linkpath which contains
    the string target.",
    "readlink() places the contents of the symbolic link pathname in
    the buffer buf, which has size bufsiz.  readlink() does not
    append a null byte to buf.  It will (silently) truncate the
    contents (to a length of bufsiz characters), in case the buffer
    is too small to hold all of the contents.",
    "chmod() changes the mode of the file specified whose pathname
    is given in pathname, which is dereferenced if it is a symbolic
    link.",
    "fchmod() changes the mode of the file referred to by the open
    file descriptor fd.",
    "chown() changes the ownership of the file specified by
    pathname, which is dereferenced if it is a symbolic link.",
    "fchown() changes the ownership of the file referred to by the
    open file descriptor fd.",
    "lchown() is like chown(), but does not dereference symbolic
    links.",
    "umask() sets the calling process's file mode creation mask
    (umask) to mask & 0777 (i.e., only the file permission bits of
    mask are used), and returns the previous value of the mask.",
    "The function gettimeofday() can get and set
    the time as well as a timezone.",
    "The getrlimit() system call get and set resource
    limits.  Each resource has an associated soft and hard limit, as
    defined by the rlimit structure",
    "getrusage - get resource usage",
    "sysinfo() returns certain statistics on memory and swap usage, as
    well as the load average.",
    "times() stores the current process times in the struct tms that
    buf points to",
    "The ptrace() system call provides a means by which one process
    (the tracer) may observe and control the execution of another
    process (the tracee), and examine and change the tracee's
    memory and registers.  It is primarily used to implement
    breakpoint debugging and system call tracing.",
    "getuid() returns the real user ID of the calling process.",
    "syslog, klogctl - read and/or clear kernel message ring buffer",
    "getgid() returns the real group ID of the calling process.",
    "setuid() sets the effective user ID of the calling process.  If
    the calling process is privileged (more precisely: if the process
    has the CAP_SETUID capability in its user namespace), the real
    UID and saved set-user-ID are also set.",
    "setgid() sets the effective group ID of the calling process.  If
    the calling process is privileged (more precisely: has the
    CAP_SETGID capability in its user namespace), the real GID and
    saved set-group-ID are also set.",
    "geteuid() returns the effective user ID of the calling process.",
    "getegid() returns the effective group ID of the calling process.",
    "setpgid() sets the PGID of the process specified by pid to pgid.
    If pid is zero, then the process ID of the calling process is
    used.  If pgid is zero, then the PGID of the process specified by
    pid is made the same as its process ID. ",
    "getppid() returns the process ID of the parent of the calling
    process.",
    "The POSIX.1 version of getpgrp(), which takes no arguments,
    returns the PGID of the calling process",
    "setsid() creates a new session if the calling process is not a
    process group leader.",
    "setreuid() sets real and effective user IDs of the calling
    process.",
    "Completely analogously, setregid() sets real and effective group
    ID's of the calling process, and all of the above holds with
    group instead of user.",
    "getgroups() returns the supplementary group IDs of the calling
    process in list.  The argument size should be set to the maximum
    number of items that can be stored in the buffer pointed to by
    list.",
    "setgroups() sets the supplementary group IDs for the calling
    process.",
    "setresuid() sets the real user ID, the effective user ID, and the
    saved set-user-ID of the calling process.",
    "getresuid() returns the real UID, the effective UID, and the
    saved set-user-ID of the calling process, in the arguments ruid,
    euid, and suid, respectively.",
    "Completely analogously, setresgid() sets the real GID, effective
    GID, and saved set-group-ID of the calling process (and always
    modifies the filesystem GID to be the same as the effective GID),
    with the same restrictions for unprivileged processes.",
    "getresgid() performs the analogous
    task for the process's group IDs.",
    "getpgid() returns the PGID of the process specified by pid.  If
    pid is zero, the process ID of the calling process is used.",
    "setfsuid - set user identity used for filesystem checks",
    "setfsgid - set group identity used for filesystem checks",
    "getsid(0) returns the session ID of the calling process.
    getsid() returns the session ID of the process with process ID
    pid.  If pid is 0, getsid() returns the session ID of the calling
    process.",
    "capget get capabilities of thread(s)",
    "capset - set capabilities of thread(s)",
    "sigpending() returns the set of signals that are pending for
    delivery to the calling thread (i.e., the signals which have been
    raised while blocked).  The mask of pending signals is returned
    in set.",
    "sigwaitinfo() suspends execution of the calling thread until one
    of the signals in set is pending (If one of the signals in set is
    already pending for the calling thread, sigwaitinfo() will return
    immediately.)",
    "The rt_sigqueueinfo() system call sends the signal sig to the
    thread group with the ID tgid.  (The term thread group is
    synonymous with process, and tid corresponds to the traditional
    UNIX process ID.)  The signal will be delivered to an arbitrary
    member of the thread group (i.e., one of the threads that is not
    currently blocking the signal).",
    "sigsuspend() temporarily replaces the signal mask of the calling
    thread with the mask given by mask and then suspends the thread
    until delivery of a signal whose action is to invoke a signal
    handler or to terminate a process.",
    "sigaltstack() allows a thread to define a new alternate signal
    stack and/or retrieve the state of an existing alternate signal stack",
    "The utime() system call changes the access and modification times
    of the inode specified by filename to the actime and modtime
    fields of times respectively",
    "The system call mknod() creates a filesystem node (file, device
    special file, or named pipe) named pathname, with attributes
    specified by mode and dev.",
    "The system call uselib() serves to load a shared library to be
    used by the calling process.  It is given a pathname.  The
    address where to load is found in the library itself.  The
    library can have any recognized binary format.",
    "personality - set the process execution domain",
    "ustat() returns information about a mounted filesystem.  dev is a
    device number identifying a device containing a mounted
    filesystem.",
    "The statfs() system call returns information about a mounted
    filesystem.  path is the pathname of any file within the mounted
    filesystem.",
    "fstatfs() returns the same information about an open file
    referenced by descriptor fd.", 
    "The (obsolete) sysfs() system call returns information about the
    filesystem types currently present in the kernel.",
    "getpriority - get program scheduling priority",
    "setpriority - set program scheduling priority",
    "sched_setparam() sets the scheduling parameters associated with
    the scheduling policy for the thread whose thread ID is specified
    in pid.",
    "sched_getparam() retrieves the scheduling parameters for the
    thread identified by pid.  If pid is zero, then the parameters of
    the calling thread are retrieved.",
    "The sched_setscheduler() system call sets both the scheduling
    policy and parameters for the thread whose ID is specified in
    pid.  If pid equals zero, the scheduling policy and parameters of
    the calling thread will be set.",
    "sched_getscheduler() returns the current scheduling policy of the
    thread identified by pid.  If pid equals zero, the policy of the
    calling thread will be retrieved.",
    "sched_get_priority_max() returns the maximum priority value that
    can be used with the scheduling algorithm identified by policy.",
    "sched_get_priority_min() returns the minimum priority value that
    can be used with the scheduling algorithm identified by policy.",
    "sched_rr_get_interval() writes into the timespec structure
    pointed to by tp the round-robin time quantum for the process
    identified by pid.  The specified process should be running under
    the SCHED_RR scheduling policy.",
    "mlock() locks pages in the address range starting at addr and
    continuing for len bytes.  All pages that contain a part of the
    specified address range are guaranteed to be resident in RAM when
    the call returns successfully; the pages are guaranteed to stay
    in RAM until later unlocked.",
    "munlock() unlocks pages in the address range starting at addr and
    continuing for len bytes.  After this call, all pages that
    contain a part of the specified memory range can be moved to
    external swap space again by the kernel.",
    "mlockall() locks all pages mapped into the address space of the
    calling process.  This includes the pages of the code, data and
    stack segment, as well as shared libraries, user space kernel
    data, shared memory, and memory-mapped files.  All mapped pages
    are guaranteed to be resident in RAM when the call returns
    successfully; the pages are guaranteed to stay in RAM until later
    unlocked.",
    "munlockall() unlocks all pages mapped into the address space of
    the calling process.",
    "vhangup() simulates a hangup on the current terminal.  This call
    arranges for other users to have a “clean” terminal at login
    time.",
    "modify_ldt() reads or writes the local descriptor table (LDT) for
    a process.  The LDT is an array of segment descriptors that can
    be referenced by user code.",
    "pivot_root() changes the root mount in the mount namespace of the
    calling process.  More precisely, it moves the root mount to the
    directory put_old and makes new_root the new root mount.",
    "The _sysctl() call reads and/or writes kernel parameters.  For
    example, the hostname, or the maximum number of open files.",
    "prctl() manipulates various aspects of the behavior of the
    calling thread or process. Note that careless use of some prctl() operations can confuse the
    user-space run-time environment, so these operations should be
    used with care.",
    "arch_prctl() sets architecture-specific process or thread state.
    code selects a subfunction and passes argument addr to it; addr
    is interpreted as either an unsigned long for the set
    operations, or as an unsigned long *, for the get operations.",
    "The system call adjtimex() reads and optionally sets
    adjustment parameters for this algorithm.  It takes a pointer to
    a timex structure, updates kernel parameters from (selected)
    field values, and returns the same structure updated with the
    current kernel values.",
    "The setrlimit() system call set resource limits.",
    "chroot() changes the root directory of the calling process to
    that specified in path.  This directory will be used for
    pathnames beginning with /.  The root directory is inherited by
    all children of the calling process.",
    "sync() causes all pending modifications to filesystem metadata
    and cached file data to be written to the underlying filesystems.",
    "The acct() system call enables or disables process accounting.
    If called with the name of an existing file as its argument,
    accounting is turned on, and records for each terminating process
    are appended to filename as it terminates.  An argument of NULL
    causes accounting to be turned off.",
    "The function settimeofday() can set the time as well as a timezone.",
    "mount() attaches the filesystem specified by source (which is
    often a pathname referring to a device, but can also be the
    pathname of a directory or file, or a dummy string) to the
    location (a directory or file) specified by the pathname in
    target.",
    "Linux 2.1.116 added the umount2() system call, which, like
    umount(), unmounts a target, but allows additional flags
    controlling the behavior of the operation",
    "swapon() sets the swap area to the file or block device specified
    by path.",
    "swapoff() stops swapping to the file or block device
    specified by path.",
    "The reboot() call reboots the system, or enables/disables the
    reboot keystroke",
    "sethostname() sets the hostname to the value given in the
    character array name.  The len argument specifies the number of
    bytes in name.",
    "getdomainname() returns the null-terminated domain name in the
    character array name, which has a length of len bytes.  If the
    null-terminated domain name requires more than len bytes,
    getdomainname() returns the first len bytes (glibc) or gives an
    error (libc).",
    "iopl() changes the I/O privilege level of the calling thread, as
    specified by the two least significant bits in level.
    The I/O privilege level for a normal thread is 0.  Permissions
    are inherited from parents to children.",
    "ioperm() sets the port access permission bits for the calling
    thread for num bits starting from port address from.  If turn_on
    is nonzero, then permission for the specified bits is enabled;
    otherwise it is disabled.  If turn_on is nonzero, the calling
    thread must be privileged (CAP_SYS_RAWIO).",
    "create_module() attempts to create a loadable module entry and
    reserve the kernel memory that will be needed to hold the module.
    This system call requires privilege.",
    "init_module() loads an ELF image into kernel space, performs any
    necessary symbol relocations, initializes module parameters to
    values provided by the caller, and then runs the module's init
    function.  This system call requires privilege.",
    "The delete_module() system call attempts to remove the unused
    loadable module entry identified by name.  If the module has an
    exit function, then that function is executed before unloading
    the module.  The flags argument is used to modify the behavior of
    the system call, as described below.  This system call requires
    privilege.",
    "get_kernel_syms - retrieve exported kernel and module symbols",
    "query_module() requests information from the kernel about
    loadable modules.  The returned information is placed in the
    buffer pointed to by buf.  The caller must specify the size of
    buf in bufsize.",
    "The quotactl() call manipulates disk quotas.  The cmd argument
    indicates a command to be applied to the user or group ID
    specified in id.  To initialize the cmd argument, use the
    QCMD(subcmd, type) macro.",
    "nfsservctl - syscall interface to kernel nfs daemon. Since Linux 3.1, 
    this system call no longer exists.  It has been replaced by a set of 
    files in the nfsd filesystem",
    "The getpmsg(2) and
    putpmsg(2) calls are for kernels patched to support STREAMS, and
    may never be in the standard kernel.",
    "The getpmsg(2) and
    putpmsg(2) calls are for kernels patched to support STREAMS, and
    may never be in the standard kernel.",
    "Unimplemented system calls.",
    "Unimplemented system calls.",
    "Unimplemented system calls.",
    "gettid() returns the caller's thread ID (TID).  In a single-
    threaded process, the thread ID is equal to the process ID (PID,
    as returned by getpid(2)).  In a multithreaded process, all
    threads have the same PID, but each one has a unique TID.",
    "readahead() initiates readahead on a file so that subsequent
    reads from that file will be satisfied from the cache, and not
    block on disk I/O (assuming the readahead was initiated early
    enough and that other activity on the system did not in the
    meantime flush pages from the cache).",
    "setxattr() sets the value of the extended attribute identified by
    name and associated with the given path in the filesystem.  The
    size argument specifies the size (in bytes) of value; a zero-
    length value is permitted.",
    "lsetxattr() is identical to setxattr(), except in the case of a
    symbolic link, where the extended attribute is set on the link
    itself, not the file that it refers to.",
    "fsetxattr() is identical to setxattr(), only the extended
    attribute is set on the open file referred to by fd (as returned
    by open(2)) in place of path.",
    "getxattr() retrieves the value of the extended attribute
    identified by name and associated with the given path in the
    filesystem.  The attribute value is placed in the buffer pointed
    to by value; size specifies the size of that buffer.  The return
    value of the call is the number of bytes placed in value.",
    "lgetxattr() is identical to getxattr(), except in the case of a
    symbolic link, where the link itself is interrogated, not the
    file that it refers to.",
    "fgetxattr() is identical to getxattr(), only the open file
    referred to by fd (as returned by open(2)) is interrogated in
    place of path.",
    "listxattr() retrieves the list of extended attribute names
    associated with the given path in the filesystem.  The retrieved
    list is placed in list, a caller-allocated buffer whose size (in
    bytes) is specified in the argument size.  The list is the set of
    (null-terminated) names, one after the other.",
    "llistxattr() is identical to listxattr(), except in the case of a
    symbolic link, where the list of names of extended attributes
    associated with the link itself is retrieved, not the file that
    it refers to.",
    "flistxattr() is identical to listxattr(), only the open file
    referred to by fd (as returned by open(2)) is interrogated in
    place of path.",
    "removexattr() removes the extended attribute identified by name
    and associated with the given path in the filesystem.",
    "lremovexattr() is identical to removexattr(), except in the case
    of a symbolic link, where the extended attribute is removed from
    the link itself, not the file that it refers to.",
    "fremovexattr() is identical to removexattr(), only the extended
    attribute is removed from the open file referred to by fd (as
    returned by open(2)) in place of path.",
    "tkill() is an obsolete predecessor to tgkill().  It allows only
    the target thread ID to be specified, which may result in the
    wrong thread being signaled if a thread terminates and its thread
    ID is recycled.  Avoid using this system call.",
    "time() returns the time as the number of seconds since the Epoch,
    1970-01-01 00:00:00 +0000 (UTC).",
    "The futex() system call provides a method for waiting until a
    certain condition becomes true.  It is typically used as a
    blocking construct in the context of shared-memory
    synchronization.",
    "sched_setaffinity() sets the CPU affinity mask of the thread
    whose ID is pid to the value specified by mask.  If pid is zero,
    then the calling thread is used.  The argument cpusetsize is the
    length (in bytes) of the data pointed to by mask.  Normally this
    argument would be specified as sizeof(cpu_set_t).",
    "sched_getaffinity() writes the affinity mask of the thread whose
    ID is pid into the cpu_set_t structure pointed to by mask.  The
    cpusetsize argument specifies the size (in bytes) of mask.  If
    pid is zero, then the mask of the calling thread is returned.",
    "set_thread_area - manipulate thread-local
    storage information",
    "The io_setup() system call creates an asynchronous I/O context
    suitable for concurrently processing nr_events operations.  The
    ctx_idp argument must not point to an AIO context that already
    exists, and must be initialized to 0 prior to the call.",
    "The io_destroy() system call will attempt to cancel all
    outstanding asynchronous I/O operations against ctx_id, will
    block on the completion of all operations that could not be
    canceled, and will destroy the ctx_id.",
    "Read asynchronous I/O events from the completion
    queue. The io_getevents() system call attempts to read at least min_nr
    events and up to nr events from the completion queue of the AIO
    context specified by ctx_id.",
    "The io_submit() system call queues nr I/O request blocks for
    processing in the AIO context ctx_id.  The iocbpp argument should
    be an array of nr AIO control blocks, which will be submitted to
    context ctx_id.",
    "The io_cancel() system call attempts to cancel an asynchronous
    I/O operation previously submitted with io_submit(2).",
    "get_thread_area - manipulate thread-local
    storage information",
    "lookup_dcookie - return a directory entry's path",
    "epoll_create - open an epoll file descriptor",
    "epoll_ctl - control interface for an epoll file descriptor",
    "epoll_wait - wait for an I/O event on an epoll file
    descriptor",
    "The remap_file_pages() system call is used to create a nonlinear
    mapping, that is, a mapping in which the pages of the file are
    mapped into a nonsequential order in memory.",
    "The original Linux getdents() system call did not handle large
    filesystems and large file offsets.  Consequently, Linux 2.4
    added getdents64(), with wider types for the d_ino and d_off
    fields.  In addition, getdents64() supports an explicit d_type
    field.",
    "set_tid_address - set pointer to thread ID",
    "The restart_syscall() system call is used to restart certain
    system calls after a process that was stopped by a signal (e.g.,
    SIGSTOP or SIGTSTP) is later resumed after receiving a SIGCONT
    signal.  This system call is designed only for internal use by
    the kernel.",
    "semtimedop() behaves identically to semop() except that in those
    cases where the calling thread would sleep, the duration of that
    sleep is limited by the amount of elapsed time specified by the
    timespec structure whose address is passed in the timeout
    argument.",
    "Programs can use posix_fadvise() to announce an intention to
    access file data in a specific pattern in the future, thus
    allowing the kernel to perform appropriate optimizations.",
    "timer_create() creates a new per-process interval timer.  The ID
    of the new timer is returned in the buffer pointed to by timerid,
    which must be a non-null pointer.  This ID is unique within the
    process, until the timer is deleted.  The new timer is initially
    disarmed.",
    "timer_settime() arms or disarms the timer identified by timerid.
    The new_value argument is pointer to an itimerspec structure that
    specifies the new initial value and the new interval for the
    timer",
    "timer_gettime() returns the time until next expiration, and the
    interval, for the timer specified by timerid, in the buffer
    pointed to by curr_value.",
    "timer_getoverrun() returns the overrun count for the timer
    referred to by timerid.  An application can use the overrun count
    to accurately calculate the number of timer expirations that
    would have occurred over a given time interval.",
    "timer_delete() deletes the timer whose ID is given in timerid.
    If the timer was armed at the time of this call, it is disarmed
    before being deleted.  The treatment of any pending signal
    generated by the deleted timer is unspecified.",
    "The function clock_settime() set the time of the specified clock clockid.",
    "The function clock_gettime() retrieve the time of the specified clock clockid.",
    "The function clock_getres() finds the resolution (precision) of
    the specified clock clockid, and, if res is non-NULL, stores it
    in the struct timespec pointed to by res.",
    "clock_nanosleep() allows the calling thread to
    sleep for an interval specified with nanosecond precision.  It
    differs in allowing the caller to select the clock against which
    the sleep interval is to be measured, and in allowing the sleep
    interval to be specified as either an absolute or a relative
    value.",
    "This system call is equivalent to _exit(2) except that it
    terminates not only the calling thread, but all threads in the
    calling process's thread group.",
    "The epoll_wait() system call waits for events on the epoll(7)
    instance referred to by the file descriptor epfd.  The buffer
    pointed to by events is used to return information from the ready
    list about file descriptors in the interest list that have some
    events available.",
    "This system call is used to add, modify, or remove entries in the
    interest list of the epoll(7) instance referred to by the file
    descriptor epfd.  It requests that the operation op be performed
    for the target file descriptor, fd.",
    "tgkill() sends the signal sig to the thread with the thread ID
    tid in the thread group tgid.  (By contrast, kill(2) can be used
    to send a signal only to a process (i.e., thread group) as a
    whole, and the signal will be delivered to an arbitrary thread
    within that process.)",
    "The utime() system call changes the access and modification times
    of the inode specified by filename to the actime and modtime
    fields of times respectively.",
    "Unimplemented system calls.",
    "mbind() sets the NUMA memory policy, which consists of a policy
    mode and zero or more nodes, for the memory range starting with
    addr and continuing for len bytes.  The memory policy defines
    from which node memory is allocated.",
    "set_mempolicy() sets the NUMA memory policy of the calling
    thread, which consists of a policy mode and zero or more nodes,
    to the values specified by the mode, nodemask, and maxnode
    arguments.",
    "get_mempolicy() retrieves the NUMA policy of the calling thread
    or of a memory address, depending on the setting of flags.",
    "mq_open() creates a new POSIX message queue or opens an existing
    queue.  The queue is identified by name.",
    "mq_unlink() removes the specified message queue name.  The
    message queue name is removed immediately.  The queue itself is
    destroyed once any other processes that have the queue open close
    their descriptors referring to the queue.",
    "mq_timedsend() behaves just like mq_send(), except that if the
    queue is full and the O_NONBLOCK flag is not enabled for the
    message queue description, then abs_timeout points to a structure
    which specifies how long the call will block.",
    "mq_timedreceive() behaves just like mq_receive(), except that if
    the queue is empty and the O_NONBLOCK flag is not enabled for the
    message queue description, then abs_timeout points to a structure
    which specifies how long the call will block.",
    "mq_notify() allows the calling process to register or unregister
    for delivery of an asynchronous notification when a new message
    arrives on the empty message queue referred to by the message
    queue descriptor mqdes.",
    "mq_getsetattr - get/set message queue attributes. Do not use this system call.",
    "The kexec_load() system call loads a new kernel that can be
    executed later by reboot(2).",
    "The waitid() system call (available since Linux 2.6.9) provides
    more precise control over which child state changes to wait for.",
    "add_key() creates or updates a key of the given type and
    description, instantiates it with the payload of length plen,
    attaches it to the nominated keyring, and returns the key's
    serial number.",
    "request_key() attempts to find a key of the given type with a
    description (name) that matches the specified description.  If
    such a key could not be found, then the key is optionally
    created.",
    "keyctl() allows user-space programs to perform key manipulation.",
    "The ioprio_set() system call set the
    I/O scheduling class and priority of one or more threads.",
    "The ioprio_get() system call get the
    I/O scheduling class and priority of one or more threads.",
    "inotify_init() initializes a new inotify instance and returns a
    file descriptor associated with a new inotify event queue.",
    "inotify_add_watch() adds a new watch, or modifies an existing
    watch, for the file whose location is specified in pathname; the
    caller must have read permission for this file.",
    "inotify_rm_watch() removes the watch associated with the watch
    descriptor wd from the inotify instance associated with the file
    descriptor fd.",
    "migrate_pages() attempts to move all pages of the process pid
    that are in memory nodes old_nodes to the memory nodes in
    new_nodes.",
    "openat() open a file",
    "mkdirat - create a directory",
    "mknodat - create a special or ordinary file",
    "fchownat - change ownership of a file",
    "futimesat - change timestamps of a file relative to a directory
    file descriptor",
    "newfstatat() retrieve information about the file pointed
    to by pathname;",
    "unlinkat deletes a name from the filesystem.  If that name was
    the last link to a file and no processes have the file open, the
    file is deleted and the space it was using is made available for
    reuse.",
    "renameat() renames a file, moving it between directories if
    required.  Any other hard links to the file (as created using
    link(2)) are unaffected.  Open file descriptors for oldpath are
    also unaffected.",
    "linkat() creates a new link (also known as a hard link) to an
    existing file.",
    "symlinkat() creates a symbolic link named linkpath which contains
    the string target.",
    "readlinkat() places the contents of the symbolic link pathname in
    the buffer buf, which has size bufsiz.",
    "fchmodat() changes the mode of the file specified whose pathname
    is given in pathname, which is dereferenced if it is a symbolic
    link.",
    "faccessat() checks whether the calling process can access the file
    pathname.  If pathname is a symbolic link, it is dereferenced.",
    "pselect6() - synchronous I/O multiplexing",
    "ppoll() allows an application to safely wait until either a file
    descriptor becomes ready or until a signal is caught.",
    "unshare() allows a process (or thread) to disassociate parts of
    its execution context that are currently being shared with other
    processes (or threads).",
    "set_robust_list - set list of robust futexes",
    "get_robust_list - get list of robust futexes",
    "splice() moves data between two file descriptors without copying
    between kernel address space and user address space.  It
    transfers up to len bytes of data from the file descriptor fd_in
    to the file descriptor fd_out, where one of the file descriptors
    must refer to a pipe.",
    "tee() duplicates up to len bytes of data from the pipe referred
    to by the file descriptor fd_in to the pipe referred to by the
    file descriptor fd_out.",
    "sync_file_range() permits fine control when synchronizing the
    open file referred to by the file descriptor fd with disk.",
    "vmsplice - splice user pages to/from a pipe",
    "move_pages() moves the specified pages of the process pid to the
    memory nodes specified by nodes.  The result of the move is
    reflected in status.  The flags indicate constraints on the pages
    to be moved.",
    "utimensat() - change file timestamps with nanosecond
    precision",
    "The epoll_wait() system call waits for events on the epoll(7)
    instance referred to by the file descriptor epfd.",
    "signalfd() creates a file descriptor that can be used to accept
    signals targeted at the caller.  This provides an alternative to
    the use of a signal handler or sigwaitinfo(2), and has the
    advantage that the file descriptor may be monitored by select(2),
    poll(2), and epoll(7).",
    "timerfd_create() creates a new timer object, and returns a file
    descriptor that refers to that timer.",
    "eventfd() creates an eventfd object that can be used as an
    event wait/notify mechanism by user-space applications, and by
    the kernel to notify user-space applications of events.",
    "fallocate() allows the caller to directly manipulate the
    allocated disk space for the file referred to by fd for the byte
    range starting at offset and continuing for len bytes.",
    "timerfd_settime() arms (starts) or disarms (stops) the timer
    referred to by the file descriptor fd.",
    "timerfd_gettime() returns, in curr_value, an itimerspec structure
    that contains the current setting of the timer referred to by the
    file descriptor fd.",
    "accept4 - accept a connection on a socket",
    "signalfd4 - create a file descriptor for accepting signals",
    "eventfd2 - create a file descriptor for event notification",
    "epoll_create1() creates a new epoll(7) instance.  Since Linux
    2.6.8, the size argument is ignored, but must be greater than
    zero",
    "dup3 - duplicate a file descriptor",
    "pipe2 - create pipe",
    "inotify_init1 - initialize an inotify instance",
    "The preadv() system call combines the functionality of readv()
    and pread(2).  It performs the same task as readv(), but adds a
    fourth argument, offset, which specifies the file offset at which
    the input operation is to be performed.",
    "The pwritev() system call combines the functionality of writev()
    and pwrite(2).  It performs the same task as writev(), but adds a
    fourth argument, offset, which specifies the file offset at which
    the output operation is to be performed.",
    "rt_tgsigqueueinfo() system call are
    the low-level interfaces used to send a signal plus data to a
    process or thread.",
    "A call to perf_event_open() creates a file descriptor that allows
    measuring performance information.  Each file descriptor
    corresponds to one event that is measured; these can be grouped
    together to measure multiple events simultaneously.",
    "The recvmmsg() system call is an extension of recvmsg(2) that
    allows the caller to receive multiple messages from a socket
    using a single system call.",
    "fanotify_init() initializes a new fanotify group and returns a
    file descriptor for the event queue associated with the group.",
    "fanotify_mark() adds, removes, or modifies an fanotify mark on a
    filesystem object.  The caller must have read permission on the
    filesystem object that is to be marked.",
    "The Linux-specific prlimit() system call combines and extends the
    functionality of setrlimit() and getrlimit().  It can be used to
    both set and get the resource limits of an arbitrary process.",
    "name_to_handle_at() returns an opaque handle that corresponds to
    a specified file",
    "open_by_handle_at() opens the file
    corresponding to a handle returned by a previous call to
    name_to_handle_at() and returns an open file descriptor.",
    "The clock_adjtime() system call (added in Linux 2.6.39) behaves
    like adjtimex() but takes an additional clk_id argument to
    specify the particular clock on which to act. - tune kernel clock",
    "syncfs() is like sync(), but synchronizes just the filesystem
    containing file referred to by the open file descriptor fd.
    - commit filesystem caches to disk",
    "The sendmmsg() system call is an extension of sendmsg(2) that
    allows the caller to transmit multiple messages on a socket using
    a single system call.",
    "The setns() system call allows the calling thread to move into
    different namespaces.",
    "The getcpu() system call identifies the processor and node on
    which the calling thread or process is currently running and
    writes them into the integers pointed to by the cpu and node
    arguments",
    "The process_vm_readv() system call transfers data from the remote
    process to the local process.",
    "The process_vm_writev() system call is the converse of
    process_vm_readv()—it transfers data from the local process to
    the remote process.",
    "The kcmp() system call can be used to check whether the two
    processes identified by pid1 and pid2 share a kernel resource
    such as virtual memory, file descriptors, and so on.",
    "The finit_module() system call is like init_module(), but reads
    the module to be loaded from the file descriptor fd. - load a kernel module",
    "The sched_setattr() system call sets the scheduling policy and
    associated attributes for the thread whose ID is specified in
    pid.  If pid equals zero, the scheduling policy and attributes of
    the calling thread will be set.",
    "The sched_getattr() system call fetches the scheduling policy and
    the associated attributes for the thread whose ID is specified in
    pid.  If pid equals zero, the scheduling policy and attributes of
    the calling thread will be retrieved.",
    "renameat() renames a file, moving it between directories if
    required.  Any other hard links to the file (as created using
    link(2)) are unaffected.  Open file descriptors for oldpath are
    also unaffected.",
    "The seccomp() system call operates on the Secure Computing
    (seccomp) state of the calling process.",
    "The getrandom() system call fills the buffer pointed to by buf
    with up to buflen random bytes.  These bytes can be used to seed
    user-space random number generators or for cryptographic
    purposes.",
    " memfd_create() creates an anonymous file and returns a file
    descriptor that refers to it.  The file behaves like a regular
    file, and so can be modified, truncated, memory-mapped, and so
    on.",
    "The kexec_load() system call loads a new kernel that can be
    executed later by reboot(2).",
    "The bpf() system call performs a range of operations related to
    extended Berkeley Packet Filters.",
    "The execveat() system call executes the program referred to by
    the combination of dirfd and pathname.",
    "userfaultfd() creates a new userfaultfd object that can be used
    for delegation of page-fault handling to a user-space
    application, and returns a file descriptor that refers to the new
    object.",
    "The membarrier() system call helps reducing the overhead of the
    memory barrier instructions required to order memory accesses on
    multi-core systems.",
    "mlock2() also locks pages in the specified range starting at addr
    and continuing for len bytes.  However, the state of the pages
    contained in that range after the call returns successfully will
    depend on the value in the flags argument.",
    "The copy_file_range() system call performs an in-kernel copy
    between two file descriptors without the additional cost of
    transferring data from the kernel to user space and then back
    into the kernel.",
    "These system calls are similar to preadv() and pwritev() calls,
    but add a fifth argument, flags, which modifies the behavior on a
    per-call basis.",
    "These system calls are similar to preadv() and pwritev() calls,
    but add a fifth argument, flags, which modifies the behavior on a
    per-call basis.",
    "pkey_mprotect() changes the access protections for the calling
    process's memory pages containing any part of the address range
    in the interval [addr, addr+len-1].  addr must be aligned to a
    page boundary.",
    "pkey_alloc() allocates a protection key (pkey) and allows it to
    be passed to pkey_mprotect(2).",
    "pkey_free() frees a protection key and makes it available for
    later allocations.  After a protection key has been freed, it may
    no longer be used in any protection-key-related operations.",
];