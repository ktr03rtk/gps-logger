use serde::{Deserialize, Serialize};
use std::{
    error,
    fs::OpenOptions,
    io::{self, prelude::*},
    net, str,
};

#[derive(Serialize, Debug)]
struct Query {
    class: String,
    enable: bool,
    json: bool,
}

#[derive(Serialize, Default, Deserialize, Debug)]
#[serde(default)]
struct TPV {
    class: String,
    device: String,
    status: u8,
    mode: Option<u8>,
    time: String,
    ept: f64,
    lat: f64,
    lon: f64,
    alt: f64,
    epx: f64,
    epy: f64,
    epv: f64,
    track: f64,
    speed: f64,
    climb: f64,
    eps: f64,
    epc: f64,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut stream = net::TcpStream::connect("127.0.0.1:2947")?;

    let conditions = Query {
        class: "WATCH".to_string(),
        enable: true,
        json: true,
    };

    let query = format!("?WATCH={}\n", serde_json::to_string(&conditions)?);
    stream.write_all(query.as_bytes())?;

    loop {
        let mut reader = io::BufReader::new(&stream);
        reader.fill_buf()?;

        let deserialized: Result<TPV, serde_json::Error> =
            serde_json::from_str(str::from_utf8(reader.buffer())?);

        // TODO: define other class type, SKY and so on, then deserialize and exclude them
        match deserialized {
            Ok(n) => match n.class.as_str() {
                "TPV" => {
                    let log = serde_json::to_string(&n)? + "\n";
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open("gps.log")?;

                    let mut f = io::BufWriter::new(file);
                    f.write_all(log.as_bytes())?;
                    f.flush()?;
                }
                _ => (),
            },
            Err(_err) => (),
        }
    }
}
