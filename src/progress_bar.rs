use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;

use indicatif::{ProgressBar, ProgressStyle};

pub fn get_progress_bar() -> ProgressBar {
    let progress_bar = ProgressBar::new_spinner();
    progress_bar.set_style(get_style());

    progress_bar
}

pub fn get_style() -> ProgressStyle {
    ProgressStyle::with_template("{spinner:.green} {wide_msg}")
        .unwrap()
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ")
}

pub fn get_progress_bar_handle(
    done: Arc<AtomicBool>,
    progress_bar_arc: Arc<Mutex<ProgressBar>>,
    msg: String,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn({
        move || {
            while !done.load(Ordering::SeqCst) {
                let progress_bar = progress_bar_arc.lock().unwrap();
                progress_bar.set_message(msg.clone());
                progress_bar.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            done.store(false, Ordering::SeqCst);
        }
    })
}

pub fn get_progress_bar_handle_with_rx(
    done: Arc<AtomicBool>,
    progress_bar_arc: Arc<Mutex<ProgressBar>>,
    fallback_msg: String,
    rx: std::sync::mpsc::Receiver<String>,
) -> std::thread::JoinHandle<()> {
    std::thread::spawn({
        move || {
            let mut msg: Option<String> = None;
            while !done.load(Ordering::SeqCst) {
                let progress_bar = progress_bar_arc.lock().unwrap();

                if let Ok(new_msg) = rx.try_recv() {
                    msg = Some(new_msg);
                }

                let progress_msg = &msg.as_ref().unwrap_or(&fallback_msg);

                progress_bar.set_message(progress_msg.to_string());
                progress_bar.inc(1);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            done.store(false, Ordering::SeqCst);
        }
    })
}
