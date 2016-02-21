extern crate coroutine;

use std::thread;
use coroutine::asymmetric::Coroutine;

fn run_skynet(start: u64, total: u64) -> u64 {
    let coro = Coroutine::spawn(&skynet);
    let (x, _) = coro.resume_with((start, total)).unwrap().unwrap();
    coro.resume().unwrap();
    return x;
}

fn skynet(me: coroutine::asymmetric::CoroutineRef<(u64, u64)>) {
    let (my_number, remaining) = me.take_data().unwrap();

    if remaining==1 {
        me.yield_with((my_number, 0));
    } else {
        let coros: Vec<_> = (0..10).map(|_| Coroutine::spawn(&skynet)).collect();

        let mut res = 0u64;
        for i in 0..10 {
            let (x, _) = coros[i].resume_with((my_number + (i as u64)*remaining/10, remaining/10)).unwrap().unwrap();
            res += x;
            coros[i].resume().unwrap();
        }

        me.yield_with((res, 0));
    }
}

fn main() {
    let num = (0..10)
        .map(|i| thread::spawn(move || run_skynet(i*100000, 100000)))
        .map(|thread| thread.join().unwrap())
        .fold(0, std::ops::Add::add);

    println!("{}", num);
}
