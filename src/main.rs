use axum::{routing::get, Router};
use tokio;
use tokio_cron_scheduler::{Job, JobScheduler, JobToRunAsync};

// mod cron;
// use cron::start;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let mut sched = JobScheduler::new();
    let job = Job::new("1/10 * * * * *", |_uuid, _lock| {
        println!("I get executed every 10 seconds!")
    });
    sched.add(job).await;

    // Schedule your tasks as before...

    // Start the scheduler
    tokio::spawn(sched.start());

    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // let _ = start();
    axum::serve(listener, app).await.unwrap();

    // Ensure the scheduler is gracefully stopped when the server exits
    scheduler_task.abort();
}

async fn my_scheduled_task() -> Result<(), Box<dyn std::error::Error>> {
    // Your task logic here
    println!("Task executed!");
    Ok(())
}
