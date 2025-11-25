use anyhow::Result;
use driver::{OnError, MyDeviceDriver, DeviceFilter, LedState};
use clap::Parser;
use clap_repl::reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use clap_repl::{ClapEditor, ReadCommandOutput};
use console::style;
use tracing::{error, info};

#[derive(Parser)]
enum Command {
    Connect,
    Disconnect,
    Exit,

    LedOn,
    LedOff,
    // Add your commands here.
    // Arguments can be added like so: MyCommand { x: f32 },
}

async fn handle_command(driver: &mut MyDeviceDriver, cmd: Command) -> Result<()> {
    match cmd {
        Command::LedOn => {
            driver.root()
                .set_led_state(LedState::On)
                .await?;
        }
        Command::LedOff => {
            driver.root()
                .set_led_state(LedState::Off)
                .await?;
        }
        // Handle additional commands here

        // Already handled
        Command::Connect | Command::Disconnect | Command::Exit => {
        }
    }
    Ok(())
}

async fn connect_to_device() -> Result<MyDeviceDriver> {
    let filter = DeviceFilter::UsbVidPid {
        vid: 0xc0de,
        pid: 0xcafe,
    };
    let device = MyDeviceDriver::connect(filter, OnError::ExitImmediately).await?;
    Ok(device)
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let mut driver = None;
    let mut rl = setup_repl();

    loop {
        let should_exit = handle_user_command(&mut driver, &mut rl).await?;
        if should_exit {
            break;
        }
    }

    if let Some(d) = driver.as_mut() {
        d.disconnect_and_exit().await?;
    }
    Ok(())
}

async fn handle_user_command(driver: &mut Option<MyDeviceDriver>, rl: &mut ClapEditor<Command>) -> Result<bool> {
    match rl.read_command() {
        ReadCommandOutput::Command(c) => match c {
            Command::Connect => {
                match connect_to_device().await {
                    Ok(d) => {
                        info!("Connected!");
                        *driver = Some(d);
                    }
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
            Command::Disconnect => {
                if let Some(mut d) = driver.take() {
                    d.disconnect_and_exit().await?;
                    info!("Disconnected!");
                } else {
                    info!("Already disconnected");
                }
            }
            Command::Exit => return Ok(false),
            c => {
                let Some(d) = driver.as_mut() else {
                    println!("{}", style("No connection, connect first").yellow());
                    return Ok(true);
                };
                let r = handle_command(d, c).await;
                match r {
                    Ok(_) => println!("{}", style("ok").green()),
                    Err(e) => println!("{}", style(e).red()),
                }
            }
        },
        ReadCommandOutput::EmptyLine => (),
        ReadCommandOutput::ClapError(e) => {
            e.print()?;
        }
        ReadCommandOutput::ShlexError => {
            println!(
                "{} input was not valid and could not be processed",
                style("Error:").red().bold()
            );
        }
        ReadCommandOutput::ReedlineError(e) => {
            panic!("{e}");
        }
        ReadCommandOutput::CtrlC => return Ok(true),
        ReadCommandOutput::CtrlD => return Ok(false),
    }
    Ok(false)
}

fn setup_repl() -> ClapEditor<Command> {
    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("ww_template".to_owned()),
        ..DefaultPrompt::default()
    };
    let rl = ClapEditor::<Command>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            // Do custom things with `Reedline` instance here
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "repl_history.txt".into()).unwrap(),
            ))
        })
        .build();
    rl
}
