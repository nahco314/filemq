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
    let start_time = std::time::Instant::now();

    for _ in 0..1000 {
        let mut reses = Vec::new();

        for i in 0..10 {
            let a = Req::Add(i, i);
            reses.push(write_message_req::<_, i32>(&PathBuf::from("./s/"), a).unwrap());
        }

        for _ in 0..10 {
            loop {
                if let Some((name, msg)) = read_earliest_req::<Req>(&PathBuf::from("./s/")).unwrap()
                {
                    write_message_res(&PathBuf::from("./s/"), &name, msg.handle()).unwrap();
                    break;
                }
            }
        }

        for res in reses {
            res.read().unwrap();
        }
    }

    println!("Time: {:?}", start_time.elapsed());
}
