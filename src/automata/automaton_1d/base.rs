use macroquad::window::{screen_height, screen_width};

pub struct Automaton1DInfo<Data: PartialEq + PartialOrd> {
    pub current_row: usize,         
    pub data: Vec<Vec<Data>>,
    pub seen_range: (usize, usize), // the cells that are actually seen on-screen
}

impl<Data: Clone + Copy + Default + PartialEq + PartialOrd> Automaton1DInfo<Data> {
    pub fn new(width: usize, seen_range: (usize, usize)) -> Self {
        let cell_width = screen_width() / (seen_range.1 - seen_range.0) as f32;
        let height = (screen_height() / cell_width as f32) as usize;

        let mut data: Vec<Vec<Data>> = Vec::new();
        for _ in 0..height {
            data.push(vec![Data::default(); width]);
        }

        Self {
            current_row: 0,
            data,
            seen_range,
        }
    }

    pub fn get_width(&self) -> usize {
        self.data[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.data.len()
    }

    pub fn move_window(&mut self, direction: i32) {
        if direction < 0 {
            if self.seen_range.0 > 0 {
                self.seen_range.0 -= 1;
                self.seen_range.1 -= 1;
            }
        }

        if direction > 0 {
            if self.seen_range.1 < self.data[0].len()-1 {
                self.seen_range.0 += 1;
                self.seen_range.1 += 1;
            }
        }
    }

    pub fn increment_time(&mut self) {
        if self.current_row < self.get_height()-1 {
            self.current_row += 1;
        } else {
            for h in 0..self.get_height()-1 {
                for i in 0..self.get_width() {
                    self.data[h][i] = self.data[h+1][i];
                }
            }
        }
    }

    pub fn get_previous_row(&self) -> Vec<Data> {
        self.data[self.current_row-1].clone()
    }
}

pub trait Automaton1D<Data: Clone + Default + PartialEq + PartialOrd, Config, Rule> {
    fn new(width: usize, config: Config, rule: Rule) -> Self;
    fn apply_config(&mut self, config: Config);

    fn restart(&mut self);
    fn pass_time(&mut self);

    fn move_window(&mut self, direction: i32);

    fn control(&mut self);

    fn draw(&self);
}