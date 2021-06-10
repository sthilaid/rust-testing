use core::fmt::Debug;
use std::cmp::*;
use std::sync::{Arc, RwLock};
use std::thread;

pub fn run() {
    println!("\n*****************************************************************");
    println!("concurency");
    println!("*****************************************************************");

    multithread_quicksort_test();
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

fn quicksort<T: Ord + Copy + Debug + Send + Sync + 'static>(
    v: Arc<RwLock<Vec<T>>>,
    lo: usize,
    hi: usize,
) {
    //println!("quicksort({:?})", v);
    if lo < hi {
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
            // let v_clone1 = Arc::clone(&v);
            // thread::spawn(move || quicksort(v_clone2, p, hi));
        }
        for handle in handles {
            handle.join().unwrap();
        }
    }
}

fn multithread_quicksort_test() {
    println!("\n--- multithreaded quicksort ---\n");

    let length = 100;
    let locked_unsorted: Arc<RwLock<Vec<u16>>> = Arc::new(RwLock::new(vec![]));
    {
        let mut unsorted = locked_unsorted.write().unwrap();
        for _ in 0..length {
            unsorted.push(rand::random());
        }
        println!("unsorted: {:?}", *unsorted);
    }

    quicksort(Arc::clone(&locked_unsorted), 0, length - 1);

    let sorted = locked_unsorted.read().unwrap();
    let mut is_sorted = true;
    for i in 1..(sorted.len()) {
        if sorted[i - 1] > sorted[i] {
            is_sorted = false;
            break;
        }
    }
    println!("sorted: {:?} is_sorted: {:?}", *sorted, is_sorted);
}
