use std::time::Instant;
use std::{mem, ptr};
use winapi::{
    shared::minwindef,
    um::{wingdi, winuser},
};
// use image;
use enigo::{self, MouseControllable};

fn screenshot(left:i32,top:i32,width:i32,height:i32) -> (u32, u32, Vec<u8>) {
    unsafe {
        let screen = winuser::GetDC(ptr::null_mut());
        let dc = wingdi::CreateCompatibleDC(screen);
        let bitmap = wingdi::CreateCompatibleBitmap(screen, width, height);
        let result = wingdi::SelectObject(dc, bitmap.cast());
        let result = wingdi::BitBlt(dc, 0, 0, width, height, screen, left, top, wingdi::SRCCOPY);
        let mut header = wingdi::BITMAPINFOHEADER {
            biSize: mem::size_of::<wingdi::BITMAPINFOHEADER>() as minwindef::DWORD,
            biWidth: width,
            biHeight: -height,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: wingdi::BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        };
        let buf_size = width as usize * height as usize * 4;
        let mut buffer: Vec<u8> = Vec::with_capacity(buf_size);
        let result = wingdi::GetDIBits(
            dc,
            bitmap,
            0,
            height as minwindef::UINT,
            buffer.as_mut_ptr().cast(),
            &mut header as *mut _ as wingdi::LPBITMAPINFO,
            wingdi::DIB_RGB_COLORS,
        );
        buffer.set_len(buf_size);
        return (width as u32, height as u32, buffer);
    }
}
fn main() {
    let mut controler = enigo::Enigo::new();
    loop {
        
        let (w,h,buff) = screenshot(395,395,282,1);
        let mut pixel_form:Vec<(u8,u8,u8,u8)> = vec![(0,0,0,0);buff.len()/4];
        for i in (0..buff.len()).step_by(4){
            let temp_tup = (buff[i],buff[i+1],buff[i+2],buff[i+3]);
            pixel_form[i/4] = temp_tup;
        }
        let (p1,p2,p3,p4) = (pixel_form[35],pixel_form[35+70],pixel_form[35+70*2],pixel_form[35+70*3]);
        if p1.0 == 0 && p1.1 == 0 && p1.2 == 0{
            
            controler.mouse_move_to(395+35, 395);
            controler.mouse_down(enigo::MouseButton::Left);
        }
        if p2.0 == 0 && p2.1 == 0 && p2.2 == 0{
            
            controler.mouse_move_to(395+35+70, 395);
            controler.mouse_down(enigo::MouseButton::Left);
        }
        if p3.0 == 0 && p3.1 == 0 && p3.2 == 0{
            
            controler.mouse_move_to(395+35+70*2, 395);
            controler.mouse_down(enigo::MouseButton::Left);
        }
        if p4.0 == 0 && p4.1 == 0 && p4.2 == 0{
            
            controler.mouse_move_to(395+35+70*3, 395);
            controler.mouse_down(enigo::MouseButton::Left);
        }
        // break;
    }
}