use evdev::{Device, InputEventKind, Key};
use serde::Serialize;
use serde_json::{json, Value};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Runtime, command};

#[derive(Debug, Clone, Serialize)]
pub enum DeviceEventKind {
    MousePress,
    MouseRelease,
    MouseMove,
    KeyboardPress,
    KeyboardRelease,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeviceEvent {
    kind: DeviceEventKind,
    value: Value,
}

static IS_LISTENING: AtomicBool = AtomicBool::new(false);

/// Converte KEY_A → KeyA, KEY_LEFTCTRL → ControlLeft, etc.
/// Formato compatível com o que o frontend / modelos esperam.
fn key_to_frontend(key: Key) -> String {
    match key {
        // Letras
        Key::KEY_A => "KeyA".into(),
        Key::KEY_B => "KeyB".into(),
        Key::KEY_C => "KeyC".into(),
        Key::KEY_D => "KeyD".into(),
        Key::KEY_E => "KeyE".into(),
        Key::KEY_F => "KeyF".into(),
        Key::KEY_G => "KeyG".into(),
        Key::KEY_H => "KeyH".into(),
        Key::KEY_I => "KeyI".into(),
        Key::KEY_J => "KeyJ".into(),
        Key::KEY_K => "KeyK".into(),
        Key::KEY_L => "KeyL".into(),
        Key::KEY_M => "KeyM".into(),
        Key::KEY_N => "KeyN".into(),
        Key::KEY_O => "KeyO".into(),
        Key::KEY_P => "KeyP".into(),
        Key::KEY_Q => "KeyQ".into(),
        Key::KEY_R => "KeyR".into(),
        Key::KEY_S => "KeyS".into(),
        Key::KEY_T => "KeyT".into(),
        Key::KEY_U => "KeyU".into(),
        Key::KEY_V => "KeyV".into(),
        Key::KEY_W => "KeyW".into(),
        Key::KEY_X => "KeyX".into(),
        Key::KEY_Y => "KeyY".into(),
        Key::KEY_Z => "KeyZ".into(),

        // Números
        Key::KEY_1 => "Num1".into(),
        Key::KEY_2 => "Num2".into(),
        Key::KEY_3 => "Num3".into(),
        Key::KEY_4 => "Num4".into(),
        Key::KEY_5 => "Num5".into(),
        Key::KEY_6 => "Num6".into(),
        Key::KEY_7 => "Num7".into(),
        Key::KEY_8 => "Num8".into(),
        Key::KEY_9 => "Num9".into(),
        Key::KEY_0 => "Num0".into(),

        // Modificadores
        Key::KEY_LEFTCTRL => "ControlLeft".into(),
        Key::KEY_RIGHTCTRL => "ControlRight".into(),
        Key::KEY_LEFTSHIFT => "ShiftLeft".into(),
        Key::KEY_RIGHTSHIFT => "ShiftRight".into(),
        Key::KEY_LEFTALT => "Alt".into(),
        Key::KEY_RIGHTALT => "AltGr".into(),
        Key::KEY_LEFTMETA => "Meta".into(),
        Key::KEY_RIGHTMETA => "Meta".into(),

        // Especiais
        Key::KEY_SPACE => "Space".into(),
        Key::KEY_ENTER => "Return".into(),
        Key::KEY_ESC => "Escape".into(),
        Key::KEY_TAB => "Tab".into(),
        Key::KEY_BACKSPACE => "Backspace".into(),
        Key::KEY_CAPSLOCK => "CapsLock".into(),
        Key::KEY_DELETE => "Delete".into(),

        // Setas (nome do modelo, não ArrowLeft)
        Key::KEY_UP => "UpArrow".into(),
        Key::KEY_DOWN => "DownArrow".into(),
        Key::KEY_LEFT => "LeftArrow".into(),
        Key::KEY_RIGHT => "RightArrow".into(),

        // Pontuação
        Key::KEY_GRAVE => "BackQuote".into(),
        Key::KEY_SLASH => "Slash".into(),
        Key::KEY_BACKSLASH => "Slash".into(),
        Key::KEY_102ND => "Slash".into(), // tecla extra ABNT

        // Função
        Key::KEY_F1 | Key::KEY_F2 | Key::KEY_F3 | Key::KEY_F4 |
        Key::KEY_F5 | Key::KEY_F6 | Key::KEY_F7 | Key::KEY_F8 |
        Key::KEY_F9 | Key::KEY_F10 | Key::KEY_F11 | Key::KEY_F12 => "Fn".into(),

        // Mouse
        Key::BTN_LEFT => "Left".into(),
        Key::BTN_RIGHT => "Right".into(),
        Key::BTN_MIDDLE => "Middle".into(),

        other => {
            let s = format!("{:?}", other);
            s.strip_prefix("KEY_").unwrap_or(&s).to_string()
        }
    }
}

fn is_mouse_button(key: Key) -> bool {
    matches!(
        key,
        Key::BTN_LEFT
            | Key::BTN_RIGHT
            | Key::BTN_MIDDLE
            | Key::BTN_SIDE
            | Key::BTN_EXTRA
            | Key::BTN_FORWARD
            | Key::BTN_BACK
    )
}

#[command]
pub async fn start_device_listening<R: Runtime>(app_handle: AppHandle<R>) -> Result<(), String> {
    if IS_LISTENING.load(Ordering::SeqCst) {
        return Ok(());
    }

    IS_LISTENING.store(true, Ordering::SeqCst);

    // ============================================
    // Thread por dispositivo (não precisa nonblocking)
    // ============================================
    for (path, device) in evdev::enumerate() {
        let has_keys = device
            .supported_keys()
            .map(|k| k.iter().next().is_some())
            .unwrap_or(false);

        if !has_keys {
            continue;
        }

        let name = device.name().unwrap_or("unknown").to_string();
        println!("[device] Abrindo teclado/mouse: {} ({:?})", name, path);

        let app = app_handle.clone();

        thread::spawn(move || {
            let mut device = device;

            loop {
                if !IS_LISTENING.load(Ordering::SeqCst) {
                    break;
                }

                // fetch_events é bloqueante — ok porque cada device tem sua thread
                match device.fetch_events() {
                    Ok(events) => {
                        for event in events {
                            if let InputEventKind::Key(key) = event.kind() {
                                let value = event.value();
                                let pressed = value == 1 || value == 2;
                                let released = value == 0;

                                let frontend_key = key_to_frontend(key);

                                eprintln!(
                                    "[device] raw={:?} → frontend={} value={}",
                                    key, frontend_key, value
                                );

                                if is_mouse_button(key) {
                                    if pressed {
                                        let _ = app.emit(
                                            "device-changed",
                                            DeviceEvent {
                                                kind: DeviceEventKind::MousePress,
                                                value: json!(frontend_key),
                                            },
                                        );
                                    } else if released {
                                        let _ = app.emit(
                                            "device-changed",
                                            DeviceEvent {
                                                kind: DeviceEventKind::MouseRelease,
                                                value: json!(frontend_key),
                                            },
                                        );
                                    }
                                } else {
                                    if pressed {
                                        let _ = app.emit(
                                            "device-changed",
                                            DeviceEvent {
                                                kind: DeviceEventKind::KeyboardPress,
                                                value: json!(frontend_key),
                                            },
                                        );
                                    } else if released {
                                        let _ = app.emit(
                                            "device-changed",
                                            DeviceEvent {
                                                kind: DeviceEventKind::KeyboardRelease,
                                                value: json!(frontend_key),
                                            },
                                        );
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("[device] Erro em {}: {e}", name);
                        thread::sleep(Duration::from_millis(100));
                    }
                }
            }
        });
    }

    // ============================================
    // Posição do cursor (Hyprland)
    // ============================================
    let app_mouse = app_handle.clone();
    thread::spawn(move || {
        let mut last_x = -1.0;
        let mut last_y = -1.0;

        loop {
            if !IS_LISTENING.load(Ordering::SeqCst) {
                break;
            }

            if let Ok(output) = std::process::Command::new("hyprctl")
                .args(["cursorpos"])
                .output()
            {
                if output.status.success() {
                    let pos = String::from_utf8_lossy(&output.stdout);
                    let pos = pos.trim();

                    if let Some((x_str, y_str)) = pos.split_once(',') {
                        if let (Ok(x), Ok(y)) =
                            (x_str.trim().parse::<f64>(), y_str.trim().parse::<f64>())
                        {
                            if (x - last_x).abs() > 0.5 || (y - last_y).abs() > 0.5 {
                                last_x = x;
                                last_y = y;

                                let _ = app_mouse.emit(
                                    "device-changed",
                                    DeviceEvent {
                                        kind: DeviceEventKind::MouseMove,
                                        value: json!({ "x": x, "y": y }),
                                    },
                                );
                            }
                        }
                    }
                }
            }

            thread::sleep(Duration::from_millis(16));
        }
    });

    Ok(())
}