// In embedded the determinism is crucial 

use std::ffi::c_void;
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

const POOL_SIZE: usize = 1024 * 1024; // 1 MB pool
const BLOCK_SIZE: usize = 64;         // fixed block size (bytes)
const MAX_BLOCKS: usize = POOL_SIZE / BLOCK_SIZE;

static mut MEMORY_POOL: [u8; POOL_SIZE] = [0; POOL_SIZE];

static mut FREE_LIST: [usize; MAX_BLOCKS] = [0; MAX_BLOCKS];

static FREE_HEAD: AtomicUsize = AtomicUsize::new(0);

static INIT: AtomicUsize = AtomicUsize::new(0);

fn init_pool() {
    unsafe {
        for i in 0..MAX_BLOCKS - 1 {
            FREE_LIST[i] = i + 1;
        }
        FREE_LIST[MAX_BLOCKS - 1] = usize::MAX;
    }

    FREE_HEAD.store(0, Ordering::SeqCst);
}

#[no_mangle]
pub extern "C" fn aligned_allocator_init() {
    if INIT.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
        init_pool();
    }
}

#[no_mangle]
pub extern "C" fn aligned_alloc(_alignment: usize, size: usize) -> *mut c_void {
    // Deterministic constraint: only support <= BLOCK_SIZE
    if size > BLOCK_SIZE {
        return ptr::null_mut();
    }

    let head = FREE_HEAD.load(Ordering::SeqCst);

    if head == usize::MAX {
        return ptr::null_mut(); // pool exhausted
    }

    let next;
    unsafe {
        next = FREE_LIST[head];
    }

    FREE_HEAD.store(next, Ordering::SeqCst);

    unsafe {
        let ptr = MEMORY_POOL.as_mut_ptr().add(head * BLOCK_SIZE);
        ptr as *mut c_void
    }
}

#[no_mangle]
pub extern "C" fn aligned_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let base = MEMORY_POOL.as_ptr() as usize;
        let offset = (ptr as usize) - base;

        let index = offset / BLOCK_SIZE;

        let head = FREE_HEAD.load(Ordering::SeqCst);

        FREE_LIST[index] = head;
        FREE_HEAD.store(index, Ordering::SeqCst);
    }
}

struct aligned_allocator{
    
}

impl aligned_allocator{

}

