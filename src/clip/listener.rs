use std::{borrow::{Borrow, Cow}, hash::Hash, vec};

use arboard::{Clipboard, ImageData};

static MASK_BASE:u64 = 0x0000_0000_0000_00ff;
static MASK_6:u64 = MASK_BASE << 8;
static MASK_5:u64 = MASK_BASE << 16;
static MASK_4:u64 = MASK_BASE << 24;
static MASK_3:u64 = MASK_BASE << 32;
static MASK_2:u64 = MASK_BASE << 40;
static MASK_1:u64 = MASK_BASE << 48;
static MASK_0:u64 = MASK_BASE << 56;

static SHIFT_0: u8 = 56;
static SHIFT_1: u8 = 48;
static SHIFT_2: u8 = 40;
static SHIFT_3: u8 = 32;
static SHIFT_4: u8 = 24;
static SHIFT_5: u8 = 16;
static SHIFT_6: u8 = 8;


trait ClipDeser {
    fn serialize_text(&self, text: String) -> Vec<u8>;

    fn serialize_image(&self, img: &ImageData) -> Vec<u8>;

    fn deserialize_text(&self, bts: Vec<u8>) -> String;

    fn deserialize_image(&self, bts: Vec<u8>) -> ImageData;
}

struct DefaultDeser {}



pub struct ClipListener {
    stopped: bool,
    deser: dyn ClipDeser
}


impl ClipListener {

    /// watch for clipboard change
    /// block until stop is called
    pub fn watch(&self, onText: impl Fn(Vec<u8>), onImg: impl Fn(Vec<u8>)) {
        let mut c = Clipboard::new().unwrap();
        while !self.stopped {
            self.get_image_change().map(|img| {
                let bts: Vec<u8> = self.deser.serialize_image(&img);
                onImg(bts);
            });
            self.get_text_change().map(|txt| {
                let bts = self.deser.serialize_text(txt);
                onText(bts);
            });
        }
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    fn get_text_change(&self) -> Option<String> {
        None
    }

    fn get_image_change(&self) -> Option<ImageData> {
        None
    }

    fn hash_txt(&self, text: String) -> u64 {
        0
    }

    fn hash_img(&self, image: &ImageData) -> u64 {
        0
    }
}

impl ClipDeser for DefaultDeser {

    fn serialize_text(&self, text: String) -> Vec<u8> {
        text.into_bytes()
    }

    /// |----------------------------|
    /// |height(8bytes)|width(8bytes)|
    /// |----------------------------|
    /// |                            |
    /// |       content in [u8]      |
    /// |----------------------------|
    fn serialize_image(&self, img: &ImageData) -> Vec<u8> {
        let height_u64 = img.height as u64;
        let width_u64 = img.width as u64;
        let mut header = u64_to_bytes(height_u64);
        header.append(&mut u64_to_bytes(width_u64));

        for ele in img.bytes.iter() {
            header.push(*ele);
        }

        header

    }
    
    fn deserialize_text(&self, bts: Vec<u8>) -> String {
        todo!()
    }
    
    fn deserialize_image(&self, bts: Vec<u8>) -> ImageData {
        let height_bytes = bts.as_slice()[0..8].to_vec();
        let width_bytes = bts.as_slice()[8..16].to_vec();
        let height = bytes_to_u64(height_bytes);
        let width = bytes_to_u64(width_bytes);
        ImageData{
            width: width as usize, height: height as usize, bytes: Cow::Owned(bts.as_slice()[16..].to_vec())
        }
    }

}

fn bytes_to_u64(bytes: Vec<u8>) -> u64 {
    ((bytes[0] as u64) << SHIFT_0) & MASK_0 |
    ((bytes[1] as u64) << SHIFT_1) & MASK_1|
    ((bytes[2] as u64) << SHIFT_2) & MASK_2 |
    ((bytes[3] as u64) << SHIFT_3) & MASK_3 |
    ((bytes[4] as u64) << SHIFT_4) & MASK_4|
    ((bytes[5] as u64) << SHIFT_5) & MASK_5 |
    ((bytes[6] as u64) << SHIFT_6) & MASK_6 |
    (bytes[7] as u64)
}

fn u64_to_bytes(num: u64) -> Vec<u8> {
    vec![
    ((num & MASK_0) >> SHIFT_0) as u8,
    ((num & MASK_1) >> SHIFT_1) as u8,
    ((num & MASK_2) >> SHIFT_2) as u8,
    ((num & MASK_3) >> SHIFT_3) as u8,
    ((num & MASK_4) >> SHIFT_4) as u8,
    ((num & MASK_5) >> SHIFT_5) as u8,
    ((num & MASK_6) >> SHIFT_6) as u8,
    (num) as u8,
    ]
}


#[test]
fn test_associtivity() {
    let a = (0b0110 & 0b0100) >> 2;
    assert_eq!(0b0001, a);
}

#[test]
fn test_bytes_to_u64(){
    let res = bytes_to_u64(vec![0x66, 0x88, 0x66, 0x88, 0x66, 0x88, 0x66, 0x88,]);
    assert_eq!(0x6688668866886688, res);

    let res_vec = u64_to_bytes(0x6688668866886688);
    assert_eq!(vec![0x66, 0x88, 0x66, 0x88, 0x66, 0x88, 0x66, 0x88,], res_vec)
}

#[test]
fn test_img_deser() {
    let img = ImageData{
        width: 32, height: 16, bytes: Cow::Owned(vec![1,1,2,2,3,3,4,4,5,5,6,6,7,7,8,8,9,9,0,0])
    };

    let deser = DefaultDeser{};
    let new_img = deser.deserialize_image(deser.serialize_image(&img));
    assert_eq!(16, new_img.height);
    assert_eq!(32, new_img.width);
    assert_eq!(img.bytes, new_img.bytes);
}