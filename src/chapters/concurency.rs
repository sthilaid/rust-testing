use core::fmt::Debug;
use std::cmp::*;
use std::sync::{Arc, RwLock};
use std::thread;

pub fn run() {
    println!("\n*****************************************************************");
    println!("concurency");
    println!("*****************************************************************");

    channel_test();
    multithread_quicksort_test();
}

fn summify(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..n + 1 {
        result += i
    }
    result
}

fn channel_test() {
    println!("\n--- channel test ---\n");

    let locked_should_stop: Arc<RwLock<bool>> = Arc::new(RwLock::new(false));
    let locked_v: Arc<RwLock<Vec<u32>>> = Arc::new(RwLock::new(vec![]));
    let (tx, rx) = std::sync::mpsc::channel();

    let locked_v_clone = Arc::clone(&locked_v);
    let locked_should_stop_clone = Arc::clone(&locked_should_stop);
    let generator_handle = thread::spawn(move || {
        let mut v = locked_v_clone.write().unwrap();
        for _ in 0..50 {
            let n = rx.recv().unwrap();
            v.push(summify(n));
        }
        let mut should_stop = locked_should_stop_clone.write().unwrap();
        *should_stop = true;
    });

    for _ in 0..8 {
        let tx_copy = tx.clone();
        let locked_should_stop_clone = Arc::clone(&locked_should_stop);
        thread::spawn(move || {
            let mut i = 2;
            while {
                let should_stop = locked_should_stop_clone.read().unwrap();
                !(*should_stop)
            } {
                tx_copy.send(i).unwrap();
                i += 1;
                std::thread::yield_now();
            }
        });
    }
    generator_handle.join().unwrap();
    let v = locked_v.read().unwrap();
    println!("v: {:?}", *v);
}

fn swap<T: Copy + Debug>(locked_v: Arc<RwLock<Vec<T>>>, i: usize, j: usize) {
    let mut v = locked_v.write().unwrap();
    let tmp = v[i];
    v[i] = v[j];
    v[j] = tmp;
}

fn partition<T: Ord + Copy + Debug>(locked_v: Arc<RwLock<Vec<T>>>, lo: usize, hi: usize) -> usize {
    let mut i = lo;
    let pivot = {
        let v = locked_v.read().unwrap();
        v[hi]
    };
    //println!("partition({:?}) pivot: {:?}", v, pivot);
    for j in lo..(hi + 1) {
        let vj = {
            let v = locked_v.read().unwrap();
            v[j]
        };
        if vj < pivot {
            swap(Arc::clone(&locked_v), i, j);
            i += 1;
        }
    }
    swap(Arc::clone(&locked_v), i, hi);
    i
}

fn bubblesort<T: Ord + Copy + Debug + Send + Sync + 'static>(
    locked_v: Arc<RwLock<Vec<T>>>,
    lo: usize,
    hi: usize,
) {
    let mut swapped = true;
    let mut n = hi;
    while swapped {
        swapped = false;
        for i in lo + 1..(n + 1) {
            let (prev, current) = {
                let v = locked_v.read().unwrap();
                (v[i - 1], v[i])
            };
            if prev > current {
                swap(Arc::clone(&locked_v), i - 1, i);
                swapped = true;
            }
        }
        n -= 1;
    }
}

fn quicksort<T: Ord + Copy + Debug + Send + Sync + 'static>(
    v: Arc<RwLock<Vec<T>>>,
    lo: usize,
    hi: usize,
) {
    //println!("quicksort({:?})", v);
    if lo < hi {
        if hi - lo < 20 {
            bubblesort(v, lo, hi);
        } else {
            let mut handles = vec![];
            let v_clone0 = Arc::clone(&v);
            let p = partition(v_clone0, lo, hi);
            //println!("partitionned: ({:?}) p: {:?}", v, p);

            if p > 0 {
                let v_clone = Arc::clone(&v);
                let handle = thread::spawn(move || quicksort(v_clone, lo, p - 1));
                handles.push(handle);

                let v_clone = Arc::clone(&v);
                let handle = thread::spawn(move || quicksort(v_clone, p, hi));
                handles.push(handle);
            } else {
                let v_clone = Arc::clone(&v);
                let handle = thread::spawn(move || quicksort(v_clone, p, hi));
                handles.push(handle);
            }
            for handle in handles {
                handle.join().unwrap();
            }
        }
    }
}

fn multithread_quicksort_test() {
    println!("\n--- multithreaded quicksort ---\n");

    let length = 1000;
    let locked_unsorted: Arc<RwLock<Vec<u16>>> = Arc::new(RwLock::new(vec![]));
    {
        let mut unsorted = locked_unsorted.write().unwrap();
        for _ in 0..length {
            unsorted.push(rand::random());
        }
        //println!("unsorted: {:?}", *unsorted);
    }

    quicksort(Arc::clone(&locked_unsorted), 0, length - 1);
    //bubblesort(Arc::clone(&locked_unsorted), 0, length - 1);

    let sorted = locked_unsorted.read().unwrap();
    let mut is_sorted = true;
    for i in 1..(sorted.len()) {
        if sorted[i - 1] > sorted[i] {
            is_sorted = false;
            break;
        }
    }
    //println!("sorted: {:?} is_sorted: {:?}", *sorted, is_sorted);
    println!("is_sorted: {:?}", is_sorted);
}
