use crate::error;

fn parse_str_to_rgb(s: String, length: usize) -> Vec<u8> {
    let components: Vec<u8> = s
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    if components.len() != length {
        panic!("{}", error::UNPARSEBALE_COLOR);
    }

    components
}

#[derive(Debug)]
pub struct Rgb(u8, u8, u8);

impl Rgb {
    fn as_u8_array(&self) -> [u8; 3] {
        [self.0, self.1, self.2]
    }

    pub fn squared_distance(&self, other: Rgb) -> u32 {
        (((self.0 as i32) - (other.0 as i32)).pow(2)
            + ((self.1 as i32) - (other.1 as i32)).pow(2)
            + ((self.2 as i32) - (other.2 as i32)).pow(2)) as u32
    }

    pub fn to_lab(&self) -> deltae::LabValue {
        let lab = lab::Lab::from_rgb(&self.as_u8_array());

        deltae::LabValue {
            l: lab.l,
            a: lab.a,
            b: lab.b,
        }
    }
}

impl From<&image::Rgb<u8>> for Rgb {
    fn from(rgb: &image::Rgb<u8>) -> Rgb {
        Rgb(rgb[0], rgb[1], rgb[2])
    }
}

impl From<String> for Rgb {
    fn from(s: String) -> Rgb {
        let split = parse_str_to_rgb(s, 3);

        Rgb(split[0], split[1], split[2])
    }
}
