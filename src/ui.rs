use std::io::Write;
use std::time::Duration;
use termcolor as tc;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use dialoguer as dg;
// use indicatif::ProgressBar;
use crossterm as ct;
use crossterm::{
    event, terminal, style::{self, Stylize},
    QueueableCommand, ExecutableCommand
};
use crate::totp;

// Colors with termcolor
pub fn example1() -> Result<(), std::io::Error> {
    let mut stdout = tc::StandardStream::stdout(tc::ColorChoice::Always);

    let mut cs = ColorSpec::new();

    stdout.set_color(cs.set_fg(Some(Color::Blue)))?;

    writeln!(&mut stdout, "cock and balls!")?;

    stdout.reset()?;

    writeln!(&mut stdout, "except not")?;

    stdout.set_color(cs.set_bg(Some(Color::Red)).set_fg(Some(Color::Black)))?;

    writeln!(&mut stdout, "haha good one")?;

    Ok(())
}

pub fn example2() {
    let mut stdout = tc::StandardStream::stdout(tc::ColorChoice::Always);
    let mut cs = ColorSpec::new();
    stdout.set_color(cs.set_fg(Some(Color::Blue))).unwrap();

    let mut prompt = dg::Confirm::new();
    prompt
        .with_prompt("Cock?")
        .wait_for_newline(true);

    if prompt.interact().unwrap() {
        println!("Yes");
    } else {
        println!("No");
    }
}

pub fn example3() {
    let items = vec!["cock", "balls", "[other]"];
    let select = dg::Select::with_theme(&dg::theme::ColorfulTheme::default())
        .items(&items)
        .interact_opt()
        .unwrap();

    match select {
        Some(index) => println!("User selected item : {}", items[index]),
        None => println!("User did not select anything")
    }
}

pub fn example4() {
    let interval: u64 = 30;

    let mut stdout = std::io::stdout();

    let dur = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH).unwrap();
    println!("dur: {:?}", &dur);
    let mut begin_sec = interval - dur.as_secs() % interval;
    let begin_nano = 1_000_000_000 - dur.as_nanos() % 1_000_000_000;

    println!("begin_sec: {}", &begin_sec);
    println!("begin_nano: {}", &begin_nano);

    std::thread::sleep(Duration::new(0, begin_nano as u32));

    // let secret: &[u8] = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA====";
    // let x = totp::generate_totp(secret);

    stdout.execute(ct::terminal::Clear(ct::terminal::ClearType::All)).unwrap();
    let (cols, rows) = ct::terminal::size().unwrap();
    // stdout.execute(ct::cursor::MoveTo(cols - 1, rows - 1)).unwrap();

    // let bar = ProgressBar::new(interval);

    let mut i = 0;
    let duration_1_sec = Duration::new(1, 0);
    terminal::enable_raw_mode().unwrap();
    'infinite: loop {
        // stdout.queue(ct::cursor::MoveTo(0, 1)).unwrap();
        // stdout.queue(style::PrintStyledContent(format!("Output\t\t{}", &i).magenta())).unwrap();
        // stdout.queue(ct::cursor::MoveToRow(rows)).unwrap();
        // stdout.queue(ct::cursor::RestorePosition).unwrap();
        // stdout.flush().unwrap();

        for sec in (1 .. begin_sec + 1).rev() {
            // bar.set_position(sec);

            let progress_empty_cell = (interval - sec) as usize;
            let progress_filled_cell = sec as usize;
            // println!("progress_empty_cell {}:", progress_empty_cell);
            // println!("progress_filled_cell {}:", progress_filled_cell);


            let output = format!(
                "{}sec |{}|",
                &sec,
                (
                    &std::iter::repeat('█').take(progress_filled_cell)
                    .chain(std::iter::repeat(' ').take(progress_empty_cell))
                    .collect::<String>()
                )
            );
            stdout
                .queue(ct::terminal::Clear(ct::terminal::ClearType::CurrentLine)).unwrap()
                .queue(ct::cursor::MoveToColumn(0)).unwrap()
                .queue(style::PrintStyledContent(output.cyan())).unwrap()
                .queue(ct::cursor::Hide).unwrap();

            stdout.flush().unwrap();
            // Doubles as a sleep, during which we listen for a key event to quit
            if check_quit_keypress(duration_1_sec) { break 'infinite; }
        }
        begin_sec = interval;
        i += 1;
    }
    terminal::disable_raw_mode().unwrap();
    stdout.queue(ct::cursor::MoveToRow(0)).unwrap();
    // stdout.queue(ct::terminal::Clear(ct::terminal::ClearType::CurrentLine)).unwrap();
}

pub fn check_quit_keypress(duration: Duration) -> bool {
    if event::poll(duration).unwrap() {
        match ct::event::read().unwrap() {
            ct::event::Event::Key(ke) => {
                if ke.code == ct::event::KeyCode::Char('q') {
                    return true;
                } else if
                    ke.code == event::KeyCode::Char('c')
                    && (ke.modifiers & event::KeyModifiers::CONTROL) == event::KeyModifiers::CONTROL
                {
                    return true;
                }
            },
            _ => {},
        }
    }
    false
}

// Colors with crossterm
pub fn example5() -> Result<(), std::io::Error> {
    use crossterm::style::Stylize;
    use crossterm::style;

    println!("{}", "Bold".bold().cyan());
    println!("{}", "Underlined".underlined());
    println!("{}", "Negative".negative());

    Ok(())
}
