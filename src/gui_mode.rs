use super::context_menu;
use log::debug;
use native_windows_gui as nwg;
use std::rc::Rc;

pub fn gui() {
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

    // イベントをバインドさせてる。handlerイベントハンドラー(イベントを受け取ってくれるオブジェクト)
    let handler = nwg::full_bind_event_handler(&window.handle, move |evt, _evt_data, handle| {
        use nwg::Event as E;

        #[allow(clippy::single_match)]
        match evt {
            // ボタンが押されたイベントすべてを受け取る?
            E::OnButtonClick => {
                // コントロールハンドラー? ControlHandleっていうのはウェジットとかの部品っぽい
                if handle == button_set_context_menu {
                    debug!("右クリックメニューに追加します");
                    if context_menu::set_to_context_menu().is_ok() {
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
                    // ファイルダイアログ
                } else if handle == button_remove_context_menu {
                    debug!("右クリックメニューにから削除します");
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
                }
            }
            _ => {}
        }
    });

    // これは何をしているかわからない、、
    nwg::dispatch_thread_events();
    nwg::unbind_event_handler(&handler);
}
