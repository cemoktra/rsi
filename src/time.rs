// ========================================
// Time module
// ========================================
mod si_time {
    #[derive(Copy, Clone)]
    pub enum TimeUnit {
        Milliseconds,
        Seconds,
        Minutes,
        Days,
    }
}