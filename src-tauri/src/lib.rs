use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use std::sync::Mutex;
use tauri::{Manager, State};
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowLongPtrA, GetWindowTextA, GetWindowTextLengthW,
    SetForegroundWindow, SetWindowLongPtrA, GWL_EXSTYLE, WS_EX_NOACTIVATE,
};

macro_rules! press_me {
    ($enigo:expr, $key:expr, $action:expr) => {
        $enigo.key($key, $action).map_err(|e| e.to_string())
    };
}

struct MaDic {
    matcher: SkimMatcherV2,
    dictionary: Vec<String>,
}
impl MaDic {
    pub fn new() -> Self {
        let dictionary = include_str!("./dict/words.txt")
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();

        Self {
            matcher: SkimMatcherV2::default(),
            dictionary,
        }
    }
}

#[tauri::command]
fn spell_check(input: &str, limit: usize, state: State<Mutex<MaDic>>) -> Vec<String> {
    let madic = state.lock().unwrap();

    // Collect the results with scores
    let mut result: Vec<(i64, String)> = madic
        .dictionary
        .iter()
        .filter_map(|word| {
            madic.matcher.fuzzy_match(word, input).map(|score| {
                if score > 0 {
                    Some((score, word.clone()))
                } else {
                    None
                }
            })
        })
        .filter_map(|x| x)
        .collect();

    result.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let top_10: Vec<String> = result
        .into_iter()
        .take(limit)
        .map(|(_, word)| word)
        .collect();

    top_10
}

#[tauri::command]
fn send_key(key: &str, ctrl: bool, state: State<Mutex<Enigo>>) -> core::result::Result<(), String> {
    let mut enigo = state.lock().unwrap();
    if ctrl {
        press_me!(enigo, Key::Control, Press)?;
        if let Some(c) = key.chars().next() {
            press_me!(enigo, Key::Unicode(c), Click)?;
        }
        press_me!(enigo, Key::Control, Release)?;
    } else {
        enigo.text(key).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn send_key_replace(key: &str, state: State<Mutex<Enigo>>) -> core::result::Result<(), String> {
    let mut enigo = state.lock().unwrap();
    press_me!(enigo, Key::Control, Press)?;
    press_me!(enigo, Key::Shift, Press)?;
    press_me!(enigo, Key::LeftArrow, Click)?;
    press_me!(enigo, Key::Control, Release)?;
    press_me!(enigo, Key::Shift, Release)?;
    enigo.text(key).map_err(|e| e.to_string())
}

#[tauri::command]
fn send_key_alt(key: &str, state: State<Mutex<Enigo>>) -> core::result::Result<(), String> {
    let mut enigo = state.lock().unwrap();
    match key {
        "{tab}" => press_me!(enigo, Key::Tab, Click),
        "{space}" => press_me!(enigo, Key::Space, Click),
        "{bksp}" => press_me!(enigo, Key::Backspace, Click),
        "{enter}" => press_me!(enigo, Key::Return, Click),
        "{control}" => press_me!(enigo, Key::Control, Click),
        "{meta}" => press_me!(enigo, Key::Meta, Click),
        "{esc}" => press_me!(enigo, Key::Escape, Click),
        _ => Ok(()),
    }
}

#[tauri::command]
fn focus_window(hwnd: usize) {
    unsafe {
        let hwnd = HWND(hwnd as *mut _);
        _ = SetForegroundWindow(hwnd);
        println!("Window focused with handle: {:?}", hwnd);
    }
}

#[tauri::command]
fn get_hwid(state: State<Mutex<VBoard>>) -> usize {
    let vb = state.lock().unwrap();
    vb.hwnd
}
#[tauri::command]
fn enable_focus(hwnd: usize) {
    unsafe {
        let hwnd = HWND(hwnd as *mut _);
        let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrA(hwnd, GWL_EXSTYLE, ex_style & !(WS_EX_NOACTIVATE.0 as isize));
    }
}

#[tauri::command]
fn disable_focus(hwnd: usize) {
    unsafe {
        let hwnd = HWND(hwnd as *mut _);
        let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
        SetWindowLongPtrA(hwnd, GWL_EXSTYLE, ex_style | WS_EX_NOACTIVATE.0 as isize);
        println!("Focus disabled for window: {:?}", hwnd);
    }
}

#[derive(Default)]
struct VBoard {
    hwnd: usize,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(Mutex::new(VBoard::default()))
        .setup(|app| {
            let vboard = app.state::<Mutex<VBoard>>();
            unsafe {
                let hwnd = GetForegroundWindow();
                let mut vboard = vboard.lock().unwrap();
                vboard.hwnd = hwnd.0 as usize;
                let length = GetWindowTextLengthW(hwnd);
                let mut buffer = vec![0u8; length as usize + 1];
                GetWindowTextA(hwnd, &mut buffer);
                let window_title = String::from_utf8_lossy(&buffer);
                let ex_style = GetWindowLongPtrA(hwnd, GWL_EXSTYLE);
                SetWindowLongPtrA(hwnd, GWL_EXSTYLE, ex_style | WS_EX_NOACTIVATE.0 as isize);
                println!("Focus disabled for window: {:?}::{:?}", hwnd, window_title);
            }
            Ok(())
        })
        .manage(Mutex::new(MaDic::new()))
        .manage(Mutex::new(Enigo::new(&Settings::default()).unwrap()))
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_taurimation::init())
        .invoke_handler(tauri::generate_handler![
            spell_check,
            send_key,
            get_hwid,
            send_key_alt,
            send_key_replace,
            focus_window,
            disable_focus,
            enable_focus
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
