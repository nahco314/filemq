use crossbeam::channel::internal::SelectHandle;
use crossbeam::channel::unbounded;
use filemq::req_res::{find_earliest_req, read_earliest_req, write_message_req, write_message_res};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::thread;
use std::thread::sleep;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum Req {
    Add(i32, i32),
    Sub(i32, i32),
}

impl Req {
    fn handle(&self) -> i32 {
        match self {
            Req::Add(a, b) => a + b,
            Req::Sub(a, b) => a - b,
        }
    }
}

fn main() {
    let (tx, rx) = unbounded();

    let mut server_handles = Vec::new();
    let mut client_handles = Vec::new();
    for i in 0..100 {
        let rr = rx.clone();
        server_handles.push(thread::spawn(move || loop {
            if let Some((name, msg)) = read_earliest_req::<Req>(&PathBuf::from("./s/")).unwrap() {
                write_message_res(&PathBuf::from("./s/"), &name, msg.handle()).unwrap();
            }

            if rr.try_recv().is_ok() {
                break;
            }
        }));

        client_handles.push(thread::spawn(move || {
            let a = Req::Add(i, i);
            let b = Req::Sub(i, i);
            let res_a = write_message_req::<_, i32>(&PathBuf::from("./s/"), a).unwrap();
            let res_b = write_message_req::<_, i32>(&PathBuf::from("./s/"), b).unwrap();

            println!("{:?}, {:?}", res_a.read().unwrap(), a.handle());
            println!("{:?}, {:?}", res_b.read().unwrap(), b.handle());
        }))
    }

    sleep(std::time::Duration::from_secs(3));

    for _ in 0..100 {
        tx.send(()).unwrap();
    }
}
