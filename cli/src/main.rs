use chrono::Datelike; // For extracting month and day from a date
use chrono::Local; // For getting the local date
use clap::Parser; // For parsing command-line arguments // For getting today's date

// Define the structure for CLI arguments
#[derive(Parser)]
#[command(name = "saints-cli")]
#[command(about = "Get the saint of the day", long_about = None)]
struct Cli {
    /// Date in format YYYY-MM-DD (optional)
    #[arg(short, long)]
    date: Option<String>,

    /// Path to the saints JSON file
    #[arg(long, default_value = "saints.json")]
    data: String,
}

fn main() {
    // Parse command-line arguments into the Cli struct
    let args = Cli::parse();

    // Load the list of saints from the JSON file
    let saints = saintlib::load_saints_from_file(&args.data);

    // Determine which date to use
    let (month, day) = if let Some(date_str) = args.date {
        let date = chrono::NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
            .expect("Date must be in YYYY-MM-DD format");
        (date.month0() + 1, date.day())
    } else {
        let today = Local::now().date_naive();
        (today.month0() + 1, today.day())
    };

    // Find all saints for the chosen date
    let found = saintlib::saints_for_date(&saints, month, day);

    // Print the result(s)
    if found.is_empty() {
        println!("No saint found for this date.");
    } else {
        for saint in found {
            println!(
                "Saint: {}\nDescription: {}\n",
                saint.name, saint.description
            );
        }
    }
}
