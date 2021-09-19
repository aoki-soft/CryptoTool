//! # 暗号化ツール
mod cli_arg_accepter;
mod crypto;
mod crypto_mode;

/// ツールのエントリーポイント
fn main() {
    env_logger::init();

    let (input_file_path, key_file_path) = cli_arg_accepter::accept_cli_arg();
    let _ = crypto_mode::crypto_mode(input_file_path, key_file_path);
}
