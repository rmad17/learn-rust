use std::error::Error;
use std::fs::File;
use std::path::Path;

use axum::{routing::get, Router};
use tokio;
use tokio_cron_scheduler::{Job, JobScheduler, JobToRunAsync};


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


// mod cron;
// use cron::start;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let mut sched = JobScheduler::new().await?;

    // Add basic cron job
    sched
        .add(Job::new("1/10 * * * * *", |_uuid, _l| {
            println!("I run every 10 seconds");
        }))
        .await;

    // Add async job
    sched
        .add(Job::new_async("1/7 * * * * *", |uuid, mut l| {
            Box::pin(async move {
                println!("I run async every 7 seconds");

                // Query the next execution time for this job
                let next_tick = l.next_tick_for_job(uuid).await;
                match next_tick {
                    Ok(Some(ts)) => println!("Next time for 7s job is {:?}", ts),
                    _ => println!("Could not get next tick for 7s job"),
                }
            })
        }))
        .await;

    // Add one-shot job with given duration
    sched
        .add(Job::new_one_shot(Duration::from_secs(18), |_uuid, _l| {
            println!("I only run once");
        }))
        .await;

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // let _ = start();
    axum::serve(listener, app).await.unwrap();
}

  
fn main() -> Result<(), Box<dyn Error>> {
    let filename = "/home/sourav/steam_reviews.csv";
    read_csv(filename)
}
  
  async fn my_scheduled_task() -> Result<(), Box<dyn std::error::Error>> {
    // Your task logic here
    println!("Task executed!");
    Ok(())
}
