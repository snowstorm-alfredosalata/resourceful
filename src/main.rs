use csv::Writer;
use std::fs::File;
use std::io::Write;
use std::time::Instant;

mod error;
mod machine;
mod resource;

use machine::*;

#[cfg(test)]
mod test;

fn main() -> std::io::Result<()> {
    let mut machine = Machine::default();
    let available_employees: Vec<usize> = vec![1, 2];
    
    let records_count = 50000;
    let benchmark = Instant::now();
    machine.generate_random_events(&available_employees, records_count);
    let elapsed = benchmark.elapsed().as_secs_f64();

    println!("Generato {records_count} eventi di produzione validi in {elapsed} secondi.");

    let mut events = machine.0.closed_events.clone();
    events.sort_by(|a, b| a.start.cmp(&b.start));

    // let mut second_machine = Machine::default();

    // let benchmark = Instant::now();
    // second_machine.import_and_validate_history(events);
    // let elapsed = benchmark.elapsed().as_secs_f64();

    // println!("Validato {records_count} eventi di produzione {elapsed} secondi.");

    let mut wtr = Writer::from_writer(File::create("output.csv")?);

    for record in events {
        wtr.serialize(record)?;
    }

    wtr.flush()?;

    Ok(())
}
