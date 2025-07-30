use eframe::egui;      // Import egui for GUI elements
use rand::Rng;         // Import random number generation

// Define the structure of the guessing game app
struct GuessApp {
    input: String,     // Stores user input as string
    message: String,   // Message to display as feedback
    secret: u32,       // Secret number to guess
}

// Initialize the app with default values
impl Default for GuessApp {
    fn default() -> Self {
        Self {
            input: String::new(),
            message: "Enter a number between 1 and 100".into(),
            secret: rand::thread_rng().gen_range(1..=100), // Generate random number
        }
    }
}

// Implement the main app logic using egui
impl eframe::App for GuessApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Create the central UI panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Guess the Number!"); // Display the title

            // Horizontal row: label + input field
            ui.horizontal(|ui| {
                ui.label("Your guess:");
                ui.text_edit_singleline(&mut self.input); // Text input box
            });

            // Button to submit the guess
            if ui.button("Guess").clicked() {
                // Try to parse the input into a number
                if let Ok(num) = self.input.trim().parse::<u32>() {
                    if num == self.secret {
                        // Correct guess
                        self.message = format!("🎉 Correct! The number was {}. Starting new game...", self.secret);
                        self.secret = rand::thread_rng().gen_range(1..=100); // Start a new game
                        self.input.clear(); // Clear input box
                    } else if num < self.secret {
                        // Guess too low
                        self.message = format!("Too small! The correct number was {}. Try guessing another one!", self.secret);
                        self.secret = rand::thread_rng().gen_range(1..=100); // New game
                        self.input.clear();
                    } else {
                        // Guess too high
                        self.message = format!("Too big! The correct number was {}. Try guessing another one!", self.secret);
                        self.secret = rand::thread_rng().gen_range(1..=100); // New game
                        self.input.clear();
                    }
                } else {
                    // Input couldn't be parsed
                    self.message = "❌ Please enter a valid number.".into();
                }
            }

            // Show result or feedback message
            ui.label(&self.message);
        });
    }
}

// Entry point for the app
fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default(); // Default window options
    eframe::run_native(
        "Guessing Game",  // Window title
        native_options,
        Box::new(|_cc| Box::new(GuessApp::default())), // Initialize app
    )
}
