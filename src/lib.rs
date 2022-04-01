use std::{time::Duration, thread};

pub fn thread_test(){
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

pub fn thread_test_fix(){

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

pub fn thread_test_with_value(){

    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}",v);
    });
    
    handle.join().unwrap();
}

use std::sync::mpsc;
pub fn channel_test(){
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    // when the chennel is closed the iterator will end
    for received in rx {
        println!("Got: {}", received);
    }
}

pub fn channel_test_with_multiple_producer(){

    let (tx, rx) = mpsc::channel();
    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // when the chennel is closed the iterator will end
    for received in rx {
        println!("Got: {}", received);
    }        
}

use std::sync::Mutex;
pub fn mutex_test_single_th(){
    
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("m = {:?}", m);
}

use std::sync::Arc;
pub fn mutex_test_multiple_th(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter =Arc::clone(&counter);
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num+=1;
        });
        handles.push(handle);
    }

    for handle in handles {
         handle.join().unwrap();
       }
       println!("Result: {}", *counter.lock().unwrap());
}