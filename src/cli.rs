use std::path::{Path, PathBuf};

use windows::{
    core::{w, HSTRING},
    Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK}
};

use crate::installer::{self, Installer};

#[derive(Default)]
struct Args {
    command: Option<Command>,
    install_dir: Option<PathBuf>,
    target: Option<String>
}

enum Command {
    Install,
    Uninstall
}

#[inline]
fn require_next_arg(args: &mut std::env::Args) -> String {
    args.next().unwrap_or_else(|| std::process::exit(128))
}

impl Args {
    fn parse() -> Args {
        let mut args = Args::default();

        let mut iter = std::env::args();
        iter.next();

        loop {
            let Some(arg) = iter.next() else {
                break;
            };

            match arg.as_str() {
                "install" => args.command = Some(Command::Install),
                "uninstall" => args.command = Some(Command::Uninstall),

                "--install-dir" => args.install_dir = Some(require_next_arg(&mut iter).into()),
                "--target" => args.target = Some(require_next_arg(&mut iter)),

                _ => {
                    // Invalid argument
                    std::process::exit(128);
                }
            }
        }

        args
    }
}

pub fn run() -> Result<bool, installer::Error> {
    let mut args = Args::parse();
    
    if let Some(command) = args.command {
        if let Some(target) = &args.target {
            // Check if target is an absolute path;
            // If it is, set the install dir unconditionally so that it will be completely
            // overridden by the target later (without relying on install dir detection)
            let target_path = Path::new(target);
            if target_path.is_absolute() {
                // Doesn't matter which path it is, just use the target path
                args.install_dir = Some(target_path.into());
            }
        }

        let installer = Installer::custom(args.install_dir, args.target);
        let res = match command {
            Command::Install => installer.install(),
            Command::Uninstall => installer.uninstall()
        };
        if let Err(e) = res {
            unsafe { MessageBoxW(None, &HSTRING::from(e.to_string()), w!("Shinmy Installer"), MB_ICONERROR | MB_OK); }
            return Err(e);
        }

        Ok(true)
    }
    else {
        Ok(false)
    }
}