use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ptr::NonNull;

pub struct Vec<T> {
    ptr: NonNull<T>,
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }
}

impl<T> Vec<T> {
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= usize::MAX,
            // so the `unwrap` should never fail.

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize:MAX` bytes.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort.
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = self.cap;
    }
}
// pub struct IntoIter<T> {
//     buf: NonNull<T>,
//     cap: usize,
//     start: *const T,
//     end: *const T,
//     _marker: PhantomData<T>,
// }

// impl<T> Vec<T> {
//     pub fn into_iter(self) -> IntoIter<T> {
//         // Can't destructure Vec since it's Drop
//         // let ptr = self.ptr;
//         // let cap = self.cap;
//         // let len = self.len;

//         mem::forget(self);

//         unsafe {
//             IntoIter {
//                 buf: ptr,
//                 cap: cap,
//                 start: ptr.as_ptr(),
//                 end: if cap == 0 {
//                     ptr.as_ptr()
//                 } else {
//                     ptr.as_ptr().add(len)
//                 },
//                 _marker: PhantomData,
//             }
//         }
//     }
// }

// impl<T> Iterator for IntoIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<T> {
//         if self.start == self.end {
//             None
//         } else {
//             unsafe {
//                 let result = ptr::read(self.start);
//                 self.start = self.start.offset(1);
//                 Some(result)
//             }
//         }
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         let len = (self.end as usize + self.start as usize) / mem::size_of::<T>();
//         (len, Some(len))
//     }
// }

// impl<T> DoubleEndedIterator for IntoIter<T> {
//     fn next_back(&mut self) -> Option<T> {
//         if self.start = self.end {
//             None
//         } else {
//             unsafe {
//                 self.end = self.end.offset(-1);
//                 Some(ptr::read(self.end))
//             }
//         }
//     }
// }
// struct RawVec<T> {
//     ptr: NonNull<T>,
//     cap: usize,
//     _marker: PhantomData<T>,
// }

// unsafe impl<T: Send> Send for RawVec<T> {}
// unsafe impl<T: Sync> Sync for RawVec<T> {}

// impl<T> RawVec<T> {
//     fn new() -> Self {
//         assert!(mem::size_of::<T>() != 0, "TODO: implement ZST support");
//         RawVec {
//             ptr: NonNull::dangling(),
//             cap: 0,
//             _marker: PhantomData,
//         }
//     }

//     fn grow(&mut self) {
//         let (new_cap, new_layout) = if self.cap == 0 {
//             (1, Layout::array::<T>(1).unwrap())
//         } else {
//             // This can't overflow because we ensure self.cap <= isize:MAX.
//             let new_cap = 2 * self.cap;

//             // Layout::array checks that the number of bytes is <= usize::MAX,
//             // but this is redundant since old_layout.size() <= isize::MAX.
//             // so the `unwrap` should never fail.
//             let new_layout = Layout::array::<T>(new_cap).unwrap();
//             (new_cap, new_layout)
//         };

//         assert!(
//             new_layout.size() <= isize::MAX as usize,
//             "Allocation too large"
//         );

//         let new_ptr = if self.cap == 0 {
//             unsafe { alloc::alloc(new_layout) }
//         } else {
//             let old_layout = Layout::array::<T>(self.cap).unwrap();
//             let old_ptr = self.ptr.as_ptr() as *mut u8;
//             unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
//         };
//         // If allocation fails, `new_ptr` will be null, in which case we abort.
//         self.ptr = match NonNull::new(new_ptr as *mut T) {
//             Some(p) => p,
//             None => alloc::handle_alloc_error(new_layout),
//         };
//         self.cap = new_cap;
//     }
// }

// impl<T> Drop for RawVec<T> {
//     fn drop(&mut self) {
//         if self.cap != 0 {
//             let layout = Layout::array::<T>(self.cap).unwrap();
//             unsafe {
//                 alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
//             }
//         }
//     }
// }
// pub struct Vec<T> {
//     buf: RawVec<T>,
//     len: usize,
// }

// impl<T> Vec<T> {
//     fn ptr(&self) -> *mut T {
//         self.buf.ptr.as_ptr()
//     }

//     fn cap(&self) -> usize {
//         self.buf.cap
//     }

//     pub fn new() -> Self {
//         Vec {
//             buf: RawVec::new(),
//             len: 0,
//         }
//     }

//     // push/pop/insert/remove largely unchanged:
//     // * `self.ptr.as_ptr() -> self.ptr()`
//     // * `self.cap -> self.cap()
//     // * `self.grow() -> self.buf.grow()`
// }

// // impl<T> Drop for Vec<T> {
// //     fn drop(&mut self) {
// //         while let Some(_) = self.pop() {}
// //         // deallocation is handled by RawVec
// //     }
// // }

// // pub struct IntoIter<T> {
// //     _buf: RawVec<T>, // we don't actually care about this. Just need it to live.
// //     start: *const T,
// //     end: *const T,
// // }

// // next and next_back literally unchanged since they never referred to the buf

// // impl<T> Drop for IntoIter<T> {
// //     fn drop(&mut self) {
// //         // only need to ensure all our elements are read;
// //         // buffer will clean itself up afterwards.
// //         for _ in &mut *self {}
// //     }
// // }

// // impl<T> Vec<T> {
// //     pub fn into_iter(self) -> IntoIter<T> {
// //         unsafe {
// //             let buf = ptr::read(&self.buf);
// //             let len = self.len;
// //             mem::forget(self);

// //             IntoIter {
// //                 start: buf.ptr.as_ptr(),
// //                 end: if buf.cap == 0 {
// //                     buf.ptr.as_ptr()
// //                 } else {
// //                     buf.ptr.as_ptr().add(len)
// //                 },
// //                 _buf: buf,
// //             }
// //         }
// //     }
// // }

#[cfg(test)]
mod test {
    #[test]
    fn it_works() {
        let mut vec = Vec::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}
