use std::env;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "my-rust-app")]
struct ProgramOption {
    /// 表示対象のファイルを指定する
    #[structopt(parse(from_os_str))]
    file: PathBuf,

    /// テーマ名を指定する
    #[structopt(short, long = "theme")]
    theme_name: Option<String>,
}

fn parse_args() -> ProgramOption {
    ProgramOption::from_args()
}

fn main() {
    // let options = parse_args();
    // println!("options:{:?}", options);

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
