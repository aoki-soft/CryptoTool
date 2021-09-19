//! CLI引数を受け取るモジュール

// Cli ArgumentParser
use clap::*;
use log::debug;

/// # CLI引数を受け取る関数
pub fn accept_cli_arg() -> Option<String> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("input_file")
                .short("i")
                .long("input_file")
                .takes_value(true)
                .value_name("FILE"),
        )
        .get_matches();

    let arg_len = std::env::args().len();
    debug!("arg_len: {}", arg_len);
    if arg_len == 1 {
        return None;
    }

    let input_file_path = matches
        .value_of_lossy("input_file")
        .map(|file| file.to_string());

    input_file_path
}
