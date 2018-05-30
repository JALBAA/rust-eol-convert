extern crate clap;
use clap::{Arg, App, SubCommand};

use std::fs::*;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::SeekFrom;
use std::fs;
use std::fs::OpenOptions;
use clap::ArgMatches;



#[derive(Debug)]
struct Config<'a> {
    filename: Option<&'a str>,
    from:Option<&'a str>,
    to:Option<&'a str>,
}
impl<'a> Config<'a> {
    fn new () -> Config<'a> {
        Config {
            filename: None,
            from: None,
            to: None,
        }
    }
    fn add_value (&mut self, key: &str, value: &'a str) {
        if key == "file" {
            self.filename = Some(value);
        }
        if key == "from" {
            self.from = Some(value);
        }
        if key == "to" {
            self.to = Some(value);
        }
    }
    fn add (&mut self, matches:&'a ArgMatches<'a>, key: &str) -> &mut Self {
        if let Some(value) = matches.value_of(key) {
            self.add_value(key, value);
        }
        self
    }
}


fn main()  {
    let matches = App::new("crlf")
                    .arg(Arg::with_name("file")
                        .long("file")
                        .value_name("file")
                        .takes_value(true))
                    .arg(Arg::with_name("from")
                        .long("from")
                        .value_name("from")
                        .takes_value(true))
                    .arg(Arg::with_name("to")
                        .long("to")
                        .value_name("to")
                        .takes_value(true))
                    .get_matches();
    let mut conf = Config::new();
    conf.add(&matches, "file")
        .add(&matches, "from")
        .add(&matches, "to");
    let mut file = File::open(conf.filename.unwrap()).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s);
    let mut cur_file_eol = "crlf";
    {
        let mut chars = s.bytes();
        let mut last: u8 = 0;
        loop {
            if let Some(b) = chars.next() {
                if b == b'\r' {
                    let bb = chars.next().unwrap();
                    if bb == b'\n' {
                        break;
                        cur_file_eol = "crlf";
                    } else {
                        cur_file_eol = "cr";
                        break;
                    }
                } else if b == b'\n' {
                    if last != b'\r' {
                        cur_file_eol = "lf";
                        break;
                    }
                }
                last = b;
            } else {
                break;
            }
        }
    }
    if cur_file_eol == conf.to.unwrap() {
        println!("当前已经是{}类型了，不需要转换", cur_file_eol);
    } else {
        if cur_file_eol == "crlf" && conf.from.unwrap() == "crlf" && conf.to.unwrap() == "lf" {
            let mut f = File::create(conf.filename.unwrap()).unwrap();
            let mut writer = BufWriter::new(f);
            for (idx, c) in s.bytes().enumerate() {
                if c == b'\r' {
                    // ignore
                } else {
                    writer.write(&[c]);
                }
            }
        }
        if cur_file_eol == "lf" && conf.from.unwrap() == "lf" && conf.to.unwrap() == "crlf" {
            let mut f = File::create(conf.filename.unwrap()).unwrap();
            let mut writer = BufWriter::new(f);
            for (idx, c) in s.bytes().enumerate() {
                if c == b'\n' {
                    writer.write(b"\r\n");
                } else {
                    writer.write(&[c]);
                }
            }
        }
    }
}
