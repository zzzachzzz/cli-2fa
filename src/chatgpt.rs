use crossterm::{
    cursor::{Hide, Show},
    cursor,
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
    // Result,
};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::{self, stdout, Write};
use std::thread;
use std::time::{Duration, Instant};

pub fn the_main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize terminal and hide cursor
    let mut stdout = stdout();
    execute!(stdout, Hide)?;
    terminal::enable_raw_mode()?;

    // Set up the progress bar
    let pb = ProgressBar::new(10);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{wide_bar} {pos}/{len} seconds")?
            .progress_chars("=> "),
    );

    // Initial number
    let mut number = 42;
    let mut countdown = 10;

    loop {
        // Update the number
        let current_number = if countdown == 0 {
            number = 24;
            number
        } else {
            number
        };

        // Render UI
        render(number, countdown)?;

        // Countdown loop
        let start_time = Instant::now();
        while start_time.elapsed() < Duration::from_secs(10) {
            // Update the progress bar
            pb.set_position((10 - countdown) as u64);

            // Handle key events (e.g., to exit the loop)
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key_event) = event::read()? {
                    if key_event.code == KeyCode::Char('q') {
                        cleanup()?;
                        return Ok(());
                    }
                }
            }

            // Sleep for a short duration
            thread::sleep(Duration::from_millis(100));
        }

        // Decrease the countdown
        countdown -= 1;
    }
}

fn render(number: u32, countdown: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut stdout = stdout();

    // Move the cursor to the top-left corner
    execute!(stdout, cursor::MoveTo(0, 0))?;

    // Clear the terminal
    execute!(stdout, Clear(ClearType::All))?;

    // Print the number
    execute!(
        stdout,
        SetForegroundColor(Color::Green),
        Print(format!("Number: {}", number)),
        ResetColor,
        cursor::MoveToNextLine(1),
    )?;

    // Print the countdown progress bar
    execute!(stdout, SetBackgroundColor(Color::Blue), Print(" "), ResetColor)?;
    execute!(stdout, cursor::MoveToNextLine(1), Clear(ClearType::CurrentLine))?; // NOTE New line here
    let _ = stdout.flush(); // Flush the output to ensure the progress bar is displayed correctly

    // Show the progress bar
    execute!(
        stdout,
        cursor::MoveToNextLine(1),
        Show,
        SetBackgroundColor(Color::Blue)
    )?;
    let _ = stdout.flush(); // Flush the output to ensure the progress bar is displayed correctly

    // Hide the cursor
    execute!(stdout, Hide)?;

    Ok(())
}

fn cleanup() -> Result<(), Box<dyn std::error::Error>> {
    // Restore cursor visibility and disable raw mode
    let mut stdout = stdout();
    execute!(stdout, Show)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
