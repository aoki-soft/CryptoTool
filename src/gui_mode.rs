use super::context_menu;
use log::debug;
use native_windows_gui as nwg;
use std::rc::Rc;

pub fn gui() -> std::io::Result<()> {
    debug!("GUIモードで起動しました。");

    nwg::init().unwrap_or_else(|e| {
        debug!("Failed to init Native Windows GUI");
        debug!("{:?}", e);
        std::process::exit(0);
    });

    nwg::Font::set_global_family("Segoe UI").unwrap_or_else(|e| {
        debug!("Failed to set default font");
        debug!("{:?}", e);
        std::process::exit(0);
    });

    // ウェジットのオブジェクトを作成
    let mut window = Default::default();
    let mut button_set_context_menu = Default::default();
    let mut button_remove_context_menu = Default::default();
    let layout = Default::default();

    // ウェジットのオブジェクトのスタイルを変更する
    nwg::Window::builder()
        .size((600, 115))
        .position((600, 300))
        .title("Crypto Tool")
        .build(&mut window)
        .unwrap();

    nwg::Button::builder()
        .text("右クリックメニューにCrypto Toolを追加します。")
        .parent(&window)
        .build(&mut button_set_context_menu)
        .unwrap();

    nwg::Button::builder()
        .text("右クリックメニューからCrypto Toolを削除します。")
        .parent(&window)
        .build(&mut button_remove_context_menu)
        .unwrap();

    nwg::GridLayout::builder()
        .parent(&window)
        .spacing(1)
        .child_item(nwg::GridLayoutItem::new(
            &button_set_context_menu,
            0,
            0,
            1,
            1,
        ))
        .child_item(nwg::GridLayoutItem::new(
            &button_remove_context_menu,
            0,
            1,
            1,
            1,
        ))
        .build(&layout)
        .unwrap();

    let window = Rc::new(window);
    let events_window = window.clone();
    let button_set_context_menu = Rc::new(button_set_context_menu);
    let events_button_set_context_menu = button_set_context_menu.clone();
    let button_remove_context_menu = Rc::new(button_remove_context_menu);
    let events_button_remove_context_menu = button_remove_context_menu.clone();

    // イベントをバインドさせてる。handlerイベントハンドラー(イベントを受け取ってくれるオブジェクト)
    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        #[allow(clippy::single_match)]
        match evt {
            // ボタンが押されたイベントすべてを受け取る?
            E::OnButtonClick => {
                // コントロールハンドラー? ControlHandleっていうのはウェジットとかの部品っぽい
                if handle == events_button_set_context_menu.handle {
                    // ボタンを無効化する
                    events_button_set_context_menu.set_enabled(false);
                    events_button_remove_context_menu.set_enabled(false);

                    debug!("右クリックメニューに追加します");
                    let mut file_dialog = Default::default();
                    let _ = nwg::FileDialog::builder()
                        .title("使用する鍵ファイルを選択してください")
                        .action(nwg::FileDialogAction::Open)
                        .multiselect(false)
                        .build(&mut file_dialog);
                    file_dialog.run(Some(&events_window.handle));
                    let file_path = file_dialog.get_selected_item();
                    match  file_path {
                        Ok(file) => {
                            // コンテクストメニューにセットする
                            if context_menu::set_to_context_menu(file.to_str().unwrap()).is_ok() {
                                nwg::modal_info_message(
                                    &events_window.handle,
                                    "Digest Tool",
                                    "右クリックメニューに追加しました。",
                                );
                            } else {
                                nwg::modal_info_message(
                                    &events_window.handle,
                                    "Digest Tool",
                                    "右クリックメニューに追加できませんでした。",
                                );
                            }
                        },
                        Err(e) => {
                            debug!("{:?}", e);
                        }
                    }

                    // ボタンを有効化する
                    events_button_set_context_menu.set_enabled(true);
                    events_button_remove_context_menu.set_enabled(true);
                    
                } else if handle == button_remove_context_menu.handle {
                    debug!("右クリックメニューにから削除します");
                    
                    // ボタンを無効化する
                    events_button_set_context_menu.set_enabled(false);
                    events_button_remove_context_menu.set_enabled(false);

                    // 右クリックメニューから削除する
                    if context_menu::remove_from_context_menu().is_ok() {
                        nwg::modal_info_message(
                            &events_window.handle,
                            "Digest Tool",
                            "右クリックメニューから削除しました。",
                        );
                    } else {
                        nwg::modal_info_message(
                            &events_window.handle,
                            "Digest Tool",
                            "右クリックメニューから削除できませんでした。",
                        );
                    }
                    // ボタンを有効化する
                    events_button_set_context_menu.set_enabled(true);
                    events_button_remove_context_menu.set_enabled(true);
                }
            }
            _ => {}
        }
    });

    // これは何をしているかわからない、、
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
    Ok(())
}
