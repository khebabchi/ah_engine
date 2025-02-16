use std::time::Instant;

pub struct Time {
    time_started: Instant,
    pub delta_time: f32,
    begin_time: f32,
    end_time: f32,
    time_elapsed: f32,
}

impl Time {
    pub const FIXED_DELTA_TIME: f32 = 0.01;

    pub fn new() -> Self {
        Self {
            time_started: Instant::now(),
            delta_time: 0.0,
            begin_time: 0.0,
            end_time: 0.0,
            time_elapsed: 0.0,
        }
    }

    pub fn get_time(&self) -> f32 {
        self.time_started.elapsed().as_secs_f32()
    }

    pub fn init(&mut self) {
        self.begin_time = self.get_time();
        self.end_time = self.get_time();
    }

    pub fn update_dt(&mut self) {
        self.end_time = self.get_time();
        self.delta_time = self.end_time - self.begin_time;
        self.time_elapsed += self.delta_time;
        self.begin_time = self.end_time;
    }

    pub fn fixed_dt_elapsed(&mut self )->bool
    {
        if self.time_elapsed < Self::FIXED_DELTA_TIME {
            return false;
        }
        while self.time_elapsed > Self::FIXED_DELTA_TIME {
            self.time_elapsed -= Self::FIXED_DELTA_TIME;
        }
        true
    }
}
