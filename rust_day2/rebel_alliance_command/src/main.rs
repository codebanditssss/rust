// star wars episode iv: rebel alliance command center
// interactive strategy game with real player choices and consequences

use std::io;
use std::io::Write;

// simplified game structs for better interaction
struct RebelCommander {
    name: String,
    reputation: u32,     // 0-100, affects available options
    force_points: u32,   // special ability points
    credits: u32,        // money for upgrades
    alive: bool,
}

struct GameState {
    commander: RebelCommander,
    ships_available: u32,     // number of ships in fleet
    pilots_available: u32,    // number of pilots
    death_star_plans: bool,   // key plot item
    leia_rescued: bool,       // mission status
    obi_wan_alive: bool,      // affects force training
    current_phase: u32,       // 1-4 game progression
    game_over: bool,
}

impl RebelCommander {
    fn new(name: String) -> RebelCommander {
        RebelCommander {
            name,
            reputation: 50,     // start with average reputation
            force_points: 0,    // earn through choices
            credits: 100,       // starting money
            alive: true,
        }
    }
    
    // gain reputation from good decisions
    fn gain_reputation(&mut self, amount: u32) {
        self.reputation = (self.reputation + amount).min(100);
        println!("📈 reputation increased by {}! current: {}/100", amount, self.reputation);
    }
    
    // lose reputation from bad decisions
    fn lose_reputation(&mut self, amount: u32) {
        self.reputation = self.reputation.saturating_sub(amount);
        println!("📉 reputation decreased by {}! current: {}/100", amount, self.reputation);
    }
    
    // earn force points through training
    fn gain_force_points(&mut self, amount: u32) {
        self.force_points += amount;
        println!("✨ gained {} force points! total: {}", amount, self.force_points);
    }
    
    // spend credits on upgrades
    fn spend_credits(&mut self, amount: u32) -> bool {
        if self.credits >= amount {
            self.credits -= amount;
            println!("💰 spent {} credits. remaining: {}", amount, self.credits);
            true
        } else {
            println!("❌ not enough credits! need: {}, have: {}", amount, self.credits);
            false
        }
    }
}

impl GameState {
    fn new(commander_name: String) -> GameState {
        GameState {
            commander: RebelCommander::new(commander_name),
            ships_available: 5,      // start with small fleet
            pilots_available: 8,     // more pilots than ships
            death_star_plans: false,
            leia_rescued: false,
            obi_wan_alive: true,
            current_phase: 1,
            game_over: false,
        }
    }
    
    // main game loop with real interactions
    fn run_game(&mut self) {
        self.show_title_and_instructions();
        
        while !self.game_over {
            self.show_status();
            
            match self.current_phase {
                1 => self.phase_rescue_mission(),
                2 => self.phase_decode_plans(), 
                3 => self.phase_prepare_assault(),
                4 => self.phase_death_star_battle(),
                _ => {
                    println!("🌟 congratulations! you've completed the rebellion! 🌟");
                    self.game_over = true;
                }
            }
            
            // check if commander died
            if !self.commander.alive {
                println!("💀 game over! the rebellion has lost its commander!");
                self.game_over = true;
            }
        }
        
        self.show_final_results();
    }
    
    // show game instructions and controls
    fn show_title_and_instructions(&self) {
        println!("╔════════════════════════════════════════════╗");
        println!("║        STAR WARS: REBEL COMMAND           ║");
        println!("║         Episode IV: A New Hope            ║");
        println!("╚════════════════════════════════════════════╝");
        println!();
        println!("🎮 HOW TO PLAY:");
        println!("• make strategic decisions by typing numbers (1, 2, 3, etc.)");
        println!("• your choices affect reputation, resources, and story outcome");
        println!("• high reputation unlocks better options");
        println!("• collect force points for special abilities");
        println!("• manage credits to buy upgrades");
        println!("• complete all 4 phases to destroy the death star!");
        println!();
        println!("⚠️  WARNING: bad decisions can kill your commander!");
        println!();
        self.wait_for_enter();
    }
    
    // show current game status
    fn show_status(&self) {
        println!("\n╔═══════════ COMMAND STATUS ═══════════╗");
        println!("║ commander: {:<26} ║", self.commander.name);
        println!("║ reputation: {:<25} ║", format!("{}/100", self.commander.reputation));
        println!("║ force points: {:<23} ║", self.commander.force_points);
        println!("║ credits: {:<28} ║", self.commander.credits);
        println!("║ ships: {:<30} ║", self.ships_available);
        println!("║ pilots: {:<29} ║", self.pilots_available);
        println!("║ phase: {:<30} ║", format!("{}/4", self.current_phase));
        println!("╚═══════════════════════════════════════╝");
        println!();
    }
    
    // phase 1: rescue princess leia
    fn phase_rescue_mission(&mut self) {
        println!("🚀 PHASE 1: RESCUE PRINCESS LEIA");
        println!("the death star has captured princess leia!");
        println!("r2-d2 and c-3po have escaped with secret plans.");
        println!("you must decide how to rescue the princess...");
        println!();
        
        loop {
            println!("choose your rescue strategy:");
            println!("1. 🎭 disguise as stormtroopers (risky but stealthy)");
            println!("2. 💰 hire smugglers han solo & chewbacca (costs 50 credits)");
            println!("3. ⚔️  direct assault on death star (requires high reputation)");
            println!("4. 📊 check detailed mission intel first");
            
            match self.get_player_choice() {
                1 => {
                    println!("\n🎭 disguising as stormtroopers...");
                    if self.commander.reputation >= 40 {
                        println!("✅ mission success! your reputation helped maintain the disguise.");
                        println!("princess leia rescued! but obi-wan sacrifices himself...");
                        self.leia_rescued = true;
                        self.obi_wan_alive = false;
                        self.commander.gain_reputation(20);
                        self.commander.gain_force_points(10);
                        self.current_phase = 2;
                        break;
                    } else {
                        println!("❌ disguise failed! low reputation made guards suspicious.");
                        println!("you barely escape but leia remains captured.");
                        self.commander.lose_reputation(10);
                        continue;
                    }
                },
                2 => {
                    if self.commander.spend_credits(50) {
                        println!("\n💰 han solo: 'i've got a bad feeling about this...'");
                        println!("✅ smugglers successfully extract princess leia!");
                        println!("millennium falcon escapes with everyone alive!");
                        self.leia_rescued = true;
                        self.ships_available += 1; // millennium falcon joins fleet
                        self.commander.gain_reputation(15);
                        self.current_phase = 2;
                        break;
                    } else {
                        continue; // not enough credits, try again
                    }
                },
                3 => {
                    if self.commander.reputation >= 70 {
                        println!("\n⚔️ launching full rebel assault on death star!");
                        println!("✅ your legendary reputation inspires the fleet!");
                        println!("massive battle but leia is rescued!");
                        self.leia_rescued = true;
                        self.ships_available -= 2; // lost ships in battle
                        self.pilots_available -= 3; // casualties
                        self.commander.gain_reputation(30);
                        self.current_phase = 2;
                        break;
                    } else {
                        println!("❌ insufficient reputation for assault! rebels refuse dangerous mission.");
                        println!("build your reputation with successful smaller missions first.");
                        continue;
                    }
                },
                4 => {
                    println!("\n📊 MISSION INTEL:");
                    println!("• death star defenses: extremely heavy");
                    println!("• imperial presence: maximum alert");
                    println!("• success probability: depends on your reputation and strategy");
                    println!("• stealth missions work better with high reputation");
                    println!("• assault missions need very high reputation (70+)");
                    println!("• hired help is expensive but reliable");
                    println!();
                    continue;
                },
                _ => {
                    println!("❌ invalid choice! enter 1, 2, 3, or 4");
                    continue;
                }
            }
        }
    }
    
    // phase 2: decode death star plans
    fn phase_decode_plans(&mut self) {
        println!("\n🔍 PHASE 2: DECODE DEATH STAR PLANS");
        
        if !self.leia_rescued {
            println!("❌ without princess leia, the rebels have no access to the plans!");
            println!("💀 the death star destroys yavin 4 base! game over!");
            self.commander.alive = false;
            return;
        }
        
        println!("r2-d2 has the complete death star technical readouts!");
        println!("but the data is encrypted with imperial codes...");
        println!();
        
        loop {
            println!("how do you want to decode the plans?");
            println!("1. 🤖 use c-3po's protocol skills (slow but safe)");
            println!("2. 🧠 use force meditation to understand them (costs 5 force points)");
            println!("3. 💻 hire rebel technicians (costs 30 credits)");
            println!("4. ⏰ rush the analysis (risky but fast)");
            
            match self.get_player_choice() {
                1 => {
                    println!("\n🤖 c-3po: 'the probability of successfully decoding this is 3,720 to 1!'");
                    println!("✅ slow but successful! plans fully decoded!");
                    println!("discovered weakness: thermal exhaust port!");
                    self.death_star_plans = true;
                    self.commander.gain_reputation(10);
                    self.current_phase = 3;
                    break;
                },
                2 => {
                    if self.commander.force_points >= 5 {
                        self.commander.force_points -= 5;
                        println!("\n🧠 using the force to understand imperial technology...");
                        if self.obi_wan_alive {
                            println!("✅ obi-wan guides your meditation! perfect understanding!");
                            println!("bonus insight: discovered secondary weakness!");
                            self.death_star_plans = true;
                            self.commander.gain_reputation(25);
                            self.commander.gain_force_points(10); // net gain
                        } else {
                            println!("✅ obi-wan's spirit helps from beyond! plans decoded!");
                            self.death_star_plans = true;
                            self.commander.gain_reputation(15);
                        }
                        self.current_phase = 3;
                        break;
                    } else {
                        println!("❌ insufficient force points! you need at least 5.");
                        continue;
                    }
                },
                3 => {
                    if self.commander.spend_credits(30) {
                        println!("\n💻 rebel technicians working around the clock...");
                        println!("✅ expert analysis complete! weakness identified!");
                        self.death_star_plans = true;
                        self.ships_available += 1; // technicians upgrade a ship
                        self.current_phase = 3;
                        break;
                    } else {
                        continue;
                    }
                },
                4 => {
                    println!("\n⏰ rushing the analysis with limited time...");
                    if self.commander.reputation >= 60 {
                        println!("✅ your reputation attracts the best analysts!");
                        println!("quick but accurate decode! weakness found!");
                        self.death_star_plans = true;
                        self.current_phase = 3;
                        break;
                    } else {
                        println!("❌ rushed analysis produces incomplete data!");
                        println!("you'll attack the death star without full intel...");
                        self.death_star_plans = false; // incomplete plans
                        self.commander.lose_reputation(15);
                        self.current_phase = 3;
                        break;
                    }
                }
                _ => {
                    println!("❌ invalid choice! enter 1, 2, 3, or 4");
                    continue;
                }
            }
        }
    }
    
    // phase 3: prepare for final assault
    fn phase_prepare_assault(&mut self) {
        println!("\n⚔️ PHASE 3: PREPARE FOR BATTLE");
        println!("the death star is approaching yavin 4!");
        println!("you have limited time to prepare the final assault...");
        println!();
        
        // give player multiple preparation choices
        let mut preparations_made = 0;
        
        while preparations_made < 2 {
            println!("preparation options (choose 2):");
            println!("1. 🎓 train pilots in combat maneuvers");
            println!("2. 🔧 upgrade ship weapons and shields");
            println!("3. 👥 recruit additional volunteers");
            println!("4. 🧘 meditation and force training");
            println!("5. 📋 review attack strategy");
            
            match self.get_player_choice() {
                1 => {
                    if preparations_made < 2 {
                        println!("\n🎓 intensive pilot training session...");
                        self.pilots_available += 2;
                        println!("✅ pilot skills improved! +2 experienced pilots");
                        preparations_made += 1;
                    } else {
                        println!("you've already made your preparations!");
                    }
                },
                2 => {
                    if preparations_made < 2 && self.commander.spend_credits(40) {
                        println!("\n🔧 upgrading x-wing fighters...");
                        self.ships_available += 1; // upgraded ship counts as extra
                        println!("✅ weapons upgraded! fleet effectiveness increased");
                        preparations_made += 1;
                    } else if preparations_made >= 2 {
                        println!("you've already made your preparations!");
                    }
                },
                3 => {
                    if preparations_made < 2 {
                        println!("\n👥 recruiting brave volunteers...");
                        if self.commander.reputation >= 50 {
                            self.pilots_available += 3;
                            println!("✅ your reputation attracts many volunteers! +3 pilots");
                        } else {
                            self.pilots_available += 1;
                            println!("✅ some volunteers join! +1 pilot");
                        }
                        preparations_made += 1;
                    } else {
                        println!("you've already made your preparations!");
                    }
                },
                4 => {
                    if preparations_made < 2 {
                        println!("\n🧘 force meditation and spiritual preparation...");
                        self.commander.gain_force_points(15);
                        if self.obi_wan_alive {
                            println!("✅ obi-wan provides advanced training!");
                            self.commander.gain_force_points(10);
                        } else {
                            println!("✅ you feel obi-wan's presence guiding you!");
                        }
                        preparations_made += 1;
                    } else {
                        println!("you've already made your preparations!");
                    }
                },
                5 => {
                    println!("\n📋 CURRENT BATTLE READINESS:");
                    println!("• ships available: {}", self.ships_available);
                    println!("• pilots available: {}", self.pilots_available);
                    println!("• death star plans: {}", if self.death_star_plans { "✅ complete" } else { "❌ incomplete" });
                    println!("• commander reputation: {}/100", self.commander.reputation);
                    println!("• force points: {}", self.commander.force_points);
                    println!();
                    continue;
                },
                _ => {
                    println!("❌ invalid choice! enter 1, 2, 3, 4, or 5");
                    continue;
                }
            }
        }
        
        println!("\n✅ preparations complete! ready for final assault!");
        self.current_phase = 4;
    }
    
    // phase 4: death star battle with multiple outcomes
    fn phase_death_star_battle(&mut self) {
        println!("\n💥 PHASE 4: DEATH STAR ASSAULT");
        println!("red squadron launches for the final battle!");
        println!("the fate of the rebellion rests in your hands...");
        println!();
        
        // calculate success probability based on preparations
        let mut success_chance = 30; // base chance
        
        if self.death_star_plans { success_chance += 30; }
        if self.ships_available >= 8 { success_chance += 20; }
        if self.pilots_available >= 12 { success_chance += 15; }
        if self.commander.reputation >= 70 { success_chance += 20; }
        
        println!("📊 mission success probability: {}%", success_chance.min(95));
        println!();
        
        loop {
            println!("final battle strategy:");
            println!("1. 🎯 precise targeting run (uses death star plans)");
            println!("2. ✨ trust in the force (uses force points)");
            println!("3. 🚀 massive coordinated assault (uses all ships)");
            println!("4. 🎲 desperate last chance (high risk/reward)");
            
            match self.get_player_choice() {
                1 => {
                    if self.death_star_plans {
                        println!("\n🎯 following the technical readouts exactly...");
                        println!("targeting the thermal exhaust port...");
                        
                        if success_chance >= 60 {
                            println!("✅ direct hit! the death star explodes!");
                            println!("🌟 VICTORY! the rebellion succeeds!");
                            self.commander.gain_reputation(50);
                            self.victory_ending();
                            return;
                        } else {
                            println!("❌ shot missed the exhaust port!");
                            println!("death star destroys yavin 4! defeat!");
                            self.defeat_ending();
                            return;
                        }
                    } else {
                        println!("❌ you don't have complete death star plans!");
                        continue;
                    }
                },
                2 => {
                    if self.commander.force_points >= 20 {
                        println!("\n✨ 'use the force, luke...' - obi-wan's voice");
                        println!("trusting in the force for the impossible shot...");
                        
                        // force always succeeds if you have enough points
                        println!("✅ the force guides your shot perfectly!");
                        println!("🌟 INCREDIBLE VICTORY! the death star is destroyed!");
                        self.commander.gain_reputation(75);
                        self.force_victory_ending();
                        return;
                    } else {
                        println!("❌ insufficient force points! you need at least 20.");
                        continue;
                    }
                },
                3 => {
                    if self.ships_available >= 6 && self.pilots_available >= 10 {
                        println!("\n🚀 launching all available fighters...");
                        println!("coordinated assault on multiple targets...");
                        
                        if success_chance >= 50 {
                            println!("✅ overwhelming firepower succeeds!");
                            println!("🌟 HARD-FOUGHT VICTORY!");
                            println!("but many brave pilots were lost in the battle...");
                            self.pilots_available -= 5;
                            self.ships_available -= 3;
                            self.costly_victory_ending();
                            return;
                        } else {
                            println!("❌ not enough firepower to penetrate defenses!");
                            self.defeat_ending();
                            return;
                        }
                    } else {
                        println!("❌ insufficient ships or pilots for massive assault!");
                        println!("need at least 6 ships and 10 pilots");
                        continue;
                    }
                },
                4 => {
                    println!("\n🎲 desperate kamikaze run at the reactor core...");
                    println!("this is extremely dangerous but could work...");
                    
                    // 50/50 chance regardless of preparation
                    if self.commander.reputation % 2 == 0 { // simple random based on reputation
                        println!("✅ miraculous success! sacrifice saves the rebellion!");
                        println!("🌟 HEROIC VICTORY AT GREAT COST!");
                        self.commander.alive = false; // commander sacrifices themselves
                        self.heroic_sacrifice_ending();
                        return;
                    } else {
                        println!("❌ desperate attack fails! all is lost!");
                        self.defeat_ending();
                        return;
                    }
                },
                _ => {
                    println!("❌ invalid choice! enter 1, 2, 3, or 4");
                    continue;
                }
            }
        }
    }
    
    // different ending scenarios
    fn victory_ending(&mut self) {
        println!("\n🎉 PERFECT VICTORY! 🎉");
        println!("the death star explodes in a brilliant flash!");
        println!("yavin 4 is saved and the rebellion lives on!");
        println!("you are hailed as a hero of the galaxy!");
        self.game_over = true;
    }
    
    fn force_victory_ending(&mut self) {
        println!("\n🌟 LEGENDARY FORCE VICTORY! 🌟");
        println!("your connection to the force guided the impossible shot!");
        println!("you have become a true jedi knight!");
        println!("the galaxy will remember this day forever!");
        self.game_over = true;
    }
    
    fn costly_victory_ending(&mut self) {
        println!("\n⚔️ COSTLY VICTORY ⚔️");
        println!("the death star is destroyed but at great cost...");
        println!("many brave pilots gave their lives for freedom");
        println!("their sacrifice will never be forgotten");
        self.game_over = true;
    }
    
    fn heroic_sacrifice_ending(&mut self) {
        println!("\n🏆 HEROIC SACRIFICE 🏆");
        println!("your ultimate sacrifice saved the entire rebellion!");
        println!("you will be remembered as the greatest hero of the war!");
        println!("statues are built in your honor across the galaxy!");
        self.game_over = true;
    }
    
    fn defeat_ending(&mut self) {
        println!("\n💀 DEFEAT 💀");
        println!("the death star remains operational...");
        println!("yavin 4 is destroyed and the rebellion is crushed");
        println!("the empire's tyranny continues across the galaxy");
        self.commander.alive = false;
        self.game_over = true;
    }
    
    // get player input with validation
    fn get_player_choice(&self) -> u32 {
        loop {
            print!("👉 enter your choice: ");
            io::stdout().flush().expect("failed to flush stdout");
            
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("failed to read input");
            
            match input.trim().parse::<u32>() {
                Ok(choice) => return choice,
                Err(_) => println!("❌ please enter a valid number!"),
            }
        }
    }
    
    // wait for enter key
    fn wait_for_enter(&self) {
        println!("press enter to continue...");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read input");
    }
    
    // show final game statistics
    fn show_final_results(&self) {
        println!("\n╔══════════ FINAL REPORT ══════════╗");
        println!("║ commander: {:<23} ║", self.commander.name);
        println!("║ final reputation: {:<15} ║", format!("{}/100", self.commander.reputation));
        println!("║ force mastery: {:<18} ║", format!("{} points", self.commander.force_points));
        println!("║ credits remaining: {:<15} ║", self.commander.credits);
        println!("║ ships saved: {:<20} ║", self.ships_available);
        println!("║ pilots survived: {:<16} ║", self.pilots_available);
        println!("║ leia rescued: {:<19} ║", if self.leia_rescued { "✅ yes" } else { "❌ no" });
        println!("║ plans decoded: {:<18} ║", if self.death_star_plans { "✅ yes" } else { "❌ no" });
        println!("╚═══════════════════════════════════╝");
        
        if self.commander.alive {
            println!("\n🎖️  you survived to fight another day!");
        } else {
            println!("\n⚰️  you gave your life for the rebellion");
        }
        
        println!("\nthanks for playing star wars: rebel command!");
        println!("may the force be with you, always! ✨");
    }
}

fn main() {
    println!("enter your rebel commander name:");
    print!("👤 name: ");
    io::stdout().flush().expect("failed to flush stdout");
    
    let mut commander_name = String::new();
    io::stdin().read_line(&mut commander_name).expect("failed to read input");
    let commander_name = commander_name.trim().to_string();
    
    if commander_name.is_empty() {
        println!("using default name: luke skywalker");
        let mut game = GameState::new(String::from("luke skywalker"));
        game.run_game();
    } else {
        let mut game = GameState::new(commander_name);
        game.run_game();
    }
}
