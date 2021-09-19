mod cli_arg_accepter;
mod crypto;

use log::debug;
use std::fs::File;
use std::io;
use std::os::windows::prelude::MetadataExt;
use std::str::FromStr;

fn main() {
    env_logger::init();

    let input_file_path = cli_arg_accepter::accept_cli_arg();
    let _ = crypto_mode(input_file_path);
}

fn crypto_mode(input_file_path: Option<String>) -> std::io::Result<()> {
    // ファイルバッファリーダーを取得する
    let (input_file_reader, input_file_size, input_file_path) = get_reader(input_file_path)?;

    // 書き込み先のバッファライターを取得する
    // 拡張子が.c20なら.c20を削除する.c20以外ならつける
    let mut output_file_path = input_file_path.clone();
    let extension = input_file_path.extension();
    debug!("input_file: {:?}", output_file_path);
    let output_file_path = match extension {
        None => output_file_path.with_extension("c20"),
        Some(extension) => {
            if extension == "c20" {
                debug!("extension: {:?} ==c20", extension);
                output_file_path.with_extension("")
            } else {
                debug!("extension: {:?} !=c20", extension);
                let new_extension = format!("{}.c20", extension.to_str().unwrap());
                debug!("new_extension: {}", new_extension);
                output_file_path.set_extension(new_extension);
                output_file_path
            }
        }
    };
    debug!("output_file_path: {:?}", output_file_path);
    let output_file_writer =
        std::io::BufWriter::new(match std::fs::File::create(output_file_path) {
            Ok(file) => file,
            Err(e) => {
                debug!("書き込み先のファイルを作成出来ませんでした。");
                debug!("{:?}", e);
                println!("書き込み先のファイルを作成出来ませんでした。");
                return Err(e);
            }
        });

    // プログレスバーのセットアップ
    let progress_bar = indicatif::ProgressBar::new(input_file_size);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})"),
    );
    // 1秒に4回プログレスバーを更新すると、少しパフォーマンスに影響出てきそう(2.5GHz 4core)
    progress_bar.set_draw_rate(4);

    // chacha20
    let key = b"an example very very secret key.";
    let nonce = b"secret nonce";

    // 暗号化
    crypto::crypto_chacha20(
        key,
        nonce,
        input_file_reader,
        output_file_writer,
        progress_bar,
    );

    Ok(())
}

fn get_reader(
    input_file_path: Option<String>,
) -> io::Result<(std::io::BufReader<File>, u64, std::path::PathBuf)> {
    // ファイルパスが引数に入っていることをチェックする
    let input_path = match input_file_path {
        Some(path) => path,
        None => {
            debug!("ファイルパスが一つも入力されていませんでした。");
            println!("ファイルパスを入力してください");
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "ファイルパスが一つも入力されていません。",
            ));
        }
    };
    // ファイルパスを特定する
    let input_path = match std::path::PathBuf::from_str(&input_path) {
        Ok(p) => p,
        Err(e) => {
            debug!("入力されたファイルパスが誤っています。");
            debug!("{:?}", e);
            println!("入力されたファイルパスが誤っています。");
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "入力されたファイルパスが誤っています。",
            ));
        }
    };
    // ファイルをオープンする
    let input_file = match std::fs::File::open(input_path.clone()) {
        Ok(f) => f,
        Err(e) => {
            debug!("ファイルにアクセスできませんでした。");
            debug!("{:?}", e);
            println!("ファイルにアクセスできませんでした。");
            return Err(e);
        }
    };
    // 読み込むファイルサイズを取得する
    let input_file_size = match input_file.metadata() {
        Ok(metadata) => metadata.file_size(),
        Err(e) => {
            debug!("ファイルのメタデータにアクセス出来ませんでした。");
            debug!("{:?}", e);
            println!("ファイルのメタデータにアクセス出来ませんでした。");
            return Err(e);
        }
    };
    // ファイルバッファリーダーを取得する
    let input_file_reader = std::io::BufReader::new(input_file);
    Ok((input_file_reader, input_file_size, input_path))
}
