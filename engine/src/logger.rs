use std::{panic, io::Write, fs::File, sync::Mutex, env::current_dir};
use backtrace::SymbolName;
use chrono::{Local, Timelike};
use env_logger::{Builder, WriteStyle};
use log::{LevelFilter, Level};

lazy_static::lazy_static! {
    static ref FILE: Mutex<File> = {
        let path = directories::UserDirs::new().unwrap().document_dir().unwrap().join(env!("DOC_PATH")).join("trace.log");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        Mutex::new(std::fs::OpenOptions::new().create(true).write(true).truncate(true).open(path).unwrap())
    };
}

pub fn start() {
    let mut builder = Builder::new();
    builder
        .filter(None, LevelFilter::Trace)
        .filter(Some("wgpu_core"), LevelFilter::Info)
        .filter(Some("wgpu_core::device"), LevelFilter::Warn)
        .filter(Some("wgpu_hal"), LevelFilter::Info)
        .filter(Some("naga"), LevelFilter::Info)
        .format(|buf, record| {
            let level = record.level();
            let args = record.args();
            if level == Level::Trace {
                append_log(format!("{args}\n"));
                writeln!(buf, "\n\x1b[35m{args}\x1b[0m")
            } else {
                let now = Local::now();
                let timestamp = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
                let module = match record.module_path() { Some(v)=>v, None=>"" };
                let line = match record.line() { Some(v)=>v, None=>0 };
                let styled_level = buf.default_styled_level(level);
                append_log(format!("{timestamp} {styled_level} {module}:{line} {args}\r\n"));
                if level == Level::Error {
                    writeln!(buf, "\x1b[90m{timestamp} {styled_level} \x1b[96m{module}:{line}\x1b[0m {args}\n{}",
                        get_backtrace())
                }else {
                    writeln!(buf, "\x1b[90m{timestamp} {styled_level} \x1b[96m{module}:{line}\x1b[0m {args}")
                }
            }
        })
        .write_style(WriteStyle::Always)
        .init();
    panic::set_hook(Box::new(|panic_info| log::error!("{panic_info}")));
}

#[inline]
pub fn append_log(v: String) {
    FILE.lock().unwrap().write(v.as_bytes()).unwrap();
}

pub fn get_backtrace() -> String {
    let cur_dir = match current_dir() { Ok(v) => v, Err(e) => return e.to_string() };
    let mut res = String::new();
    for frame in backtrace::Backtrace::new().frames() {
        let symbol = &frame.symbols()[0];
        if let Some(path) = symbol.filename() {
            if !path.starts_with(&cur_dir) { continue }
        } else { continue };
        let name = symbol.name().unwrap_or(SymbolName::new(&[])).to_string();
        if name.starts_with("engine::logger") { continue; }
        res.push_str(&format!("{}:{}\n", name, symbol.lineno().unwrap_or_default()));
    }
    res
}