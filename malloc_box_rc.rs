#![allow(dead_code)]

use std::{
    alloc::{alloc, dealloc, handle_alloc_error, Layout},
    rc::Rc,
};

/// Rust 堆内存管理
///
/// 1) *cosnt T or *mut T: std::alloc::Allocator 手动管理
/// 2) Box<T>: Unique<T> 单一所有权
/// 3) Rc<T>: 共享所有权, 引用计数
///

#[test]
fn test_alloc() {
    struct MyVec {
        data_ptr: *mut u8,
        cap: usize,
        len: usize,
    }

    impl MyVec {
        fn new(cap: usize) -> Self {
            let layout = Layout::array::<u8>(cap).unwrap();
            let data_ptr = unsafe {
                let ptr = alloc(layout);
                if ptr.is_null() {
                    handle_alloc_error(layout);
                }
                ptr
            };
            println!("alloc: cap: {}", cap);
            Self {
                data_ptr,
                cap,
                len: 0,
            }
        }
    }

    impl Drop for MyVec {
        fn drop(&mut self) {
            if !self.data_ptr.is_null() {
                unsafe {
                    dealloc(self.data_ptr, Layout::array::<u8>(self.cap).unwrap());
                }
                println!("dealloc: cap: {}", self.cap);
            }
        }
    }

    let v = MyVec::new(10);
    assert_eq!(10, v.cap);
    let _ = v;
}

#[test]
fn test_box() {
    // 单一所有权，生命周期与栈内存直接关联
    let b = Box::new("foo".to_owned()); // allocator.alloc(..) 分配 String 到堆内存，返回指针给栈内存 b
    let _ = b; // 栈内存 b 被回收并调用 drop(b) 使用 allocator.dealloc(..) 回收对应的推内存
}

#[test]
fn test_rc() {
    // 共享所有权，生命周期与栈内存间接关联
    let rc = Rc::new("foo".to_owned()); // allocator.alloc(..) 分配 String 到堆内存，返回指针给栈内存 rc

    let rc2 = rc.clone(); // strong_ref_count + 1

    let _ = rc; // 栈内存 rc 被回收并调用 drop(rc) 对 strong_ref_count - 1
    let _ = rc2; // 栈内存 rc2 被回收并调用 drop(rc) 对 strong_ref_count - 1, 此时 strong_ref_count == 0, 使用 allocator.dealloc(..) 回收对应的推内存
}
