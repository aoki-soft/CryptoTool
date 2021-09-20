//! # コンテクストメニューを変更するモジュール
//! windowsの右クリックのメニューにコマンドを追加、削除します。
//! 注意 windows でしか使うことができません。

/// コンテクストメニューにコマンドを追加します。
pub fn set_to_context_menu(key_file_path: &str) -> Result<(), std::io::Error> {
    // 登録済みの設定を削除する
    remove_key_from_file_menu(r"CryptoTool")?;

    // レジストリにキーをセットする
    set_key_to_faile_menu(r"CryptoTool\shell\ChaCha20\Command")?;

    // サブコマンドを追加できるように値をセットする
    set_property_to_faile_menu(r"CryptoTool", r"'MUIVerb'", r"'CryptoTool'")?;
    set_property_to_faile_menu(r"CryptoTool", r"'SubCommands'", r"''")?;
    // コマンド名をセット
    set_property_to_faile_menu(r"CryptoTool\shell\ChaCha20", r"'(default)'", r"'ChaCha20'")?;
    // コマンドをセット
    set_property_to_faile_menu(
        r"CryptoTool\shell\ChaCha20\Command",
        r"'(default)'",
        &format!(
            "'\"{}\" -i \"%V\" -k \"{}\"'",
            std::env::current_exe().unwrap().display(),
            key_file_path
        ),
    )?;

    Ok(())
}

/// コンテクストメニューにキーを追加する
/// ファイルの右クリックメニューを追加する
/// 引数のパスのキーを追加します
/// 引数には`HKCU:SOFTWARE\Classes\Directory\Background\shell\`の後に付ける値を入力する
fn set_key_to_faile_menu(post_path: &str) -> Result<(), std::io::Error> {
    let pre_path = r"HKCU:SOFTWARE\Classes\*\shell\";
    let result_set_key = std::process::Command::new("powershell.exe")
        .args(&[
            r"New-Item",
            &format!("{}{}", pre_path, post_path),
            r"-Force",
        ])
        .output()?;
    if !result_set_key.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "レジストリの編集ができませんでした",
        ));
    };
    Ok(())
}

/// コンテクストメニューのアイテムを追加する
/// ファイルの右クリックメニューを追加する
fn set_property_to_faile_menu(
    post_path: &str,
    name: &str,
    value: &str,
) -> Result<(), std::io::Error> {
    let pre_path = r"HKCU:SOFTWARE\Classes\*\shell\";
    let result_set_key = std::process::Command::new("powershell.exe")
        .args(&[
            r"New-ItemProperty",
            r"-LiteralPath",
            &format!("{}{}", pre_path, post_path),
            r"-Name",
            name,
            r"-Value",
            value,
        ])
        .output()?;
    if !result_set_key.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "レジストリの編集ができませんでした",
        ));
    }
    Ok(())
}

/// コンテクストメニューのコマンドを削除します。
pub fn remove_from_context_menu() -> Result<(), std::io::Error> {
    remove_key_from_file_menu(r"CryptoTool")
}

/// コンテクストメニューのコマンドを削除
/// 引数のkeyを削除します
fn remove_key_from_file_menu(post_path: &str) -> Result<(), std::io::Error> {
    let pre_path = r"HKCU:SOFTWARE\Classes\*\shell\";
    let result_remove_key = std::process::Command::new("powershell.exe")
        .args(&[
            r"Remove-Item",
            r"-LiteralPath",
            &format!("{}{}", pre_path, post_path),
            r"-Recurse",
        ])
        .output()?;
    if !result_remove_key.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "レジストリの編集ができませんでした",
        ));
    }
    Ok(())
}