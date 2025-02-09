use std::error::Error;
use std::fs::File;
use std::path::Path;

fn read_csv<P: AsRef<Path>>(filename: P) -> Result<(), Box<dyn Error>> {
    use std::time::Instant;
    let now = Instant::now();
    let file = File::open(filename)?;
    let mut rdr = csv::Reader::from_reader(file);
    println!("Now: {:.2?}", now);

    for result in rdr.records() {
        let record = result?;
        // println!("{:?}", record);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let filename = "/home/sourav/steam_reviews.csv";
    read_csv(filename)
}
