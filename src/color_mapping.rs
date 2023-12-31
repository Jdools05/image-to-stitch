use image::Rgba;

use crate::color_threads;
use crate::utils;


pub struct ColorMapping {
    colors: Vec<Rgba<u8>>,
}


impl ColorMapping {
    pub fn new(colors: Vec<Rgba<u8>>) -> ColorMapping {
        ColorMapping {
            colors: colors,
        }
    }

    pub fn push(&mut self, color: Rgba<u8>) {
        self.colors.push(color);
    }
    
    // get the closest color from the list of colors using euclidean distance
    pub fn get_color(&self, color: Rgba<u8>) -> Rgba<u8> {
        let mut closest_color: Rgba<u8> = Rgba([0, 0, 0, 0]);
        let mut closest_distance: f32 = 1000.0;

        for c in &self.colors {
            let distance: f32 = self.distance(&color, &c);
            if distance < closest_distance {
                closest_color = c.clone();
                closest_distance = distance;
            }
        }

        closest_color
    }

    pub fn from_threads(color_threads: Vec<color_threads::ColorThread>) -> ColorMapping {
        let mut colors: Vec<Rgba<u8>> = Vec::new();
        for color_thread in color_threads {
            colors.push(utils::dehex_color!(color_thread.get_hex()));
        }
        ColorMapping::new(colors)
    }

    pub fn len(&self) -> usize {
        self.colors.len()
    }

    fn distance(&self, color1: &Rgba<u8>, color2: &Rgba<u8>) -> f32 {
        let mut distance: f32 = 0.0;
        for i in 0..4 {
            distance += (color1[i] as f32 - color2[i] as f32).powf(2.0);
        }
        distance.sqrt()
    }
}