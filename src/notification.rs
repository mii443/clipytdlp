use winrt_toast_reborn::{Toast, ToastManager};

pub fn notify_success(original_url: &str, download_url: &str) {
    let manager = ToastManager::new(ToastManager::POWERSHELL_AUM_ID);

    let mut toast = Toast::new();
    toast
        .text1("clipytdlp - URL変換完了")
        .text2(format!("{original_url}\n→"))
        .text3(download_url);

    if let Err(e) = manager.show(&toast) {
        eprintln!("Toast通知の表示に失敗: {e}");
    }
}
