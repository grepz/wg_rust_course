use clap::{Arg, Command};

//use std::env;
use std::error::Error;
//use std::ffi::OsString;
use std::fs::File;

const DEFAULT_CSV: &str = "coordinates.csv";
// const EARTH_R: u32 = 6371;

type Record = (f64, f64, u64);

fn radians(rec: &Record) -> (f64, f64) {
    (rec.0.to_radians(), rec.1.to_radians())
}

fn haversine_distance(start: &Record, end: &Record) -> f64 {
    let haversine_fn = |theta: f64| (1.0 - theta.cos()) / 2.0;

    let point1 = radians(start);
    let point2 = radians(end);

    let hav_delta_phi = haversine_fn(point2.0 - point1.0);
    let hav_delta_lambda =
        point1.0.cos() * point2.0.cos() * haversine_fn(point2.1 - point1.1);
    let total_delta = hav_delta_phi + hav_delta_lambda;

    (2.0 * 6371e3 * total_delta.sqrt().asin() * 1000.0).round() / 1000.0
}


#[derive(Debug)]
struct Coordinates {
   coordinates: Vec<Record>,
}

impl Coordinates {

    fn new() -> Coordinates {
        Coordinates {
            coordinates: Vec::with_capacity(1024),
        }
    }

    fn load_csv(file_path: &str, has_headers: bool) -> Result<Coordinates, Box<dyn Error>> {
        let file = File::open(file_path)?;
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(has_headers)
            .from_reader(Box::new(file));
        let mut coordinates =
            Coordinates::new();
        for result in reader.deserialize() {
            let record: Record = result?;
            coordinates.push(record);
        }
        Ok(coordinates)
    }

    fn push(&mut self, row: Record) {
        self.coordinates.push(row);
    }

    fn find_distance(
        &self, from: Option<&u64>, to: Option<&u64>
    ) -> Option<f64> {

        match (from, to) {
            (Some(from), Some(to)) => {
                let distance: (f64, (f64, f64, u64)) = self
                    .coordinates
                    .iter()
                    .fold((0.0, (-1.0, -1.0, 0)),
                          |(acc, last_rec), rec|
                            -> (f64, (f64, f64, u64)) {
                              if rec.2 >= *from && rec.2 <= *to {
                                  if last_rec == (-1.0, -1.0, 0) {
                                      (acc, *rec)
                                  } else {
                                      (acc + haversine_distance(&last_rec, rec), *rec)
                                  }
                              } else {
                                  (acc, last_rec)
                              }
                          });
                Some(distance.0)
            }
            _ => None
        }
    }

}

fn main() {
    let matches = Command::new("mnemonic")
        .about("First task for WG Rust course")
        .version("0.0.1")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .author("Stanislav M. Invakin <lessgrep@gmail.com>")
        .arg(
            Arg::new("csv")
                .short('c')
                .long("csv")
                .help("Loads CSV with coordinates/times")
                .takes_value(true)
        )
        .arg(
            Arg::new("from")
                .short('f')
                .long("from")
                .help("Set timestamp from")
                .value_parser(clap::value_parser!(u64))
                .action(clap::ArgAction::Set)
                .takes_value(true)
                .required(true)
        )
        .arg(
            Arg::new("to")
                .short('t')
                .long("to")
                .help("Set timestamp to")
                .value_parser(clap::value_parser!(u64))
                .takes_value(true)
                .required(true)
        ).get_matches();

    let file_path =
        if matches.is_present("csv") {
            matches.value_of("csv").unwrap()
        } else {
            DEFAULT_CSV
        };

    let coordinates =
        Coordinates::load_csv(file_path, false);

    let distance: Option<f64> = Coordinates::find_distance(
        &coordinates.unwrap(),
        matches.get_one::<u64>("from"),
        matches.get_one::<u64>("to")
    );

    match distance {
        Some(distance) => println!("Distance is: {} km's", distance / 1000.0),
        _ => println!("Something is wrong(CSV or arguments)")
    };
}
