use std::collections::HashMap;
use std::pin::Pin;
use std::ptr::null;
use std::sync::Arc;
use std::sync::mpsc::{channel, Receiver, sync_channel};
use std::task::{Context, Poll};
use std::thread::sleep;
use std::time::Duration;
use crossbeam_channel::unbounded;
use futures::Stream;

// // Exercise 1: STD  channels
// fn main() {
//
//     // syscall init
//     // OS -> [..............................................................................] -> computation -> serve
//     // OS -> [.] -> computation -> serve
//     // backpressure konusu
//
//     // Create a simple streaming channel
//     let (tx, rx) = sync_channel::<Vec<u8>>(1);
//     let tx2 = tx.clone();
//     std::thread::spawn(move|| {
//         tx.send(vec![0_u8; 20]).unwrap();
//     });
//
//     println!("before 20");
//
//     tx2.try_send(vec![1_u8; 20]).unwrap();
//     tx2.send(vec![3_u8; 20]).unwrap();
//
//     println!("after 20");
//
//     // Main thread
//     assert_eq!(rx.recv().unwrap(), vec![0_u8; 20]);
//     // assert_eq!(rx.recv().unwrap(), 10);
//     // assert_eq!(rx.recv().unwrap(), 20);
//     sleep(Duration::from_secs(10));
// }


// // Exercise 2: mpmc channels
// fn main() {
//     let (tx, rx) = unbounded::<usize>();
//
//     tx.clone();
//     rx.clone();
//     tx.clone();
//     rx.clone();
// }

// // Exercise 3: Future generation mental model
// #[derive(Clone)]
// pub struct RTStream {
//     rx: Receiver<usize>,
// }

// enum FutureResult {
//     Ready = 0,
//     Pending = 1
// }


// // Exercise 4: Immutable data sharing between futures.
// #[derive(Debug)]
// pub struct Person {
//     age: usize,
//     name: String,
//     surname: String
// }
//
//
// impl Person {
//     pub fn new(age: usize, name: String, surname: String) -> Self {
//         Self {
//             age,
//             name,
//             surname
//         }
//     }
// }
//
// fn main() {
//     let theo = Person::new(15, "theo".into(), "bulut".into());
//     let theo = Arc::new(theo);
//     let theo1 = theo.clone();
//
//
//     let tm = async {
//         // blocking - 10s
//         let fs = std::fs::File::open("test").unwrap();
//         theo1
//     };
//
//     let tms = async {
//         theo.age
//     };
//
//     let runner = async {
//         tm.await.age + tms.await
//     };
//
//     let x = futures::executor::block_on(runner);
//
//     dbg!(&x);
// }

// // Exercise 5: Stream implementation

// pub struct RTStream {}
//
// impl Stream for RTStream {
//     type Item = usize;
//
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
//         todo!()
//     }
//
//     fn size_hint(&self) -> (usize, Option<usize>) {
//         todo!()
//     }
// }

// // Exercise 6: Can't share raw pointer across thread boundaries.
// #[derive(Clone)]
// pub struct RTRawPointerExample {
//     age: usize,
//     __priv: *const ()
// }

// fn main() {
//     let (tx, rx) = unbounded::<usize>();
//
//     let rpe = RTRawPointerExample {
//         age: 32,
//         __priv: null()
//     };
//
//     // Sakin bundan baska birsey kullanma: Arc, Rc, Mutex, RwLock
//     let rpe = Arc::new(rpe);
//
//     std::thread::spawn(move|| {
//         rpe
//     });
// }


// mutable data sharing between futures, pin projection, spawn, spawn_blocking, future_combinatorlar, streamler, stream_combinators 2. derse

// 3.ders TM, STM, HTM. conc types