extern crate stb_rust;

use std::env;
use std::time::{Duration, Instant};
use std::fs::File;
use std::io::prelude::*;
use stb_rust::stb_image;
use stb_rust::c_runtime;

#[derive(Clone, Copy)]
struct internal_struct
{
    a:i32,
    b:i32
}

#[derive(Clone, Copy)]
struct my_struct
{
    data:[internal_struct;10],
    r:i32,
    f:Option<fn(i32) -> i32>
}

fn fff(v: i32) -> i32
{
    return v * v;
}

unsafe fn test(ms:*mut my_struct)
{
    let mut t: my_struct;

    t = (*ms);

    (*ms).f = Some(fff);
    (*ms).data[0].a = 15;
}

fn main() {
    /*    let mut ms: my_struct = unsafe { std::mem::uninitialized() };
        /*    let ms:*mut my_struct = std::ptr::null_mut();

        if ms == std::ptr::null_mut()
        {

        }*/

        let mut d:Vec<i32> = vec![1, 2, 3, 4, 5];
        let data:*mut i32 = d.as_mut_ptr();

        unsafe {
            let mut v =(data.offset(2)) as usize - (data.offset(1)) as usize;

            *(data.offset(3)) = 10;

            println!("OUTPUT: {}", v);
        }

        unsafe {
            test(&mut ms);

            if let Some(f) = ms.f
            {

                ms.data[0].a = (ms.f.unwrap())(ms.data[0].a);
            }

            println!("OUTPUT: {}", ms.data[0].a);
        }*/

    // Load file into memory
    let mut f = File::open("D:\\Temp\\map4.png").expect("file not found");

    let mut contents = vec![];
    f.read_to_end(&mut contents);

    let mut x:i32 = 0;
    let mut y:i32 = 0;
    let mut comp:i32 = 0;

    let start = Instant::now();
    unsafe {
        /*        stb_image::stbi_info_from_memory(contents.as_mut_ptr(), contents.len() as i32,
                                                 &mut x, &mut y, &mut comp);*/
        for i in 0..100 {
            let img = stb_image::stbi_load_from_memory(contents.as_mut_ptr(), contents.len() as i32,
                                             &mut x, &mut y, &mut comp, stb_image::STBI_rgb_alpha);
            c_runtime::free(img);
            println!("Try {}", i);
        }
    }

    let elapsed = start.elapsed();
    println!("X: {}, Y: {}, COMP: {}, ELAPSED MS: {}", x, y, comp,
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
}
