use std::{ops::Deref, time::{Duration, Instant}};

pub struct StopDuration(Duration);

impl Deref for StopDuration {
    type Target = Duration;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl StopDuration {
    pub fn report(&self) -> String {
        let seconds = self.as_secs();
        if seconds > 0 {
            format!("{seconds}.{:.2}", self.as_millis())
        } else {
            match self.as_nanos() {
                ..1_000 => format!("{}ns", self.as_nanos()),
                1_000..1_000_000 => format!("{}μs", self.as_micros()),
                1_000_000.. => format!("{}ms", self.as_millis()),
            }
        }
    }
}

pub struct Stopwatch {
    pub is_running: bool,
    elapsed: Duration,
    last_start: Option<Instant>,   
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            is_running: false,
            elapsed: Duration::ZERO,
            last_start: None,
        }
    }

    pub fn start(&mut self) -> bool {
        if self.is_running {
            false
        } else {
            self.last_start = Some(Instant::now());
            self.is_running = true;
            true
        }
    }

    pub fn stop(&mut self) -> StopDuration {
        if self.is_running {
            let now = Instant::now();
            self.is_running = false;
            self.elapsed += now - self.last_start.unwrap();
        }
        StopDuration(self.elapsed)
    }

    pub fn lap(&mut self) -> StopDuration {
        if self.is_running {
            let now = Instant::now();
            let lap = now - self.last_start.unwrap();
            self.elapsed += lap;
            self.last_start = Some(now);
            StopDuration(lap)
        } else {
            StopDuration(Duration::ZERO)
        }
    }

    pub fn reset(&mut self) {
        self.elapsed = Duration::ZERO;
        self.last_start = None;
        self.is_running = false;
    }
}

