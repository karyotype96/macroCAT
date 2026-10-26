use macroquad::{input::mouse_position, window::{screen_height, screen_width}};

pub struct Automaton2DInfo<Data: PartialEq + PartialOrd> {
    pub frame_number: u128,
    pub data1: Vec<Vec<Data>>,
    pub data2: Vec<Vec<Data>>,
    pub seen_range_x: (usize, usize),
    pub seen_range_y: (usize, usize),

    use_data2: bool,
    zoom_factor: u32,
}

impl<Data: Clone + Copy + Default + PartialEq + PartialOrd> Automaton2DInfo<Data> {
    pub fn new(width: usize, height: usize, seen_range: (usize, usize)) -> Self {
        let cell_width = screen_width() / (seen_range.1 - seen_range.0) as f32;
        let view_height = (screen_height() / cell_width as f32) as usize;

        let seen_range_y = (height / 2 - view_height / 2, height / 2 + view_height / 2);

        let mut data: Vec<Vec<Data>> = Vec::new();
        for _ in 0..height {
            data.push(vec![Data::default(); width]);
        }

        Self {
            frame_number: 0,
            data1: data.clone(),
            data2: data.clone(),
            seen_range_x: seen_range,
            seen_range_y,

            use_data2: false,
            zoom_factor: 1
        }
    }
    pub fn get_width(&self) -> usize {
        self.data1[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.data1.len()
    }

    pub fn move_window(&mut self, direction: (i32, i32)) {
        if direction.0 < 0 {
            if self.seen_range_x.0 > 0 {
                self.seen_range_x.0 -= 1;
                self.seen_range_x.1 -= 1;
            }
        }

        if direction.0 > 0 {
            if self.seen_range_x.1 < self.data1[0].len()-1 {
                self.seen_range_x.0 += 1;
                self.seen_range_x.1 += 1;
            }
        }

        if direction.1 < 0 {
            if self.seen_range_y.0 > 0 {
                self.seen_range_y.0 -= 1;
                self.seen_range_y.1 -= 1;
            }
        }

        if direction.1 > 0 {
            if self.seen_range_y.1 < self.data1.len()-1 {
                self.seen_range_y.0 += 1;
                self.seen_range_y.1 += 1;
            }
        }
    }

    pub fn increment_time(&mut self) {
        self.frame_number += 1;
        self.use_data2 = !self.use_data2;
    }

    pub fn mouse_get_cell(&self) -> Data {
        let mouse_pos = mouse_position();
        let cell_size = self.get_cell_size();

        let cell_position = (
            (mouse_pos.0 / cell_size).floor() as usize + self.seen_range_x.0,
            (mouse_pos.1 / cell_size).floor() as usize + self.seen_range_y.0,
        );

        self.read_from(cell_position.0 as i32, cell_position.1 as i32)
    }

    pub fn mouse_set_cell(&mut self, value: Data) {
        let mouse_pos = mouse_position();
        let cell_size = self.get_cell_size();

        let cell_position = (
            (mouse_pos.0 / cell_size).floor() as usize + self.seen_range_x.0,
            (mouse_pos.1 / cell_size).floor() as usize + self.seen_range_y.0,
        );

        self.write_data_in_place(cell_position.0 as i32, cell_position.1 as i32, value);
    }

    pub fn get_cell_size(&self) -> f32 {
        screen_width() / (self.seen_range_x.1 - self.seen_range_x.0) as f32
    }

    pub fn zoom_out(&mut self) {
        let center_x = (self.seen_range_x.1 - self.seen_range_x.0) / 2 + self.seen_range_x.0;
        let center_y = (self.seen_range_y.1 - self.seen_range_y.0) / 2 + self.seen_range_y.0;

        let horiz_half_width = center_x - self.seen_range_x.0;
        let vert_half_width = center_y - self.seen_range_y.0;

        if self.zoom_factor < 4 {
            self.zoom_factor += 1;

            // Zoom window horizontally
            let left_x_tmp = self.seen_range_x.0;
            self.seen_range_x.0 = self.seen_range_x.0.saturating_sub(horiz_half_width);

            // If the left edge of the window would go past 0, stop it from extending
            // past the edge and increase the right side of the x range by the difference
            let remaining_x = horiz_half_width - (left_x_tmp - self.seen_range_x.0);

            self.seen_range_x.1 += horiz_half_width + remaining_x;

            while self.seen_range_x.1 >= self.get_width() {
                self.move_window((-1, 0));
            }

            // Zoom window vertically
            let top_y_tmp = self.seen_range_y.0;
            self.seen_range_y.0 = self.seen_range_y.0.saturating_sub(vert_half_width);

            let remaining_y = vert_half_width - (top_y_tmp - self.seen_range_y.0);

            self.seen_range_y.1 += vert_half_width + remaining_y;

            while self.seen_range_y.1 >= self.get_height() {
                self.move_window((0, -1));
            }
        }
    }

    pub fn zoom_in(&mut self) {
        let center_x = (self.seen_range_x.1 - self.seen_range_x.0) / 2 + self.seen_range_x.0;
        let center_y = (self.seen_range_y.1 - self.seen_range_y.0) / 2 + self.seen_range_y.0;

        if self.zoom_factor > 1 {
            self.zoom_factor -= 1;

            let horiz_half_width = center_x - self.seen_range_x.0;
            let vert_half_width = center_y - self.seen_range_y.0;

            self.seen_range_x.0 += horiz_half_width / 2;
            self.seen_range_x.1 -= horiz_half_width / 2;

            self.seen_range_y.0 += vert_half_width / 2;
            self.seen_range_y.1 -= vert_half_width / 2;
        }
    }

    pub fn read_from(&self, x: i32, y: i32) -> Data {
        let width = self.get_width() as i32;
        let x_index: usize = (((x % width) + width) % width) as usize;
        let y_index: usize = (((y % width) + width) % width) as usize;
        
        if !self.use_data2 {
            self.data1[y_index][x_index]
        } else {
            self.data2[y_index][x_index]
        }
    }

    pub fn write_data_in_place(&mut self, x: i32, y: i32, data: Data) {
        let width = self.get_width() as i32;
        let x_index: usize = (((x % width) + width) % width) as usize;
        let y_index: usize = (((y % width) + width) % width) as usize;

        if !self.use_data2 {
            self.data1[y_index][x_index] = data;
        } else {
            self.data2[y_index][x_index] = data;
        }
    }

    pub fn write_data(&mut self, x: i32, y: i32, data: Data) {
        let width = self.get_width() as i32;
        let x_index: usize = (((x % width) + width) % width) as usize;
        let y_index: usize = (((y % width) + width) % width) as usize;

        if !self.use_data2 {
            self.data2[y_index][x_index] = data;
        } else {
            self.data1[y_index][x_index] = data;
        }
    }
}

pub trait Automaton2D<Data: Clone + Default + PartialEq + PartialOrd, Config, Rule> {
    fn new(width: usize, height: usize, config: Config, rule: Rule) -> Self;
    fn apply_config(&mut self, config: Config);

    fn restart(&mut self);
    fn pass_time(&mut self);

    fn move_window(&mut self, direction: (i32, i32));

    fn control(&mut self);

    fn draw(&self);
}