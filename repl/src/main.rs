use std::time::Duration;
use anyhow::Result;
use driver::{OnError, MyDeviceDriver, DeviceFilter, LedState};
use clap::Parser;
use clap_repl::reedline::{DefaultPrompt, DefaultPromptSegment, FileBackedHistory};
use clap_repl::{ClapEditor, ReadCommandOutput};
use console::style;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "")] // This name will show up in clap's error messages, so it is important to set it to "".
enum Command {
    Connect,
    Disconnect,
    Exit,

    LedOn,
    LedOff,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    // let cli = Cli::parse();

    let mut driver = None;

    let prompt = DefaultPrompt {
        left_prompt: DefaultPromptSegment::Basic("ww_template".to_owned()),
        ..DefaultPrompt::default()
    };
    let mut rl = ClapEditor::<Command>::builder()
        .with_prompt(Box::new(prompt))
        .with_editor_hook(|reed| {
            // Do custom things with `Reedline` instance here
            reed.with_history(Box::new(
                FileBackedHistory::with_file(10000, "repl_history.txt".into()).unwrap(),
            ))
        })
        .build();
    loop {
        match rl.read_command() {
            ReadCommandOutput::Command(c) => match c {
                Command::Connect => {
                    let target = DeviceFilter::UsbVidPid {
                        vid: 0xc0de,
                        pid: 0xcafe,
                    };
                    match MyDeviceDriver::connect(target, OnError::ExitImmediately).await {
                        Ok(d) => {
                            info!("Connected!");
                            driver = Some(d);
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
                Command::Exit => break,
                c => {
                    let Some(d) = driver.as_mut() else {
                        println!("{}", style("No connection, connect first").yellow());
                        continue;
                    };
                    let r = match c {
                        Command::LedOn => d
                            .root()
                            .set_led_state(LedState::On)
                            .await,
                        Command::LedOff => d
                            .root()
                            .set_led_state(LedState::Off)
                            .await,
                        _ => unreachable!(),
                    };
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
            ReadCommandOutput::CtrlC => continue,
            ReadCommandOutput::CtrlD => break,
        }
    }

    if let Some(d) = driver.as_mut() {
        d.disconnect_and_exit().await?;
    }
    Ok(())
}
