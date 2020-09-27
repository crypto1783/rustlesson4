extern crate clap;
use chrono::prelude::*;

extern crate chrono;
use std::path::Path;
use std::process;
use std::io::{Read, BufRead};
use clap::{Arg, App, SubCommand};
use std::io;
use std::fs::{File};
//use std::time::{SystemTime, UNIX_EPOCH};



fn main() {
    let matches = App::new("mycat")
        .version("0.1")
        .author("xiajianle")
        .about("Learn use Rust clap")
        .arg(Arg::with_name("file")
            //.short("v")
            .empty_values(false)
            //.multiple(true)
            .help("verbosity level"))

        //子命令1：根据关键词 搜索指定文件内容 打印包包含关键词的行内容
        .subcommand(SubCommand::with_name("s")
            .about("does testing things")
            //.arg_from_usage("-l, --list 'lists test values'")
            .arg(
                Arg::with_name("filename")
                .help("print debug information verbosely"))
            .arg(
                Arg::with_name("keyword")
                .help("print debug information verbosely"))
             )
         //子命令2：查询日期时间
        .subcommand(SubCommand::with_name("time")
            .about("get now time")
            )
        .get_matches();

    if let Some(file) = matches.value_of("file") {
        if Path::new(&file).exists() {
            let mut f = File::open(file).expect("[mycat Error] File not found.");
            let mut data = String::new();
            f.read_to_string(&mut data).expect("[kt Error] Unable to read the file.");
            println!("{}", data);
        }
        else {
            eprintln!("[mycat Error] No such file or directory.");
            process::exit(1);
        }
    }

    match matches.subcommand(){
        ("s", Some(param)) => {
            let file = param.value_of("filename").unwrap_or("file");
            let keyword = param.value_of("keyword").unwrap_or("keyword");
            let filePath = Path::new(&file);
            let mut f = File::open(filePath).expect("[mycat Error] File not found.");
            let mut buf = io::BufReader::new(f);
            for line in io::BufRead::lines(buf) {
                let line = line.unwrap_or("".to_string());
                if line.contains(keyword) {
                    //所有权转移
                    printLine(line);
                }
            }
        }
        ("time", Some(param)) => {
            //param.value_of("filename").unwrap_or("file");
            let mut dt = Local::now();
            println!("{}", dt);

        }
        _ => {}
    }

}

fn printLine(line: String) {
    println!("{}", line);
}
