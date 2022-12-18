use std::io::Write;
use termcolor as tc;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use dialoguer as dg;
use indicatif::ProgressBar;
use crossterm as ct;

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

    let dur = std::time::SystemTime::elapsed(&std::time::UNIX_EPOCH).unwrap();
    println!("dur: {:?}", &dur);
    let mut begin_sec = interval - dur.as_secs() % interval;
    let begin_nano = 1_000_000_000 - dur.as_nanos() % 1_000_000_000;

    println!("begin_sec: {}", &begin_sec);
    println!("begin_nano: {}", &begin_nano);

    let bar = ProgressBar::new(interval);
    std::thread::sleep(std::time::Duration::new(0, begin_nano as u32));

    // Why + 1 ? Is it because we want to show:
    // - The time remaining
    // - A ticking clock
    loop {
        for sec in (1 .. begin_sec + 1).rev() {
            bar.set_position(sec);
            std::thread::sleep(std::time::Duration::new(1, 0));
        }
        begin_sec = interval;
    }
}

pub fn example5() -> crossterm::Result<()> {
    ct::terminal::enable_raw_mode().unwrap();

    loop {
        if ct::event::poll(std::time::Duration::from_secs(10))? {
            match ct::event::read()? {
                ct::event::Event::Key(kv) => {
                    // kv.modifiers & kv.
                    if kv.code == ct::event::KeyCode::Char('q') {
                        println!("q pressed");
                        break;
                    } else if kv.code == ct::event::KeyCode::Char('c') && (kv.modifiers & ct::event::KeyModifiers::CONTROL) == ct::event::KeyModifiers::CONTROL {
                        println!("<C-c> pressed");
                        break;
                    } else {
                        println!("other key pressed");
                    }
                },
                _ => {
                    println!("other key event");
                },
            }
            println!("after match");
        }
        println!("after poll");
    }

    ct::terminal::disable_raw_mode().unwrap();

    Ok(())
}

