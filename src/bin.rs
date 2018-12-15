use std::alloc::System;
use std::io;
use std::io::prelude::*;

use lscolors::{LsColors, Style};

#[global_allocator]
static A: System = System;

fn run() -> io::Result<()> {
    let ls_colors = LsColors::from_env().unwrap_or_default();

    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    let mut buf = vec![];
    while let Some(size) = stdin.lock().read_until(b'\n', &mut buf).ok() {
        if size == 0 {
            break;
        }

        let path_str = String::from_utf8_lossy(&buf[..(buf.len() - 1)]);

        for (component, style) in ls_colors.style_for_path_components(path_str.as_ref()) {
            let ansi_style = style.map(Style::to_ansi_term_style).unwrap_or_default();
            write!(stdout, "{}", ansi_style.paint(component.to_string_lossy()))?;
        }
        writeln!(stdout)?;

        buf.clear();
    }

    Ok(())
}

fn main() {
    run().ok();
}
