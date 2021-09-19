//! # 暗号化・復号モード
//! 暗号化・復号モードの処理のモジュール
//! ## 処理フローチャート
//! ![](../../../../document/crypto_mode.drawio.svg)

use super::crypto;
use log::debug;
use std::fs::File;
use std::io::{self, Read};
use std::os::windows::prelude::MetadataExt;
use std::str::FromStr;

/// # 暗号化・復号モード
pub fn crypto_mode(
    input_file_path: Option<String>,
    key_file_path: Option<String>,
) -> std::io::Result<()> {
    // ファイルバッファリーダーを取得する
    let (input_file_reader, input_file_size, input_file_path) = get_reader(input_file_path)?;

    // アウトプットファイルのパスを取得する
    let output_file_path = prepare_output_file_name(input_file_path);

    // chacha20の鍵
    let key = read_key(key_file_path)?;

    // バッファライターを取得する
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
    let progress_bar = prepare_progress_bar(input_file_size);

    let nonce = b"secret nonce";

    // 暗号化
    crypto::crypto_chacha20(
        &key,
        nonce,
        input_file_reader,
        output_file_writer,
        progress_bar,
    );

    Ok(())
}

/// # ファイルバッファリーダー取得
/// ファイル名のオプションを受け取って、バッファリーダーとファイルサイズ(byte)、ファイルパスを返します。
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

/// # 書き出し先ファイル名の取得
/// 拡張子が`.c20`だったら拡張子を削除して、
/// それ以外には`.c20`の拡張子を追加する
fn prepare_output_file_name(input_file_path: std::path::PathBuf) -> std::path::PathBuf {
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
    output_file_path
}

/// # プログレスバーのセットアップ
/// ファイルサイズ(byte)を受け取ってプログレスバーを出力します
fn prepare_progress_bar(input_file_size: u64) -> indicatif::ProgressBar {
    // プログレスバーのセットアップ
    let progress_bar = indicatif::ProgressBar::new(input_file_size);
    progress_bar.set_style(
        indicatif::ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{wide_bar}] {bytes}/{total_bytes} ({eta})"),
    );
    // 1秒に4回プログレスバーを更新すると、少しパフォーマンスに影響出てきそう(2.5GHz 4core)
    progress_bar.set_draw_rate(4);
    progress_bar
}

/// 鍵データの読み込み
fn read_key(key_file_path: Option<String>) -> io::Result<[u8; 32]> {
    // ファイルパスが引数に入っていることをチェックする
    let input_path = match key_file_path {
        Some(path) => path,
        None => {
            debug!("鍵ファイル名が入力されていませんでした。");
            println!("鍵ファイル名を入力してください");
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "鍵ファイル名が入力されていませんでした。",
            ));
        }
    };
    // ファイルパスを特定する
    let input_path = match std::path::PathBuf::from_str(&input_path) {
        Ok(p) => p,
        Err(e) => {
            debug!("鍵ファイルパスが誤っています。");
            debug!("{:?}", e);
            println!("入力された鍵ファイルパスが誤っています。");
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "入力された鍵ファイルパスが誤っています。",
            ));
        }
    };
    // ファイルをオープンする
    let mut input_file = match std::fs::File::open(input_path) {
        Ok(f) => f,
        Err(e) => {
            debug!("鍵ファイルにアクセスできませんでした。");
            debug!("{:?}", e);
            println!("鍵ファイルにアクセスできませんでした。");
            return Err(e);
        }
    };
    // 鍵ファイルサイズを取得する 鍵ファイルが32byte以外だった場合はErrを返す
    let file_size = match input_file.metadata() {
        Ok(metadata) => metadata.file_size(),
        Err(e) => {
            debug!("鍵ファイルのメタデータを取得出来ませんでした。");
            debug!("{:?}", e);
            println!("鍵ファイルのメタデータを取得出来ませんでした。");
            return Err(e);
        }
    };
    if file_size != 32 {
        debug!("鍵ファイルが32byteではありませんでした。");
        println!("鍵ファイルが32byteではありませんでした。");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "鍵ファイルのサイズが32byte以外のため不正です",
        ));
    };

    // 鍵ファイルを読み込む
    let mut key = [0; 32];
    let result = input_file.read(&mut key);

    let size = match result {
        Ok(size) => size,
        Err(e) => {
            debug!("鍵ファイルを読み込めませんでした。");
            debug!("{:?}", e);
            println!("鍵ファイルを読み込めませんでした");
            return Err(e);
        }
    };
    if size != 32 {
        debug!("読み取った鍵ファイルのサイズが32byte以外でした");
        println!("鍵ファイルを読み込めませんでした");
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "読み取った鍵ファイルのサイズが32byte以外でした",
        ));
    }

    Ok(key)
}
