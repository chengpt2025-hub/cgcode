//! cgcode 开机动画：时空穿梭（约 8.5 秒，带开机音乐，按任意键跳过）。
//!
//! 画面是预先渲染好的 192x108 帧序列（20 帧/秒），在终端里用上下半块字符
//! “▀”加 24 位真彩色显示，每个字符格显示上下两个像素。
//! 音乐是一段 WAV，Windows 上用系统自带的 PlaySound 异步播放。

use std::io::Read;
use std::io::Write;
use std::time::Duration;
use std::time::Instant;

use crossterm::event;
use crossterm::event::Event;
use crossterm::event::KeyEventKind;

const FRAMES_Z: &[u8] = include_bytes!("../assets/intro_frames.bin.z");
const SOUND_Z: &[u8] = include_bytes!("../assets/intro_sound.wav.z");

const FW: usize = 192;
const FH: usize = 108;
const FPS: u64 = 20;

fn inflate(data: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    flate2::read::ZlibDecoder::new(data).read_to_end(&mut out).ok()?;
    Some(out)
}

#[cfg(windows)]
mod sound {
    use windows_sys::Win32::Media::Audio::PlaySoundW;
    use windows_sys::Win32::Media::Audio::SND_ASYNC;
    use windows_sys::Win32::Media::Audio::SND_MEMORY;
    use windows_sys::Win32::Media::Audio::SND_NODEFAULT;

    pub fn play(wav: &'static [u8]) {
        // SAFETY: wav 是 'static 数据，播放期间一直有效。
        unsafe {
            PlaySoundW(wav.as_ptr().cast(), std::ptr::null_mut(), SND_MEMORY | SND_ASYNC | SND_NODEFAULT);
        }
    }

    pub fn stop() {
        // SAFETY: 传空指针表示停止当前播放。
        unsafe {
            PlaySoundW(std::ptr::null(), std::ptr::null_mut(), 0);
        }
    }
}

#[cfg(not(windows))]
mod sound {
    pub fn play(_wav: &'static [u8]) {}
    pub fn stop() {}
}

/// 终端是否大概率支持 24 位真彩色。
fn truecolor_ok() -> bool {
    if std::env::var_os("WT_SESSION").is_some() {
        return true;
    }
    if let Ok(v) = std::env::var("COLORTERM")
        && (v.contains("truecolor") || v.contains("24bit"))
    {
        return true;
    }
    if let Ok(v) = std::env::var("TERM_PROGRAM")
        && matches!(v.as_str(), "vscode" | "WezTerm" | "iTerm.app" | "Tabby" | "Hyper")
    {
        return true;
    }
    // Windows 10 1703 以后的控制台也支持真彩色。
    cfg!(windows)
}

struct Layout {
    ox: usize,
    oy: usize,
    cols: usize,
    rows: usize,
    // 每个输出像素对应的源像素范围（预先算好，避免每帧重复计算）
    xs: Vec<(usize, usize)>,
    ys: Vec<(usize, usize)>,
}

fn layout(term_cols: usize, term_rows: usize) -> Option<Layout> {
    if term_cols < 20 || term_rows < 8 {
        return None;
    }
    // 半块字符下像素约为正方形，按 16:9 等比铺满并居中。
    let mut pw = term_cols;
    let mut ph = pw * 9 / 16;
    if ph > term_rows * 2 {
        ph = term_rows * 2;
        pw = ph * 16 / 9;
    }
    ph -= ph % 2;
    let cols = pw.max(1);
    let rows = (ph / 2).max(1);
    let span = |n_out: usize, n_src: usize| -> Vec<(usize, usize)> {
        (0..n_out)
            .map(|i| {
                let a = i * n_src / n_out;
                let b = ((i + 1) * n_src / n_out).max(a + 1).min(n_src);
                (a, b)
            })
            .collect()
    };
    Some(Layout {
        ox: (term_cols - cols) / 2,
        oy: (term_rows - rows) / 2,
        cols,
        rows,
        xs: span(cols, FW),
        ys: span(rows * 2, FH),
    })
}

fn sample(frame: &[u8], xr: (usize, usize), yr: (usize, usize)) -> (u8, u8, u8) {
    let (mut r, mut g, mut b, mut n) = (0u32, 0u32, 0u32, 0u32);
    for y in yr.0..yr.1 {
        let row = y * FW * 3;
        for x in xr.0..xr.1 {
            let i = row + x * 3;
            r += u32::from(frame[i]);
            g += u32::from(frame[i + 1]);
            b += u32::from(frame[i + 2]);
            n += 1;
        }
    }
    let n = n.max(1);
    ((r / n) as u8, (g / n) as u8, (b / n) as u8)
}

fn draw(out: &mut String, frame: &[u8], l: &Layout) {
    use std::fmt::Write as _;
    out.clear();
    for row in 0..l.rows {
        let _ = write!(out, "\x1b[{};{}H", l.oy + row + 1, l.ox + 1);
        let mut last_fg = None;
        let mut last_bg = None;
        for col in 0..l.cols {
            let top = sample(frame, l.xs[col], l.ys[row * 2]);
            let bot = sample(frame, l.xs[col], l.ys[row * 2 + 1]);
            if last_fg != Some(top) {
                let _ = write!(out, "\x1b[38;2;{};{};{}m", top.0, top.1, top.2);
                last_fg = Some(top);
            }
            if last_bg != Some(bot) {
                let _ = write!(out, "\x1b[48;2;{};{};{}m", bot.0, bot.1, bot.2);
                last_bg = Some(bot);
            }
            out.push('▀');
        }
        out.push_str("\x1b[0m");
    }
}

/// 播放开机动画。`with_sound` 为 false 时静音。任意键跳过。
pub fn play(with_sound: bool) {
    if !truecolor_ok() {
        return;
    }
    #[cfg(windows)]
    if !crossterm::ansi_support::supports_ansi() {
        return;
    }
    let Some(frames) = inflate(FRAMES_Z) else {
        return;
    };
    let frame_len = FW * FH * 3;
    let count = frames.len() / frame_len;
    if count == 0 {
        return;
    }
    let Ok((tc, tr)) = crossterm::terminal::size() else {
        return;
    };
    let Some(mut l) = layout(usize::from(tc), usize::from(tr)) else {
        return;
    };

    let mut stdout = std::io::stdout();
    if crossterm::terminal::enable_raw_mode().is_err() {
        return;
    }
    let _ = write!(stdout, "\x1b[?1049h\x1b[?25l\x1b[48;2;0;0;0m\x1b[2J");
    let _ = stdout.flush();

    if with_sound && let Some(wav) = inflate(SOUND_Z) {
        sound::play(Box::leak(wav.into_boxed_slice()));
    }

    let start = Instant::now();
    let frame_time = Duration::from_millis(1000 / FPS);
    let mut buf = String::with_capacity(l.cols * l.rows * 40);
    let mut shown = usize::MAX;
    loop {
        let idx = (start.elapsed().as_millis() / frame_time.as_millis()) as usize;
        if idx >= count {
            break;
        }
        if idx != shown {
            draw(&mut buf, &frames[idx * frame_len..(idx + 1) * frame_len], &l);
            let _ = stdout.write_all(buf.as_bytes());
            let _ = stdout.flush();
            shown = idx;
        }
        let next = start + frame_time * (idx as u32 + 1);
        let wait = next.saturating_duration_since(Instant::now());
        if event::poll(wait).unwrap_or(false) {
            match event::read() {
                Ok(Event::Key(k)) if k.kind == KeyEventKind::Press => break,
                Ok(Event::Resize(c, r)) => {
                    if let Some(nl) = layout(usize::from(c), usize::from(r)) {
                        l = nl;
                        let _ = write!(stdout, "\x1b[48;2;0;0;0m\x1b[2J");
                        shown = usize::MAX;
                    }
                }
                _ => {}
            }
        }
    }

    sound::stop();
    let _ = write!(stdout, "\x1b[0m\x1b[2J\x1b[?25h\x1b[?1049l");
    let _ = stdout.flush();
    let _ = crossterm::terminal::disable_raw_mode();
}
