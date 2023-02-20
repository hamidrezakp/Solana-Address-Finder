use std::{
    io::{BufWriter, Write},
    sync::mpsc,
};

use solana_sdk::{signature::Keypair, signer::Signer};

fn main() {
    let (trx, recv) = mpsc::channel::<Keypair>();

    let file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open("./Output")
        .unwrap();

    let mut writer = BufWriter::new(file);

    let writer_job = std::thread::spawn(move || writer_worker(&mut writer, recv));

    let mut generator_jobs = vec![];

    for _ in 0..5 {
        let trx = trx.clone();
        let handle = std::thread::spawn(move || generate_keypair_worker(trx));
        generator_jobs.push(handle);
    }

    writer_job.join().unwrap();
}

pub fn generate_keypair_worker(queue: mpsc::Sender<Keypair>) {
    loop {
        let keypair = solana_sdk::signature::Keypair::new();
        queue.send(keypair).unwrap()
    }
}

pub fn writer_worker<W: Write>(writer: &mut BufWriter<W>, queue: mpsc::Receiver<Keypair>) {
    while let Ok(keypair) = queue.recv() {
        let line = format!("{} {:?}\n", keypair.pubkey(), keypair.to_bytes());
        writer.write(line.as_bytes()).unwrap();
    }
}
