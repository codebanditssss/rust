use eframe::egui;  // for gui elements like buttons, text
use rand::Rng;     // for generating random numbers

// this struct holds all our game data
struct CowsAndBullsApp {
    secret_number: String,     // the 4-digit number to guess
    current_guess: String,     // what user types in the input box
    game_history: String,      // all previous guesses and results as one big string
    attempts: u32,             // how many guesses so far
    game_won: bool,           // true when player wins
    game_lost: bool,          // true when player runs out of attempts
    show_instructions: bool,   // whether to show game rules
    max_attempts: u32,        // maximum allowed attempts (10)
}

// create a new game
fn create_new_app() -> CowsAndBullsApp {
    let mut app = CowsAndBullsApp {
        secret_number: String::new(),
        current_guess: String::new(),
        game_history: String::new(),
        attempts: 0,
        game_won: false,
        game_lost: false,
        show_instructions: true,
        max_attempts: 10,
    };
    generate_new_secret(&mut app);
    app
}

// generates a random 4-digit number with unique digits
fn generate_new_secret(app: &mut CowsAndBullsApp) {
    let mut rng = rand::thread_rng();
    let mut secret = String::new();
    
    // generate 4 unique digits 
    while secret.len() < 4 {
        let digit = rng.gen_range(0..=9);
        let digit_str = digit.to_string();
        
        // check if this digit is already in our secret
        if !secret.contains(&digit_str) {
            secret.push_str(&digit_str);
        }
    }
    
    app.secret_number = secret;
}

// check if the guess is valid (4 digits, unique)
fn is_valid_guess(guess: &str) -> Result<(), String> {
    // check length 
    if guess.len() != 4 {
        return Err("must be exactly 4 digits".to_string());
    }
    
    // check if all are digits using string methods
    let mut all_digits = true;
    for ch in guess.chars() {
        if !ch.is_ascii_digit() {
            all_digits = false;
            break;
        }
    }
    if !all_digits {
        return Err("only numbers allowed".to_string());
    }
    
    // check for unique digits using nested loops 
    let chars: String = guess.to_string();
    for i in 0..chars.len() {
        for j in (i + 1)..chars.len() {
            let char_i = chars.chars().nth(i).unwrap();
            let char_j = chars.chars().nth(j).unwrap();
            if char_i == char_j {
                return Err("all digits must be unique".to_string());
            }
        }
    }
    
    Ok(())
}

// calculate cows and bulls using basic string operations
fn calculate_cows_and_bulls(secret: &str, guess: &str) -> (u32, u32) {
    let mut bulls = 0;
    let mut cows = 0;
    
    // count bulls (correct position)
    for i in 0..4 {
        let secret_char = secret.chars().nth(i).unwrap();
        let guess_char = guess.chars().nth(i).unwrap();
        if secret_char == guess_char {
            bulls += 1;
        }
    }
    
    // count cows (correct digit, wrong position)
    for i in 0..4 {
        let guess_char = guess.chars().nth(i).unwrap();
        let secret_char = secret.chars().nth(i).unwrap();
        
        // if not a bull, check if digit exists elsewhere
        if guess_char != secret_char {
            if secret.contains(guess_char) {
                cows += 1;
            }
        }
    }
    
    (cows, bulls)
}

// process a user's guess
fn make_guess(app: &mut CowsAndBullsApp, guess: String) {
    match is_valid_guess(&guess) {
        Ok(()) => {
            app.attempts += 1;
            let (cows, bulls) = calculate_cows_and_bulls(&app.secret_number, &guess);
            
            let cow_text = if cows == 1 { "cow" } else { "cows" };
            let bull_text = if bulls == 1 { "bull" } else { "bulls" };
            
            let result = format!(
                "attempt {}: {} -> {} {}, {} {}\n",
                app.attempts, guess, cows, cow_text, bulls, bull_text
            );
            
            app.game_history.push_str(&result);
            
            // check win condition
            if bulls == 4 {
                app.game_won = true;
                let win_msg = format!("you won in {} attempts!\n", app.attempts);
                app.game_history.push_str(&win_msg);
            } else if app.attempts >= app.max_attempts {
                // player ran out of attempts
                app.game_lost = true;
                let lose_msg = format!("game over! the secret number was: {}\n", app.secret_number);
                app.game_history.push_str(&lose_msg);
            }
            
            app.current_guess.clear();
        }
        Err(error) => {
            let error_msg = format!("invalid guess: {}\n", error);
            app.game_history.push_str(&error_msg);
        }
    }
}

// start a new game
fn restart_game(app: &mut CowsAndBullsApp) {
    generate_new_secret(app);
    app.current_guess.clear();
    app.game_history.clear();
    app.attempts = 0;
    app.game_won = false;
    app.game_lost = false;
}

// draw the instructions screen
fn show_instructions_panel(app: &mut CowsAndBullsApp, ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(20.0);
        
        ui.label("how to play cows and bulls:");
        ui.add_space(10.0);
        
        ui.label("- i have picked a secret 4-digit number");
        ui.label("- all digits are unique (no repeats)");
        ui.label("- you have 10 attempts to guess it");
        ui.add_space(10.0);
        
        ui.label("feedback system:");
        ui.label("- bull = correct digit in correct position");
        ui.label("- cow = correct digit in wrong position");
        ui.add_space(10.0);
        
        ui.label("example:");
        ui.label("secret: 1234, guess: 1567");
        ui.label("result: 1 bull, 0 cows");
        ui.add_space(20.0);
        
        if ui.button("start game").clicked() {
            app.show_instructions = false;
        }
    });
}

// draw the main game screen
fn show_game_panel(app: &mut CowsAndBullsApp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        // game status
        ui.horizontal(|ui| {
            ui.label(format!("attempts: {}/{}", app.attempts, app.max_attempts));
            ui.separator();
            if ui.button("instructions").clicked() {
                app.show_instructions = true;
            }
            ui.separator();
            if ui.button("new game").clicked() {
                restart_game(app);
            }
        });
        
        ui.separator();
        ui.add_space(10.0);
        
        // input section
        if !app.game_won && !app.game_lost {
            ui.label("enter your 4-digit guess:");
            
            ui.horizontal(|ui| {
                // text input box
                let response = ui.text_edit_singleline(&mut app.current_guess);
                
                // submit button
                let submit_button = ui.button("submit guess");
                
                // handle enter key or button click
                if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) 
                   || submit_button.clicked() {
                    if !app.current_guess.is_empty() {
                        let guess = app.current_guess.clone();
                        make_guess(app, guess);
                    }
                }
            });
        } else if app.game_won {
            ui.vertical_centered(|ui| {
                ui.colored_label(egui::Color32::GREEN, "congratulations! you won!");
                if ui.button("play again").clicked() {
                    restart_game(app);
                }
            });
        } else if app.game_lost {
            ui.vertical_centered(|ui| {
                ui.colored_label(egui::Color32::RED, "game over! you ran out of attempts");
                ui.label(format!("the secret number was: {}", app.secret_number));
                if ui.button("play again").clicked() {
                    restart_game(app);
                }
            });
        }
        
        ui.add_space(20.0);
        
        // game history
        if !app.game_history.is_empty() {
            ui.label("game history:");
            ui.separator();
            
            egui::ScrollArea::vertical()
                .max_height(200.0)
                .show(ui, |ui| {
                    ui.label(&app.game_history);
                });
        }
    });
}

// required for egui to work
struct MyApp {
    game: CowsAndBullsApp,
}

impl MyApp {
    fn new() -> MyApp {
        MyApp {
            game: create_new_app(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // create the main panel that fills the window
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("cows and bulls game");
            ui.separator();
            
            // show instructions if needed
            if self.game.show_instructions {
                show_instructions_panel(&mut self.game, ui);
            } else {
                show_game_panel(&mut self.game, ui);
            }
        });
    }
}

fn main() {
    // create window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 500.0])  // window size
            .with_title("cows and bulls game"),  // window title
        ..Default::default()
    };
    
    // run the app
    eframe::run_native(
        "cows and bulls",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    ).unwrap();
}