use std::{thread, time};
use std::io::Write;

use merlin::key::Key;

fn main() {
    // キー入力を受け付ける
    let mut saved_termattr = libc::termios {
        c_iflag: 0,
        c_oflag: 0,
        c_cflag: 0,
        c_lflag: 0,
        c_cc: [0u8; 20],
        c_ispeed: 0,
        c_ospeed: 0,
    };
    unsafe {
        let ptr = &mut saved_termattr;
        libc::tcgetattr(0, ptr);
    }
    let mut termattr = saved_termattr;
    termattr.c_lflag = termattr.c_lflag & !(libc::ICANON | libc::ECHO);
    termattr.c_cc[libc::VMIN] = 1;
    termattr.c_cc[libc::VTIME] = 0;
    unsafe {
        libc::tcsetattr(0, libc::TCSANOW, &termattr);
    }
    unsafe {
        libc::fcntl(0, libc::F_SETFL, libc::O_NONBLOCK);
    }
    let mut buf: [libc::c_char; 1] = [0; 1];
    let ptr = &mut buf;
    let mut display = Key::None;
    let mut counter = 0;

    // メインループ
    loop {
        // 60fps
        thread::sleep(time::Duration::from_millis(16));

        // キー取得
        *ptr = [0];
        unsafe { libc::read(0, ptr.as_ptr() as *mut libc::c_void, 1) };
        let key = Key::new(*ptr);
        if key == Key::None {
            counter += 1;
        } else {
            counter = 0;
            display = Key::new(*ptr);
        }
        if counter == 18 {
            counter = 0;
            display = Key::None;
        }
        print!("\r\x1b[K");
        if display != Key::None {
            print!("{:?}", display);
        }
        std::io::stdout().flush().unwrap();

        // 終了条件（ `q` キーの押下）
        if key == Key::Exit {
            break;
        }

    }

    // キー入力の受付を終了
    // これを行わないとプログラム終了後の挙動に支障をきたすので必ず実行されるよう注意する
    unsafe {
        libc::tcsetattr(0, libc::TCSANOW, &saved_termattr);
    }
}
