# egui: easy graphical user interface for rust

## what is egui

egui is a pure rust gui library that follows the immediate mode paradigm. unlike traditional retained mode guis where you create widgets once and then modify them, immediate mode means you describe what the gui should look like every single frame.

## immediate mode vs retained mode

### immediate mode (egui)
```rust
// this runs 60 times per second
fn update(&mut self, ui: &mut egui::Ui) {
    if ui.button("click me").clicked() {
        // handle click
    }
    ui.label("some text");
}
```

### retained mode (traditional)
```rust
// this runs once at startup
fn create_gui() {
    let button = Button::new("click me");
    let label = Label::new("some text");
    // then you modify these widgets later
}
```

## core concepts

### the app trait

every egui application must implement the `eframe::App` trait:

```rust
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // this function is called 60 times per second
        // here you describe what your gui should look like
    }
}
```

### context and frames

- `ctx: &egui::Context` - handles input, manages gui state
- `frame: &mut eframe::Frame` - represents the window itself
- most of the time you only need `ctx`

### the ui object

`ui` is where you actually create gui elements:

```rust
egui::CentralPanel::default().show(ctx, |ui| {
    // ui is available inside this closure
    ui.button("my button");
    ui.label("my label");
});
```

## layout system

### panels

panels are the top-level containers that divide your window:

```rust
// fills the entire window
egui::CentralPanel::default().show(ctx, |ui| {
    // your content here
});

// creates a side panel on the left
egui::SidePanel::left("side_panel").show(ctx, |ui| {
    // sidebar content
});

// creates a top panel
egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
    // top bar content
});
```

### layout containers

inside panels, you use layout containers to arrange elements:

```rust
// arranges elements vertically (one below another)
ui.vertical(|ui| {
    ui.button("top button");
    ui.button("bottom button");
});

// arranges elements horizontally (side by side)
ui.horizontal(|ui| {
    ui.button("left button");
    ui.button("right button");
});

// centers content horizontally
ui.vertical_centered(|ui| {
    ui.button("centered button");
});
```

### spacing and separators

```rust
ui.add_space(20.0);     // adds 20 pixels of empty space
ui.separator();         // draws a horizontal line
```

## basic gui elements

### buttons

```rust
// simple button
if ui.button("click me").clicked() {
    // this runs when button is clicked
}

// button with custom size
if ui.add_sized([100.0, 30.0], egui::Button::new("sized")).clicked() {
    // handle click
}
```

### text elements

```rust
// simple text label
ui.label("hello world");

// large heading text
ui.heading("big title");

// colored text
ui.colored_label(egui::Color32::RED, "red text");
ui.colored_label(egui::Color32::GREEN, "green text");
```

### text input

```rust
// single line text input
let mut text = String::new();
ui.text_edit_singleline(&mut text);

// multiline text input
let mut multiline_text = String::new();
ui.text_edit_multiline(&mut multiline_text);
```

### checkboxes and radio buttons

```rust
// checkbox
let mut checked = false;
ui.checkbox(&mut checked, "check me");

// radio buttons
let mut selected = 0;
ui.radio_value(&mut selected, 0, "option 1");
ui.radio_value(&mut selected, 1, "option 2");
ui.radio_value(&mut selected, 2, "option 3");
```

## advanced features

### scroll areas

for content that might be too long to fit:

```rust
egui::ScrollArea::vertical()
    .max_height(200.0)  // limit height to 200 pixels
    .show(ui, |ui| {
        // lots of content that can scroll
        for i in 0..100 {
            ui.label(format!("line {}", i));
        }
    });
```

### windows and popups

```rust
// create a separate window
egui::Window::new("my window")
    .resizable(true)
    .show(ctx, |ui| {
        ui.label("content inside window");
    });

// simple popup
if ui.button("show popup").clicked() {
    // trigger popup logic
}
```

### input handling

```rust
// detect keyboard input
if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
    // enter key was pressed
}

// detect when text input loses focus
let response = ui.text_edit_singleline(&mut text);
if response.lost_focus() {
    // user clicked somewhere else
}
```

## setting up an egui application

### cargo.toml dependencies

```toml
[dependencies]
eframe = "0.29"
egui = "0.29"
```

### basic application structure 

```rust
use eframe::egui;

// your game data
struct GameData {
    counter: i32,
    text: String,
}

// create new game data
fn create_game_data() -> GameData {
    GameData {
        counter: 0,
        text: String::new(),
    }
}

// update counter function
fn increment_counter(data: &mut GameData) {
    data.counter += 1;
}

// the wrapper struct required by egui
struct MyApp {
    game_data: GameData,
}

impl MyApp {
    fn new() -> MyApp {
        MyApp {
            game_data: create_game_data(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("my application");
            
            if ui.button("increment").clicked() {
                increment_counter(&mut self.game_data);
            }
            
            ui.label(format!("counter: {}", self.game_data.counter));
            
            ui.text_edit_singleline(&mut self.game_data.text);
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_title("my app"),
        ..Default::default()
    };
    
    eframe::run_native(
        "my app",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new()))),
    ).unwrap();
}
```

## state management

### application state

your application state lives in your struct:

```rust
struct GameData {
    score: i32,           // persistent between frames
    player_name: String,  // persistent between frames
    game_over: bool,      // persistent between frames
}

// functions to modify state
fn update_score(data: &mut GameData, new_score: i32) {
    data.score = new_score;
}

fn set_game_over(data: &mut GameData) {
    data.game_over = true;
}
```

### temporary state

gui element responses are temporary and only last one frame:

```rust
// this is temporary - only true for one frame when clicked
if ui.button("click").clicked() {
    // handle the click
}

// this is persistent - stored in your struct using functions
update_score(&mut self.game_data, self.game_data.score + 1);
```

## event handling patterns

### button clicks

```rust
if ui.button("save").clicked() {
    // save data
}

if ui.button("load").clicked() {
    // load data
}
```

### text input submission

```rust
let response = ui.text_edit_singleline(&mut self.input);

// check for enter key or lost focus
if (response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
    // process the input
    self.process_input();
}
```

### conditional gui

```rust
if self.game_data.game_started {
    // show game interface
    ui.label("game is running");
    if ui.button("pause").clicked() {
        pause_game(&mut self.game_data);
    }
} else {
    // show menu interface
    ui.label("game is paused");
    if ui.button("start").clicked() {
        start_game(&mut self.game_data);
    }
}

// helper functions
fn pause_game(data: &mut GameData) {
    data.game_started = false;
}

fn start_game(data: &mut GameData) {
    data.game_started = true;
}
```

## concepts beyond day 1 and day 2

### traits

```rust
impl eframe::App for MyApp {
    // implementing a trait from an external crate
}
```

**explanation**: traits are like interfaces in other languages. they define a set of methods that a type must implement. when you write `impl eframe::App for MyApp`, you're telling rust that your `MyApp` type implements all the methods required by the `eframe::App` trait.

### closures

```rust
ui.vertical(|ui| {
    // this is a closure - an anonymous function
    ui.button("inside closure");
});
```

**explanation**: the `|ui|` syntax creates a closure (anonymous function). egui uses closures heavily because they allow you to pass code to layout functions. the layout function calls your closure with a `ui` object.

### box and heap allocation

```rust
Box::new(|_cc| Ok(Box::new(MyApp::default())))
```

**explanation**: `Box` puts data on the heap instead of the stack. egui requires this because the framework needs to own your app object and call methods on it. the `Box::new()` moves your app to the heap so egui can manage it.

### default trait

```rust
#[derive(Default)]
struct MyApp {
    counter: i32,
}
```

**explanation**: `#[derive(Default)]` automatically implements the `Default` trait for your struct. this trait provides a `default()` method that creates a "default" instance of your type (usually with zero/empty values).

### option and result types (advanced usage)

```rust
Box::new(|_cc| Ok(Box::new(MyApp::default())))
//              ^^ this returns Result<Box<MyApp>, String>
```

**explanation**: the egui setup function expects a `Result` type. `Ok()` wraps your app in the success variant of the result.

### method chaining

```rust
egui::ScrollArea::vertical()
    .max_height(200.0)
    .show(ui, |ui| { /* ... */ });
```

**explanation**: each method returns `self`, allowing you to chain multiple method calls together. this is a common pattern in rust apis.

### lifetime annotations (implicit)

```rust
fn show_panel(&mut self, ui: &mut egui::Ui) {
    // ui is borrowed for the lifetime of this function
}
```

**explanation**: the `&mut egui::Ui` parameter is a mutable reference with an implicit lifetime. the ui object is borrowed temporarily and must be returned when the function ends.

## performance considerations

### frame rate

egui runs at 60 fps by default. your `update` function should be fast:

```rust
fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // this needs to complete in less than 16ms for smooth 60fps
    // avoid heavy computations here
}
```

### memory usage

gui elements are created and destroyed every frame, but this is very fast because:
- most gui elements are lightweight
- rust's ownership system ensures no memory leaks
- egui is optimized for this pattern

## debugging tips

### response inspection

```rust
let response = ui.button("debug me");
println!("button was clicked: {}", response.clicked());
println!("button is hovered: {}", response.hovered());
```

### state debugging

```rust
ui.label(format!("debug: counter = {}", self.counter));
ui.label(format!("debug: text = '{}'", self.text));
```

### conditional compilation for debug

```rust
if cfg!(debug_assertions) {
    ui.label("this only shows in debug builds");
}
```

this covers the essential concepts of egui that you need to understand for building gui applications in rust.
