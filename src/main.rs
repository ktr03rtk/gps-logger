use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::{
    error, fs,
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
    let gps_addr = "127.0.0.1:2947";
    let file_dir = "/var/log/gps-logger/";
    let file_name = "gps.log";

    // Request to gpsd server
    let mut stream = net::TcpStream::connect(gps_addr)?;

    let request_query = Query {
        class: "WATCH".to_string(),
        enable: true,
        json: true,
    };

    let query = format!("?WATCH={}\n", serde_json::to_string(&request_query)?);
    stream.write_all(query.as_bytes())?;

    // Parse the response and save it to a file
    fs::create_dir_all(file_dir)?;
    let file_path = file_dir.to_string() + file_name;
    let mut buf = vec![];

    loop {
        let mut reader = io::BufReader::new(&stream);
        reader.read_until(b'\n', &mut buf)?;

        let deserialized: Result<TPV, serde_json::Error> =
            serde_json::from_str(str::from_utf8(&buf)?);

        match deserialized {
            Ok(res) => match res.class.clone().unwrap().as_str() {
                "TPV" => {
                    let log = serde_json::to_string(&res)? + "\n";
                    let file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&file_path)?;

                    let mut f = io::BufWriter::new(file);
                    f.write_all(log.as_bytes())?;
                    f.flush()?;
                }
                // Discard all classes except TPV
                _ => (),
            },
            Err(err) => println!("Unexpected error while deserialization, {:?}", err),
        }
        buf.clear();
    }
}
