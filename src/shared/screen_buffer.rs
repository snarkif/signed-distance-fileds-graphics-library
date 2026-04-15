

const ROWS: usize = 64;
const COLUMNS: usize = 128;
const ASPECT_RATIO: usize = 2;
pub struct ScreenBuffer {
    pub(crate) screen: Box<Vec<bool>>,
}
impl ScreenBuffer {
    pub fn new() -> Self {

        Self {
            screen: Box::new(vec![false; ROWS * COLUMNS]),
        }
    }
    pub fn write_pixel(&mut self, x: usize, y: usize) {
        let page=y/8;
        self.screen[(page*1024)+(8*x)+(y-8*page)] = true;//updated
    }
    pub fn write_pixel_generic(arr:&mut [bool], x: usize, y: usize) {
        let page=y/8;
        arr[(page*1024)+(8*x)+(y-8*page)] = true;//updated
    }
    pub fn clear(&mut self ) {
        for i in 0..self.screen.len() {
            self.screen[i] = false;
        }
    }

    pub fn buf_to_window_vec(buffer: &[u8]) -> Box<Vec<u32>>{
        let mut display_buffer = Vec::<u32>::new();
        for page in 0..8 {
            for bit_row in 0..8 {
                for col in 0..128 {
                    let index = (page * 128) + col;
                    let byte = buffer[index];
                    if (byte >> bit_row) & 1 == 1 {
                        display_buffer.push(0xFFFFFFFF)
                    } else {
                        display_buffer.push(0x0);
                    }
                }

            }
        }
        Box::new(display_buffer)

    }

    pub fn pack_bits_to_bytes(&self) -> Vec<u8> {
        self.screen.chunks(8)
            .map(|chunk| {
                let mut byte = 0u8;
                for (i, &bit) in chunk.iter().enumerate() {
                    if bit {
                        byte |= 1 << i;
                    }
                }
                byte
            })
            .collect()
    }

}