extern crate serde_json;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

use std::process::Command;
use std::error::Error;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };

// Static Variables
static DATA_PATH: &'static str = "data.json";

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct User {
    name: String,
    balance: i64,
}

struct Game {
    gl: GlGraphics,
}

fn main() {

    // Clear terminal on startup
    clear_console();

    // Create File Path
    let path = Path::new(DATA_PATH);
    let display = path.display();

    // Open File
    let file = File::open(&path);

    // Check if file was found
    if file.is_ok() {

        let mut buffer = String::new();
        file.unwrap().read_to_string(&mut buffer);

        // Parse Data
        let data = serde_json::from_str(&buffer);

        // Check if parsed correctly
        if data.is_ok() {
            let res: User = data.unwrap();
            println!("Welcome back to Rust Idler!");
            start_game(&res);
        } else {
            println!("Sorry, something is wrong with your savegame!");
        }

    } else { // If not, run the following

        // Request User Input - Name
        let name = input("Welcome to Rust Idler! Looks like it's your first time, what should we call you?\n -> ")
            .expect("Something went wrong!");

        // Parse Data
        let data = User { name: name, balance: 100 };

        // Format data into a JSON string
        let res = serde_json::to_string_pretty(&data);

        // Create File
        let mut created_file = match File::create(DATA_PATH) {
            Err(why) => panic!("Can't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Write File - & Send success message
        match created_file.write_all(&res.ok().unwrap().as_bytes()) {
            Err(why) => panic!("Can't write to {}: {}", display, why.description()),
            Ok(_) => println!("\nWell {}... We've added $100 to your account so you can get started, best of luck!", data.name),
        }

        // Start Game
        start_game(&data);


    }

}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics;

        let BLURPLE: [f32; 4] = [0.447, 0.537, 0.854, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(BLURPLE, gl);
        });

    }

    fn update(&mut self) {

    }

    fn pressed(&mut self, btn: &Button) {

    }

}

fn start_game(user_info: &User) {
    println!(""); // Add Line Space

    // Print Information
    println!("Name: {}", user_info.name);
    println!("Balance: ${}", user_info.balance);

    // Create Display
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Rust Idler",
        [1280, 720]
    ).opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new()).ups(10);
    while let Some(e) = events.next(&mut window) {

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }

    }

}

fn clear_console() {
    let output = Command::new("clear").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn input(user_message: &str) -> io::Result<String> {
    use std::io::Write; // Use Trait
    print!("{}", user_message); // Add message to print buffer
    io::stdout().flush()?; // Flush buffer (output everything in print)

    let mut buffer: String = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_right().to_owned())
}
