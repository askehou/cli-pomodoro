use std::env::args;
use std::time::{Duration, Instant};

fn main() {
    let args: Vec<String> = args().collect();
    let mut durations: (u64, u64, u64) = (25, 5, 10);

    if args.len() == 4 {
        let p = args[1].parse::<u64>();
        let s = args[2].parse::<u64>();
        let l = args[3].parse::<u64>();
        match (p, s, l) {
            (Ok(p_dur), Ok(s_dur), Ok(l_dur)) => durations = (p_dur, s_dur, l_dur),
            _ => panic!("Invalid inputs!"),
        }

    // if no args then use defaults
    } else if args.len() == 1 {
        println!("Using default Pomodoro values!")
    // fail if num args not equal to 0 or 3
    } else {
        panic!("Please run the program with either 0 (default) or 3 arguments!")
    }

    let mut is_alive = true;
    let pomodoro = PomodoroTimer::new(durations);

    let mut start_time = Instant::now();
    while is_alive {
        // run pomodoro actions
        let time_elapsed = start_time.elapsed();
        
    }
}

const CONT_MSG: &str = "Press any button to start timer...";

enum TimerType {
    Pomodoro,
    ShortBreak,
    LongBreak,
}

struct PomodoroTimer {
    /// Duration of the 'active' Pomodoro timer, in seconds
    pomodoro_duration: Duration,

    /// Duration of a 'short' break timer, in seconds
    sbreak_duration: Duration,

    /// Duration of a 'long' break timer, in seconds
    lbreak_duration: Duration,

    /// Number of elapsed Pomodoro timers
    num_pomodoros_elapsed: u8,

    /// State of the PomodoroTimer
    timer_state: TimerType,
}

impl PomodoroTimer {
    pub fn new(
        (pomodoro_duration_min, sbreak_duration_min, lbreak_duration_min): (u64, u64, u64),
    ) -> PomodoroTimer {
        let pomo_duration_secs = pomodoro_duration_min * 60;
        let sbreak_duration_secs = sbreak_duration_min * 60;
        let lbreak_duration_secs = lbreak_duration_min * 60;

        PomodoroTimer {
            pomodoro_duration: Duration::from_secs(pomo_duration_secs),
            sbreak_duration: Duration::from_secs(sbreak_duration_secs),
            lbreak_duration: Duration::from_secs(lbreak_duration_secs),
            num_pomodoros_elapsed: 0,
            timer_state: TimerType::Pomodoro,
        }
    }

    pub fn run_timer(&mut self) -> Instant {
        if let TimerType::Pomodoro = self.timer_state {
            self.num_pomodoros_elapsed += 1;
        }
        Instant::now()
    }

    pub fn end_timer(&mut self) {
        match (&self.timer_state, self.num_pomodoros_elapsed) {
            (TimerType::Pomodoro, n) if n % 4 == 0 => self.timer_state = TimerType::LongBreak,
            (TimerType::Pomodoro, _) => self.timer_state = TimerType::ShortBreak,
            (_, _) => self.timer_state = TimerType::Pomodoro,
        }
    }

    pub fn poll_timer(&self, elapsed_time: Duration) -> bool {
        match self.timer_state {
            TimerType::Pomodoro => self.pomodoro_duration > elapsed_time,
            TimerType::ShortBreak => self.sbreak_duration > elapsed_time,
            TimerType::LongBreak => self.lbreak_duration > elapsed_time,
        }
    }
}
