// This library is a word-by-word unsafe translation of the following repo:
//      https://github.com/evanw/buddy-malloc/
// which is under the MIT license.

use std::ptr::null_mut;

const HEADER_SIZE: usize = 4;
const MIN_ALLOC_LOG2: usize = 2;
const MIN_ALLOC: usize = 1 << MIN_ALLOC_LOG2;

// We can allocate a full block of up to 128 MB.
const MAX_ALLOC_LOG2: usize = 27;
const MAX_ALLOC: usize = 1 << MAX_ALLOC_LOG2;

const BUCKET_COUNT: usize = MAX_ALLOC_LOG2 - MIN_ALLOC_LOG2 + 1;

#[derive(Default)]
struct ListT {
    prev: *mut Self,
    next: *mut Self,
}

static mut BUCKETS: [ListT; BUCKET_COUNT] = [ListT::default(); BUCKET_COUNT];
static mut BUCKET_LIMIT: u32 = 0u32;

static mut NODE_IS_SPLIT: [u8; (1 << (BUCKET_COUNT - 1)) / 8]
  = [0u8; (1 << (BUCKET_COUNT - 1)) / 8] ;

static mut BASE_PTR: u32 = 0u32;
static mut MAX_PTR: u32 = 0u32;

#[inline]
unsafe fn update_max_ptr(new_value: u32) -> u32 {
    if new_value > MAX_PTR {
        if new_value >= 0x0C00000 {
            return 0;
        }
        MAX_PTR = new_value;
    }
    return 1;
}

impl ListT {
    fn init(&mut self) {
        self.prev = self as *mut ListT;
        self.next = self.prev;
    }

    unsafe fn push(&mut self, entry: *mut ListT) {
        let prev = self.prev;
        (*entry).prev = prev;
        (*entry).next = self as *mut ListT;
        (*prev).next = entry;
        self.prev = entry;
    }

    unsafe fn remove(&mut self) {
        let prev = self.prev;
        let next = self.next;
        (*prev).next = next;
        (*next).prev = prev;
    }

    unsafe fn list_pop(&mut self) -> *mut ListT {
        let back = self.prev;
        if back == (self as *mut ListT) {
            return null_mut();
        } else {
            (*back).remove();
            return back;
        }
    }
}

#[inline]
unsafe fn ptr_for_node(index: u32, bucket: u32) -> u32 {
    return BASE_PTR + ((index - (1 << bucket) + 1) << (MAX_ALLOC_LOG2 - bucket));
}

#[inline]
unsafe fn node_for_ptr(ptr: u32, bucket: u32) -> u32 {
    return ((ptr - BASE_PTR) >> (MAX_ALLOC_LOG2 - bucket)) + (1 << bucket) - 1;
}

#[inline]
unsafe fn parent_is_split(index: u32) -> bool{
    let index = (index - 1) / 2;
    return (NODE_IS_SPLIT[index / 8] >> (index % 8)) == 1;
}

#[inline]
unsafe fn flip_parent_is_split(index: u32) {
    let index = (index - 1) / 2;
    NODE_IS_SPLIT[index / 8] ^= 1 << (index % 8);
}

#[inline]
unsafe fn bucket_for_request(request: u32) -> u32 {
    let mut bucket = (BUCKET_COUNT - 1) as u32;
    let mut size = MIN_ALLOC as u32;

    while size < request {
        bucket -= 1;
        size *= 2;
    }

    return bucket;
}

#[inline]
unsafe fn lower_bucket_limit(bucket: u32) {
    while bucket < BUCKET_LIMIT {
        let root = node_for_ptr(BASE_PTR, BUCKET_LIMIT);
        let mut right_child: u32;

        if !parent_is_split(root) {
            (*(BASE_PTR as *mut ListT)).remove();
            BUCKET_LIMIT -= 1;
            BUCKETS[BUCKET_LIMIT as usize].init();
            BUCKETS[BUCKET_LIMIT as usize].push(BASE_PTR as *mut ListT);
            continue;
        }

        right_child = ptr_for_node(root + 1, BUCKET_LIMIT);
        if !update_max_ptr(right_child + core::mem::size_of::<ListT>()) {

        }
    }
}