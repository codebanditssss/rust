// star wars episode iv: a new hope - rebel alliance command center
// a comprehensive text-based strategy game using structs, impl, borrowing, and string manipulation
// based on the original 1977 movie plot, characters, and locations

use std::io;

// core game entities - main characters and vehicles from episode iv

// rebel personnel struct - represents key characters like luke, leia, han, etc.
struct RebelPersonnel {
    name: String,           // luke skywalker, princess leia, han solo, etc.
    codename: String,       // red five, your highness, your worship, etc.
    force_sensitivity: u32, // 0-100, luke starts low, grows throughout game
    combat_skill: u32,      // fighting ability
    leadership: u32,        // command capability
    alive: bool,           // survival status during missions
    current_location: String, // yavin 4, death star, mos eisley, etc.
}

// starship struct - x-wings, millennium falcon, tie fighters, death star
struct Starship {
    designation: String,    // red five, millennium falcon, death star, etc.
    ship_class: String,     // x-wing, yt-1300, space station, etc.
    hull_integrity: u32,    // 0-100 health points
    shield_strength: u32,   // defensive capability
    weapons_power: u32,     // offensive capability
    operational: bool,      // working condition
    current_sector: String, // yavin system, alderaan system, etc.
}

// rebel base struct - yavin 4 base, tantive iv, etc.
struct RebelBase {
    name: String,          // yavin 4 base, echo base (future), etc.
    has_death_star_plans: bool, // critical plot element
}



// droid struct - r2-d2, c-3po from the movie
struct Droid {
    designation: String,   // r2-d2, c-3po, etc.
    has_data: bool,       // r2 has death star plans
}

// tuple structs for game mechanics
struct Resources(u32, u32, u32);      // credits, fuel, intelligence points
struct MissionStats(u32, u32, u32);   // success rate, casualties, objectives completed

// imperial message struct for decoding mini-games
struct ImperialTransmission {
    encrypted_message: String,  // coded imperial communications
    decoded_message: String,    // what it actually says
    priority_level: u32,       // how important the intel is
    intercepted_from: String,   // death star, star destroyer, etc.
}

// implementation blocks for game functionality

impl RebelPersonnel {
    // create new rebel character with episode iv stats
    fn new(name: String, codename: String) -> RebelPersonnel {
        // different starting stats based on character from movie
        let (force_sens, combat, leadership) = match name.as_str() {
            "luke skywalker" => (25, 60, 50),     // farm boy with potential
            "princess leia" => (20, 70, 95),      // natural leader
            "han solo" => (0, 85, 60),            // smuggler skills
            "chewbacca" => (0, 90, 40),           // wookiee strength
            "obi-wan kenobi" => (95, 90, 85),     // jedi master
            _ => (10, 50, 50),                     // generic rebel
        };
        
        RebelPersonnel {
            name,
            codename,
            force_sensitivity: force_sens,
            combat_skill: combat,
            leadership: leadership,
            alive: true,
            current_location: String::from("yavin 4"),
        }
    }
    
    // training increases abilities - luke's journey in the movie
    fn train_abilities(&mut self) {
        // force training like with obi-wan
        if self.name == "luke skywalker" {
            self.force_sensitivity += 15;
            println!("{} feels the force growing stronger...", self.name);
        }
        self.combat_skill += 10;
        println!("{} has completed training exercises", self.name);
    }
    
    // use the force for special actions - luke destroying death star
    fn use_force(&self, target: &str) -> bool {
        if self.force_sensitivity > 50 {
            println!("{} reaches out with the force to {}", self.name, target);
            true
        } else {
            println!("{} tries to use the force but lacks training", self.name);
            false
        }
    }
    

}

impl Starship {
    // create ships from episode iv
    fn new(designation: String, ship_class: String) -> Starship {
        // stats based on ships from the movie
        let (hull, shields, weapons) = match ship_class.as_str() {
            "x-wing" => (100, 60, 80),              // rebel fighter
            "yt-1300" => (120, 70, 75),             // millennium falcon
            "imperial star destroyer" => (500, 200, 300), // empire ship
            "death star" => (1000000, 1000, 10000),     // ultimate weapon
            "tie fighter" => (80, 20, 70),          // imperial fighter
            _ => (100, 50, 60),                      // generic ship
        };
        
        Starship {
            designation,
            ship_class,
            hull_integrity: hull,
            shield_strength: shields,
            weapons_power: weapons,
            operational: true,
            current_sector: String::from("outer rim"),
        }
    }
    
    // repair damaged ships - like fixing the millennium falcon
    fn repair_ship(&mut self) {
        if self.hull_integrity < 100 {
            self.hull_integrity += 25;
            if self.hull_integrity > 100 { self.hull_integrity = 100; }
            println!("{} has been repaired (+25 hull integrity)", self.designation);
        }
        self.operational = true;
    }
    

}

impl RebelBase {
    // create yavin 4 base or other rebel outposts
    fn new(name: String) -> RebelBase {
        RebelBase {
            name,
            has_death_star_plans: false,
        }
    }
    
    // receive stolen death star plans - key plot point
    fn receive_death_star_plans(&mut self, from_droid: &Droid) {
        if from_droid.has_data && from_droid.designation == "r2-d2" {
            self.has_death_star_plans = true;
            println!("yavin 4 base has received the death star plans from r2-d2!");
            println!("general dodonna begins analyzing the technical readouts...");
        }
    }
    

}



impl Droid {
    // create r2-d2 and c-3po from movie
    fn new(designation: String) -> Droid {
        let has_data = match designation.as_str() {
            "r2-d2" => true,  // has death star plans
            _ => false,
        };
        
        Droid {
            designation,
            has_data,
        }
    }
}

impl ImperialTransmission {
    // create intercepted imperial messages for decoding mini-games
    fn new(encrypted: String, priority: u32, source: String) -> ImperialTransmission {
        ImperialTransmission {
            encrypted_message: encrypted,
            decoded_message: String::new(),
            priority_level: priority,
            intercepted_from: source,
        }
    }
    
    // decode imperial message using string manipulation (like your string tinkerer)
    fn decode_transmission(&mut self) -> String {
        // simple cipher - reverse the string and convert case
        let reversed: String = self.encrypted_message.chars().rev().collect();
        self.decoded_message = reversed.to_lowercase();
        
        println!("transmission decoded from {}", self.intercepted_from);
        println!("encrypted: {}", self.encrypted_message);
        println!("decoded: {}", self.decoded_message);
        
        self.decoded_message.clone()
    }
    
    // analyze decoded message for intelligence value
    fn analyze_intel(&self) -> u32 {
        let intel_value = self.decoded_message.len() as u32 * self.priority_level;
        println!("intelligence value: {} points", intel_value);
        intel_value
    }
}

// main game struct that manages everything
struct RebelAllianceCommand {
    player_name: String,
    personnel: Vec<RebelPersonnel>,
    fleet: Vec<Starship>,
    bases: Vec<RebelBase>,
    droids: Vec<Droid>,
    resources: Resources,
    mission_stats: MissionStats,
    game_phase: u32, // tracks progression through episode iv plot
}

impl RebelAllianceCommand {
    // initialize game with episode iv starting conditions
    fn new(commander_name: String) -> RebelAllianceCommand {
        // create main characters from the movie
        let mut personnel = Vec::new();
        personnel.push(RebelPersonnel::new(String::from("luke skywalker"), String::from("red five")));
        personnel.push(RebelPersonnel::new(String::from("princess leia"), String::from("your highness")));
        personnel.push(RebelPersonnel::new(String::from("han solo"), String::from("captain solo")));
        personnel.push(RebelPersonnel::new(String::from("chewbacca"), String::from("chewie")));
        personnel.push(RebelPersonnel::new(String::from("obi-wan kenobi"), String::from("ben kenobi")));
        
        // create ships from the movie
        let mut fleet = Vec::new();
        fleet.push(Starship::new(String::from("red five"), String::from("x-wing")));
        fleet.push(Starship::new(String::from("red leader"), String::from("x-wing")));
        fleet.push(Starship::new(String::from("millennium falcon"), String::from("yt-1300")));
        
        // create rebel base
        let mut bases = Vec::new();
        bases.push(RebelBase::new(String::from("yavin 4 base")));
        

        
        // create droids
        let mut droids = Vec::new();
        droids.push(Droid::new(String::from("r2-d2")));
        droids.push(Droid::new(String::from("c-3po")));
        
        RebelAllianceCommand {
            player_name: commander_name,
            personnel,
            fleet,
            bases,
            droids,
            resources: Resources(1000, 500, 100), // credits, fuel, intel
            mission_stats: MissionStats(0, 0, 0), // success, casualties, objectives
            game_phase: 1, // start with tatooine/death star plans phase
        }
    }
    
    // main game loop with different phases matching movie plot
    fn run_game(&mut self) {
        println!("========================================");
        println!("star wars episode iv: a new hope");
        println!("rebel alliance command center");
        println!("========================================");
        println!("welcome, commander {}", self.player_name);
        println!("the galaxy is in turmoil...");
        println!("the death star has destroyed alderaan");
        println!("princess leia's ship has been captured");
        println!("but the droids escaped with the plans...");
        
        loop {
            match self.game_phase {
                1 => {
                    println!("\n=== phase 1: rescue mission ===");
                    if self.phase_one_rescue_leia() {
                        self.game_phase = 2;
                    }
                },
                2 => {
                    println!("\n=== phase 2: death star plans analysis ===");
                    if self.phase_two_analyze_plans() {
                        self.game_phase = 3;
                    }
                },
                3 => {
                    println!("\n=== phase 3: prepare for battle ===");
                    if self.phase_three_prepare_assault() {
                        self.game_phase = 4;
                    }
                },
                4 => {
                    println!("\n=== phase 4: death star assault ===");
                    if self.phase_four_death_star_battle() {
                        println!("\n🌟 victory! the death star has been destroyed! 🌟");
                        println!("the rebel alliance has won a major victory!");
                        break;
                    } else {
                        println!("\n💀 defeat! the rebellion has been crushed! 💀");
                        break;
                    }
                },
                _ => break,
            }
            
            if !self.continue_game() {
                break;
            }
        }
        
        self.display_final_stats();
    }
    
    // phase 1: rescue princess leia from death star
    fn phase_one_rescue_leia(&mut self) -> bool {
        println!("r2-d2 and c-3po have delivered princess leia's message");
        println!("'help me obi-wan kenobi, you're my only hope'");
        println!("obi-wan reveals luke's destiny and the need to rescue leia");
        
        // find han solo for the rescue mission
        println!("\nseeking smuggler for rescue mission...");
        println!("found han solo and chewbacca at mos eisley cantina");
        
        // rescue mission mini-game
        println!("\ninfiltrating death star detention level...");
        println!("disguised as stormtroopers, the team locates princess leia");
        
        // combat encounter
        let mut success = true;
        for person in &mut self.personnel {
            if person.name == "luke skywalker" || person.name == "han solo" {
                if person.combat_skill < 70 {
                    println!("{} struggles in the firefight!", person.name);
                    success = false;
                }
            }
        }
        
        if success {
            println!("rescue successful! princess leia has been freed!");
            println!("the team escapes in the millennium falcon");
            
            // obi-wan's sacrifice
            println!("obi-wan kenobi sacrifices himself facing darth vader");
            if let Some(obi_wan) = self.personnel.iter_mut().find(|p| p.name == "obi-wan kenobi") {
                obi_wan.alive = false;
                println!("obi-wan becomes one with the force...");
            }
            
            // luke's force awakening
            if let Some(luke) = self.personnel.iter_mut().find(|p| p.name == "luke skywalker") {
                luke.force_sensitivity += 20;
                println!("luke feels the force calling to him");
            }
            
            true
        } else {
            println!("rescue mission failed! regroup and try again");
            false
        }
    }
    
    // phase 2: analyze death star plans and find weakness
    fn phase_two_analyze_plans(&mut self) -> bool {
        println!("arriving at yavin 4 rebel base...");
        println!("r2-d2 delivers the complete technical readouts");
        
        // transfer plans to base
        if let Some(r2d2) = self.droids.iter().find(|d| d.designation == "r2-d2") {
            if let Some(base) = self.bases.iter_mut().find(|b| b.name == "yavin 4 base") {
                base.receive_death_star_plans(r2d2);
            }
        }
        
        // analysis mini-game with general dodonna
        println!("\ngeneral dodonna: 'the plans show a weakness...'");
        println!("analyzing death star schematics...");
        
        // intelligence gathering game
        self.decode_imperial_transmissions();
        
        // planning session
        println!("\nrebel council convenes to plan attack");
        println!("target: thermal exhaust port, 2 meters wide");
        println!("'that's impossible, even for a computer!' - red leader");
        println!("'it's not impossible, i used to bulls-eye womp rats' - luke");
        
        // prepare luke for the mission
        if let Some(luke) = self.personnel.iter_mut().find(|p| p.name == "luke skywalker") {
            luke.train_abilities();
            println!("luke practices force meditation");
        }
        
        true
    }
    
    // phase 3: prepare rebel fleet for death star assault
    fn phase_three_prepare_assault(&mut self) -> bool {
        println!("preparing rebel fleet for death star assault...");
        
        // recruit more pilots and ships
        self.recruit_red_squadron();
        
        // briefing room scene
        println!("\n=== briefing room ===");
        println!("general dodonna explains the mission:");
        println!("'the empire's ultimate weapon must be destroyed'");
        println!("'you'll have to maneuver straight down this trench'");
        println!("'the target area is only two meters wide'");
        
        // ship maintenance
        println!("\ntechnicians prepare the x-wing fighters");
        for ship in &mut self.fleet {
            if ship.ship_class == "x-wing" {
                ship.repair_ship();
                println!("{} is fueled and ready", ship.designation);
            }
        }
        
        // last minute preparations
        println!("\nfinal preparations before departure...");
        println!("pilots suit up and conduct pre-flight checks");
        
        // han solo's departure and return
        println!("han solo leaves to pay off jabba the hutt");
        println!("'may the force be with you' - leia to luke");
        
        true
    }
    
    // phase 4: final battle - death star assault
    fn phase_four_death_star_battle(&mut self) -> bool {
        println!("red squadron launches from yavin 4!");
        println!("30 rebel ships vs the death star");
        
        // create death star as enemy
        let mut death_star = Starship::new(String::from("death star"), String::from("death star"));
        
        // battle simulation
        println!("\nengaging tie fighters around the death star...");
        
        // multiple attack runs
        let mut attack_runs = 0;
        let mut trench_run_successful = false;
        
        while attack_runs < 3 && !trench_run_successful {
            attack_runs += 1;
            println!("\n--- attack run {} ---", attack_runs);
            
            match attack_runs {
                1 => {
                    println!("red leader begins trench run...");
                    println!("'stay on target... stay on target...'");
                    println!("red leader is hit! his x-wing is destroyed!");
                },
                2 => {
                    println!("red ten and red twelve attempt the trench...");
                    println!("tie fighters close in from behind");
                    println!("'we lost red ten and red twelve!'");
                },
                3 => {
                    println!("luke skywalker makes the final run");
                    println!("'red five to red leader, i'm going in'");
                    
                    if let Some(luke) = self.personnel.iter().find(|p| p.name == "luke skywalker") {
                        // luke uses the force
                        println!("'use the force, luke' - obi-wan's voice");
                        if luke.use_force("thermal exhaust port") {
                            println!("luke switches off his targeting computer");
                            println!("'i have you now' - darth vader closing in");
                            
                            // han solo's last-minute rescue
                            println!("'yahoo! you're all clear, kid!' - han solo returns");
                            println!("han's attack scares off darth vader");
                            
                            // the winning shot
                            println!("luke fires proton torpedoes...");
                            println!("the torpedoes go straight into the exhaust port!");
                            println!("luke escapes just as the death star explodes!");
                            
                            trench_run_successful = true;
                            self.mission_stats.2 += 1; // objectives completed
                        } else {
                            println!("luke's shot misses the target");
                        }
                    }
                },
                _ => {},
            }
        }
        
        if trench_run_successful {
            println!("\n🌟 the death star explodes in a brilliant flash! 🌟");
            death_star.operational = false;
            self.mission_stats.0 += 1; // mission success
            true
        } else {
            println!("the assault has failed... yavin 4 will be destroyed");
            false
        }
    }
    
    // recruit additional pilots for red squadron
    fn recruit_red_squadron(&mut self) {
        println!("recruiting additional pilots for red squadron...");
        
        // add more x-wings and pilots
        self.fleet.push(Starship::new(String::from("red two"), String::from("x-wing")));
        self.fleet.push(Starship::new(String::from("red three"), String::from("x-wing")));
        self.fleet.push(Starship::new(String::from("red six"), String::from("x-wing")));
        
        // add pilots
        self.personnel.push(RebelPersonnel::new(String::from("biggs darklighter"), String::from("red three")));
        self.personnel.push(RebelPersonnel::new(String::from("wedge antilles"), String::from("red two")));
        
        println!("red squadron now at full strength");
    }
    
    // mini-game: decode imperial transmissions
    fn decode_imperial_transmissions(&mut self) {
        println!("\n=== imperial intelligence intercept ===");
        println!("c-3po has intercepted coded imperial transmissions");
        
        // create sample transmissions to decode
        let mut transmission1 = ImperialTransmission::new(
            String::from("RATS HTAED EHT FO REWOP EHT"), 
            8, 
            String::from("death star command")
        );
        
        let mut transmission2 = ImperialTransmission::new(
            String::from("DEKCATTA EB LLIW ESAB LEBR"), 
            9, 
            String::from("imperial fleet")
        );
        
        println!("decoding intercepted transmissions...");
        
        // decode the messages
        transmission1.decode_transmission();
        transmission2.decode_transmission();
        
        // analyze intelligence value
        let intel1 = transmission1.analyze_intel();
        let intel2 = transmission2.analyze_intel();
        
        // add to resources
        self.resources.2 += intel1 + intel2;
        
        println!("total intelligence gathered: {} points", intel1 + intel2);
        println!("rebel intelligence now has critical information");
    }
    
    // display character status
    fn display_personnel_status(&self) {
        println!("\n=== rebel personnel status ===");
        for person in &self.personnel {
            let status = if person.alive { "alive" } else { "kia" };
            println!("{} ({}): force: {}, combat: {}, leadership: {} - {} at {}", 
                person.name, 
                person.codename,
                person.force_sensitivity,
                person.combat_skill,
                person.leadership,
                status,
                person.current_location
            );
        }
    }
    
    // display fleet status
    fn display_fleet_status(&self) {
        println!("\n=== rebel fleet status ===");
        for ship in &self.fleet {
            let status = if ship.operational { "operational" } else { "destroyed" };
            println!("{} ({}): hull: {}, shields: {}, weapons: {} - {} in {}", 
                ship.designation,
                ship.ship_class,
                ship.hull_integrity,
                ship.shield_strength,
                ship.weapons_power,
                status,
                ship.current_sector
            );
        }
    }
    
    // display resource status
    fn display_resources(&self) {
        println!("\n=== resource status ===");
        println!("credits: {}", self.resources.0);
        println!("fuel: {}", self.resources.1);
        println!("intelligence: {}", self.resources.2);
    }
    
    // ask player to continue
    fn continue_game(&self) -> bool {
        println!("\npress enter to continue to next phase, or type 'quit' to exit");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
        let input = input.trim().to_lowercase();
        
        if input == "quit" || input == "q" {
            false
        } else {
            true
        }
    }
    
    // final game statistics
    fn display_final_stats(&self) {
        println!("\n========================================");
        println!("final mission report");
        println!("========================================");
        println!("commander: {}", self.player_name);
        println!("missions successful: {}", self.mission_stats.0);
        println!("casualties: {}", self.mission_stats.1);
        println!("objectives completed: {}", self.mission_stats.2);
        
        self.display_personnel_status();
        self.display_fleet_status();
        self.display_resources();
        
        println!("\nthanks for playing star wars: rebel alliance command!");
        println!("may the force be with you, always.");
    }
}

// main function - game entry point
fn main() {
    println!("enter your rebel commander name:");
    
    let mut commander_name = String::new();
    io::stdin()
        .read_line(&mut commander_name)
        .expect("failed to read commander name");
    
    let commander_name = commander_name.trim().to_string();
    
    // create and run the game
    let mut game = RebelAllianceCommand::new(commander_name);
    game.run_game();
}

