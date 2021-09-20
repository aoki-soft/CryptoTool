//! # 暗号化ツール
mod cli_arg_accepter;
mod context_menu;
mod crypto;
mod crypto_mode;
mod gui_mode;

/// ツールのエントリーポイント
fn main() {
    env_logger::init();

    let (input_file_path, key_file_path, mode) = cli_arg_accepter::accept_cli_arg();
    match mode {
        cli_arg_accepter::Mode::CliCrypto => {
            let _ = crypto_mode::crypto_mode(input_file_path, key_file_path);
        }
        cli_arg_accepter::Mode::Gui => {
            let _ = gui_mode::gui();
        }
    }
}
