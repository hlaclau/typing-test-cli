use rand::Rng;
use std::time::Instant;

const TEST_PASSAGES: &[&str] = &[
    "The quick brown fox jumps over the lazy dog.",
    "Never underestimate the power of a good book and a warm cup of coffee.",
    "Programming in Rust can be challenging yet incredibly rewarding for many.",
    "Learning by doing is an effective way to master new skills and concepts.",
    "A journey of a thousand miles begins with a single step towards the horizon.",
    "The art of programming is the art of organizing complexity and mastering chaos.",
    "Code is like humor. When you have to explain it, it's bad. When it's good, it's obvious.",
    "First, solve the problem. Then, write the code. This approach saves time and frustration.",
    "The best error messages are the ones that tell you exactly what went wrong and why.",
    "Clean code always looks like it was written by someone who cares about the craft.",
];

#[derive(Default)]
pub struct TypingTest {
    target_text: String,
    user_input: String,
    start_time: Option<Instant>,
    current_char: usize,
    correct_chars: usize,
    total_chars: usize,
    wpm: u32,
    accuracy: f64,
    test_completed: bool,
    show_results: bool,
}

impl TypingTest {
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..TEST_PASSAGES.len());
        let target_text = TEST_PASSAGES[random_index].to_string();
        
        Self {
            target_text,
            total_chars: TEST_PASSAGES[random_index].len(),
            ..Default::default()
        }
    }

    pub fn start_test(&mut self) {
        self.start_time = Some(Instant::now());
        self.test_completed = false;
        self.show_results = false;
    }

    pub fn add_char(&mut self, c: char) {
        if self.test_completed {
            return;
        }

        self.user_input.push(c);
        self.current_char = self.user_input.len();
        self.update_stats();
        
        // Check if test is completed
        if self.user_input.len() >= self.target_text.len() {
            self.complete_test();
        }
    }

    pub fn remove_char(&mut self) {
        if self.test_completed || self.user_input.is_empty() {
            return;
        }

        self.user_input.pop();
        self.current_char = self.user_input.len();
        self.update_stats();
    }

    fn update_stats(&mut self) {
        // Calculate correct characters
        self.correct_chars = 0;
        let min_len = self.target_text.len().min(self.user_input.len());
        for i in 0..min_len {
            if self.target_text.chars().nth(i) == self.user_input.chars().nth(i) {
                self.correct_chars += 1;
            }
        }
    }

    fn complete_test(&mut self) {
        if let Some(start_time) = self.start_time {
            let duration = start_time.elapsed();
            let time_minutes = duration.as_secs_f64() / 60.0;
            
            self.wpm = if time_minutes > 0.0 {
                (self.user_input.split_whitespace().count() as f64 / time_minutes) as u32
            } else {
                0
            };
            
            self.accuracy = if self.target_text.len() > 0 {
                (self.correct_chars as f64 / self.target_text.len() as f64) * 100.0
            } else {
                0.0
            };
            
            self.test_completed = true;
            self.show_results = true;
        }
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..TEST_PASSAGES.len());
        self.target_text = TEST_PASSAGES[random_index].to_string();
        self.user_input.clear();
        self.start_time = None;
        self.current_char = 0;
        self.correct_chars = 0;
        self.total_chars = self.target_text.len();
        self.wpm = 0;
        self.accuracy = 0.0;
        self.test_completed = false;
        self.show_results = false;
    }

    // Getters
    pub fn target_text(&self) -> &str {
        &self.target_text
    }

    pub fn user_input(&self) -> &str {
        &self.user_input
    }

    pub fn current_char(&self) -> usize {
        self.current_char
    }

    pub fn correct_chars(&self) -> usize {
        self.correct_chars
    }

    pub fn total_chars(&self) -> usize {
        self.total_chars
    }

    pub fn wpm(&self) -> u32 {
        self.wpm
    }

    pub fn accuracy(&self) -> f64 {
        self.accuracy
    }

    pub fn is_test_started(&self) -> bool {
        self.start_time.is_some()
    }

    pub fn is_test_completed(&self) -> bool {
        self.test_completed
    }

    pub fn show_results(&self) -> bool {
        self.show_results
    }

    pub fn elapsed_time(&self) -> Option<u64> {
        self.start_time.map(|start| start.elapsed().as_secs())
    }
}
