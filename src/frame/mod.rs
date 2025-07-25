#[derive(Debug, Clone)]
pub struct YUVFrame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
    pub luminance_bytes: Vec<u8>,
    pub luminance_stride: i32,
    pub chrominance_bytes: Vec<u8>,
    pub chrominance_stride: i32,
}

#[derive(Debug, Clone)]
pub struct RGBFrame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct RGB8Frame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Clone)]
pub struct RGBxFrame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct XBGRFrame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BGRxFrame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BGRFrame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct BGRAFrame {
    pub display_time: u64,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum FrameType {
    #[default]
    YUVFrame,
    BGR0,
    RGB, // Prefer BGR0 because RGB is slower
    BGRAFrame,
}

#[derive(Debug, Clone)]
pub enum Frame {
    YUVFrame(YUVFrame),
    RGB(RGBFrame),
    RGBx(RGBxFrame),
    XBGR(XBGRFrame),
    BGRx(BGRxFrame),
    BGR0(BGRFrame),
    BGRA(BGRAFrame),
    None,
}

pub enum FrameData<'a> {
    NV12(&'a YUVFrame),
    BGR0(&'a [u8]),
}
impl Frame {
    pub fn to_rgb(&self) -> RGBFrame {
        match self {
            Frame::RGB(f) => f.clone(),

            // Frame::RGBx(f) => ,
            // Frame::XBGR(f) => ,
            // Frame::BGRx(f) => ,
            // Frame::BGR0(f) => ,
            Frame::BGRA(f) => RGBFrame {
                display_time: f.display_time,
                width: f.width,
                height: f.height,
                data: convert_bgra_to_rgb(f.data.clone()),
            },
            // Frame::YUVFrame(f) => ,
            _ => RGBFrame {
                display_time: 0,
                width: 0,
                height: 0,
                data: vec![],
            },
        }
    }
}

pub fn remove_alpha_channel(frame_data: Vec<u8>) -> Vec<u8> {
    let width = frame_data.len();
    let width_without_alpha = (width / 4) * 3;
    let mut data: Vec<u8> = vec![0; width_without_alpha];

    for (src, dst) in frame_data.chunks_exact(4).zip(data.chunks_exact_mut(3)) {
        dst[0] = src[0];
        dst[1] = src[1];
        dst[2] = src[2];
    }
    data
}

pub fn convert_bgra_to_rgb(frame_data: Vec<u8>) -> Vec<u8> {
    let width = frame_data.len();
    let width_without_alpha = (width / 4) * 3;

    let mut data: Vec<u8> = vec![0; width_without_alpha];

    for (src, dst) in frame_data.chunks_exact(4).zip(data.chunks_exact_mut(3)) {
        dst[0] = src[2];
        dst[1] = src[1];
        dst[2] = src[0];
    }

    data
}

pub fn convert_bgra_to_yuv420(
    bgra: &[u8],
    width: usize,
    height: usize,
) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    let mut y_plane = vec![0u8; width * height];
    let mut u_plane = vec![0u8; (width * height) / 4];
    let mut v_plane = vec![0u8; (width * height) / 4];

    for y in 0..height {
        for x in 0..width {
            let idx = (y * width + x) * 4;
            let b = bgra[idx + 0] as f32;
            let g = bgra[idx + 1] as f32;
            let r = bgra[idx + 2] as f32;

            let y_val = (0.299 * r + 0.587 * g + 0.114 * b).round() as u8;
            y_plane[y * width + x] = y_val;

            if y % 2 == 0 && x % 2 == 0 {
                // Simple 2x2 subsampling
                let u_val = ((-0.14713 * r - 0.28886 * g + 0.436 * b) + 128.0).round() as u8;
                let v_val = ((0.615 * r - 0.51499 * g - 0.10001 * b) + 128.0).round() as u8;
                let uv_index = (y / 2) * (width / 2) + (x / 2);
                u_plane[uv_index] = u_val;
                v_plane[uv_index] = v_val;
            }
        }
    }

    (y_plane, u_plane, v_plane)
}

pub fn get_cropped_data(data: Vec<u8>, cur_width: u32, height: u32, width: u32) -> Vec<u8> {
    if data.len() as u32 != height * cur_width * 4 {
        data
    } else {
        let mut cropped_data: Vec<u8> = vec![0; (4 * height * width).try_into().unwrap()];
        let mut cropped_data_index = 0;

        for (i, item) in data.iter().enumerate() {
            let x = i as u32 % (cur_width * 4);
            if x < (width * 4) {
                cropped_data[cropped_data_index] = *item;
                cropped_data_index += 1;
            }
        }
        cropped_data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_alpha_channel() {
        assert_eq!(remove_alpha_channel(vec![1, 2, 3, 0]), vec![1, 2, 3]);
        assert_eq!(
            remove_alpha_channel(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![1, 2, 3, 5, 6, 7]
        );
    }

    #[test]
    fn test_convert_bgra_to_rgb() {
        assert_eq!(convert_bgra_to_rgb(vec![1, 2, 3, 0]), vec![3, 2, 1]);
        assert_eq!(
            convert_bgra_to_rgb(vec![1, 2, 3, 4, 5, 6, 7, 8]),
            vec![3, 2, 1, 7, 6, 5]
        );
    }

    macro_rules! rgba {
        ($n:expr) => {
            &mut vec![$n, $n, $n, $n]
        };
    }

    #[test]
    pub fn test_get_cropped_data() {
        let mut data: Vec<u8> = Vec::new();
        for i in 1..=9 {
            data.append(rgba!(i));
        }
        let mut expected: Vec<u8> = Vec::new();
        expected.append(rgba!(1));
        expected.append(rgba!(2));
        expected.append(rgba!(4));
        expected.append(rgba!(5));
        expected.append(rgba!(7));
        expected.append(rgba!(8));
        assert_eq!(get_cropped_data(data, 3, 3, 2), expected)
    }
}
