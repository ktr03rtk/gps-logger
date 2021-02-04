use chrono::{DateTime, Local};
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
    class: Option<String>,
    device: Option<String>,
    status: Option<u8>,
    mode: Option<u8>,
    #[serde(alias = "time")]
    timestamp: Option<DateTime<Local>>,
    lat: Option<f64>,
    lon: Option<f64>,
    alt: Option<f64>,
    climb: Option<f64>,
    epc: Option<f64>,
    eps: Option<f64>,
    ept: Option<f64>,
    epx: Option<f64>,
    epy: Option<f64>,
    epv: Option<f64>,
    track: Option<f64>,
    speed: Option<f64>,
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

        // println!("{}", str::from_utf8(reader.buffer())?);
        let deserialized: Result<TPV, serde_json::Error> =
            serde_json::from_str(str::from_utf8(reader.buffer())?);

        match deserialized {
            Ok(n) => match n.class.clone().unwrap().as_str() {
                "TPV" => {
                    // println!("deserialize {:?}", n);
                    let log = serde_json::to_string(&n)? + "\n";
                    // println!("Serialize {:?}", log);
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
