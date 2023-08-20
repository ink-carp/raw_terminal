#![allow(non_camel_case_types)]
#![allow(dead_code)]
type tcflag_t = u32;//unsigned int
type cc_t     = u8; //unsigned char
type c_int    = i32;
//define NCCS 32
const NCCS:usize = 32;
#[repr(C)]
struct Termios{
    c_iflag:tcflag_t,
    c_oflag:tcflag_t,
    c_cflag:tcflag_t,
    c_lflag:tcflag_t,
    c_cc:[cc_t;NCCS],
}
#[link(name = "c")]
extern "C"{
    fn ioctl(fildes: c_int,request:c_int,...) -> c_int;
    fn tcgetattr(fd:c_int,termios_p:*mut Termios) -> c_int;
    fn tcsetattr(fd:c_int, optional_actions:c_int,termios_p: * const Termios)->c_int;
}
/// Get the size of the current terminal in pixels
/// Ok((x,y)) if success
pub fn get_terminal_dimensions()-> Result<(u16,u16),&'static str>{

    //#define TIOCGWINSZ 0x5413
    const TIOCGWINSZ:c_int = 0x5413;

    // struct winsize {
    //     unsigned short ws_row;
    //     unsigned short ws_col;
    //     unsigned short ws_xpixel;   /* unused */
    //     unsigned short ws_ypixel;   /* unused */
    // };
    struct WinSize{
        ws_row:u16,
        ws_col:u16,
        ws_xpixel:u16,
        ws_ypixel:u16
    }

    // io stream change to fd:i32
    use std::{mem,os::unix::io::AsRawFd};
    let winsize = unsafe {
        let mut winsz = mem::MaybeUninit::<WinSize>::uninit();
        let ret = ioctl(std::io::stdin().as_raw_fd(), TIOCGWINSZ,winsz.as_mut_ptr());
        if ret == -1{
            return Err("Could not get terminal dimensions");
        }
        winsz.assume_init()
    };
    Ok((winsize.ws_row,winsize.ws_col))
}

/// Determines the terminal's handling of input characters
/// enable is default
/// disable is no characters are displayed and standard input is not processed
pub fn set_mode(enable:bool){
    //#define ECHO 0000010
    const ECHO:tcflag_t = 0x8;
    //#define ICANON 0000002
    const ICANON:tcflag_t = 0x2;
    //#define TCSANOW  0
    const TCSANOW:c_int = 0;
    use std::{mem,os::unix::io::AsRawFd};
    let fd = std::io::stdin().as_raw_fd();
    let mut termios = unsafe {
        let mut ter = mem::MaybeUninit::<Termios>::uninit();
        let ret = tcgetattr(fd, ter.as_mut_ptr());
        if ret != 0{
            panic!("Get the the terminal attributes failed!");
        }
        ter.assume_init()
    };
    if enable{
        termios.c_lflag |= ECHO | ICANON;
    }else {
        termios.c_lflag &= !( ECHO | ICANON);
    }
    unsafe{
        let ret = tcsetattr(fd, TCSANOW,&termios);
        if ret != 0{
            panic!("Sets the parameters of terminal failed!");
        }
    }
}

/// must used after set mode
/// clean the terminal
pub fn reset() {
    print!("\x1bc");
}
/// must used after set mode
/// true to hide the cursor,false to show the cursor
pub fn hide_cursor(enable:bool) {
    if enable{
        print!("\x1b\x5b?25l");
    }else {
        print!("\x1b[?25h");
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn terminal_size(){
        assert_ne!(get_terminal_dimensions(),Err("Could not get terminal dimensions"),"Get the terminal size success");
    }
}
