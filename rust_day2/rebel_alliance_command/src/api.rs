// web api for star wars rebel alliance command center
// converts the game logic to rest endpoints for typescript frontend

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use warp::Filter;

// import our game logic
use crate::GameState;

// api data structures for json communication
#[derive(Serialize, Deserialize, Clone)]
pub struct ApiGameState {
    pub game_id: String,
    pub commander_name: String,
    pub reputation: u32,
    pub force_points: u32,
    pub credits: u32,
    pub ships_available: u32,
    pub pilots_available: u32,
    pub current_phase: u32,
    pub leia_rescued: bool,
    pub death_star_plans: bool,
    pub obi_wan_alive: bool,
    pub game_over: bool,
    pub preparations_made: u32,
    pub current_options: Vec<GameOption>,
    pub phase_description: String,
    pub last_action_result: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameOption {
    pub id: u32,
    pub text: String,
    pub description: String,
    pub icon: String,
    pub cost: Option<u32>,
    pub requirement: Option<String>,
    pub available: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CreateGameRequest {
    pub commander_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MakeChoiceRequest {
    pub choice: u32,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

// in-memory game storage (in production, use a database)
type GameStorage = Arc<Mutex<HashMap<String, GameState>>>;

impl From<&GameState> for ApiGameState {
    fn from(game: &GameState) -> Self {
        let (options, description) = get_phase_options_and_description(game);
        
        ApiGameState {
            game_id: "current".to_string(), // simplified for demo
            commander_name: game.commander.name.clone(),
            reputation: game.commander.reputation,
            force_points: game.commander.force_points,
            credits: game.commander.credits,
            ships_available: game.ships_available,
            pilots_available: game.pilots_available,
            current_phase: game.current_phase,
            leia_rescued: game.leia_rescued,
            death_star_plans: game.death_star_plans,
            obi_wan_alive: game.obi_wan_alive,
            game_over: game.game_over,
            preparations_made: game.preparations_made,
            current_options: options,
            phase_description: description,
            last_action_result: None,
        }
    }
}

// generate current phase options for frontend
fn get_phase_options_and_description(game: &GameState) -> (Vec<GameOption>, String) {
    match game.current_phase {
        1 => get_phase_1_options(game),
        2 => get_phase_2_options(game),
        3 => get_phase_3_options(game),
        4 => get_phase_4_options(game),
        _ => (vec![], "game completed".to_string()),
    }
}

fn get_phase_1_options(game: &GameState) -> (Vec<GameOption>, String) {
    let description = "🚀 PHASE 1: RESCUE PRINCESS LEIA\n\nThe Death Star has captured Princess Leia! R2-D2 and C-3PO have escaped with secret plans. You must decide how to rescue the princess...".to_string();
    
    let options = vec![
        GameOption {
            id: 1,
            text: "Disguise as Stormtroopers".to_string(),
            description: "Risky but stealthy infiltration".to_string(),
            icon: "🎭".to_string(),
            cost: None,
            requirement: Some("40+ reputation recommended".to_string()),
            available: true,
        },
        GameOption {
            id: 2,
            text: "Hire Han Solo & Chewbacca".to_string(),
            description: "Expensive but reliable smugglers".to_string(),
            icon: "💰".to_string(),
            cost: Some(50),
            requirement: None,
            available: game.commander.credits >= 50,
        },
        GameOption {
            id: 3,
            text: "Direct Assault on Death Star".to_string(),
            description: "High-risk military operation".to_string(),
            icon: "⚔️".to_string(),
            cost: None,
            requirement: Some("70+ reputation required".to_string()),
            available: game.commander.reputation >= 70,
        },
        GameOption {
            id: 4,
            text: "Check Mission Intel".to_string(),
            description: "Review strategic information".to_string(),
            icon: "📊".to_string(),
            cost: None,
            requirement: None,
            available: true,
        },
    ];
    
    (options, description)
}

fn get_phase_2_options(game: &GameState) -> (Vec<GameOption>, String) {
    if !game.leia_rescued {
        return (vec![], "❌ Mission Failed: Without Princess Leia, the Death Star plans cannot be accessed!".to_string());
    }
    
    let description = "🔍 PHASE 2: DECODE DEATH STAR PLANS\n\nR2-D2 has the complete Death Star technical readouts, but the data is encrypted with Imperial codes...".to_string();
    
    let options = vec![
        GameOption {
            id: 1,
            text: "Use C-3PO's Protocol Skills".to_string(),
            description: "Slow but safe decoding method".to_string(),
            icon: "🤖".to_string(),
            cost: None,
            requirement: None,
            available: true,
        },
        GameOption {
            id: 2,
            text: "Force Meditation".to_string(),
            description: "Use the Force to understand the plans".to_string(),
            icon: "🧠".to_string(),
            cost: Some(5), // force points cost
            requirement: Some("5 Force Points required".to_string()),
            available: game.commander.force_points >= 5,
        },
        GameOption {
            id: 3,
            text: "Hire Rebel Technicians".to_string(),
            description: "Professional analysis team".to_string(),
            icon: "💻".to_string(),
            cost: Some(30),
            requirement: None,
            available: game.commander.credits >= 30,
        },
        GameOption {
            id: 4,
            text: "Rush the Analysis".to_string(),
            description: "Quick but risky decode attempt".to_string(),
            icon: "⏰".to_string(),
            cost: None,
            requirement: Some("60+ reputation recommended".to_string()),
            available: true,
        },
    ];
    
    (options, description)
}

fn get_phase_3_options(game: &GameState) -> (Vec<GameOption>, String) {
    let description = format!("⚔️ PHASE 3: PREPARE FOR BATTLE\n\nThe Death Star is approaching Yavin 4! You have limited time to prepare the final assault. Choose 2 preparations:\n\n📊 Preparations Complete: {}/2", game.preparations_made);
    
    let options = vec![
        GameOption {
            id: 1,
            text: "Train Pilots".to_string(),
            description: "Intensive combat training session".to_string(),
            icon: "🎓".to_string(),
            cost: None,
            requirement: None,
            available: true,
        },
        GameOption {
            id: 2,
            text: "Upgrade Ships".to_string(),
            description: "Enhance weapons and shields".to_string(),
            icon: "🔧".to_string(),
            cost: Some(40),
            requirement: None,
            available: true,
        },
        GameOption {
            id: 3,
            text: "Recruit Volunteers".to_string(),
            description: "Find brave pilots to join".to_string(),
            icon: "👥".to_string(),
            cost: None,
            requirement: None,
            available: true,
        },
        GameOption {
            id: 4,
            text: "Force Training".to_string(),
            description: "Meditation and spiritual preparation".to_string(),
            icon: "🧘".to_string(),
            cost: None,
            requirement: None,
            available: true,
        },
        GameOption {
            id: 5,
            text: "Review Strategy".to_string(),
            description: "Check current battle readiness".to_string(),
            icon: "📋".to_string(),
            cost: None,
            requirement: None,
            available: true,
        },
    ];
    
    (options, description)
}

fn get_phase_4_options(game: &GameState) -> (Vec<GameOption>, String) {
    // calculate success probability
    let mut success_chance = 30;
    if game.death_star_plans { success_chance += 30; }
    if game.ships_available >= 8 { success_chance += 20; }
    if game.pilots_available >= 12 { success_chance += 15; }
    if game.commander.reputation >= 70 { success_chance += 20; }
    
    let description = format!("💥 PHASE 4: DEATH STAR ASSAULT\n\nRed Squadron launches for the final battle! The fate of the rebellion rests in your hands...\n\n📊 Mission Success Probability: {}%", success_chance.min(95));
    
    let options = vec![
        GameOption {
            id: 1,
            text: "Precision Targeting Run".to_string(),
            description: "Follow Death Star plans exactly".to_string(),
            icon: "🎯".to_string(),
            cost: None,
            requirement: Some("Complete Death Star plans required".to_string()),
            available: game.death_star_plans,
        },
        GameOption {
            id: 2,
            text: "Trust in the Force".to_string(),
            description: "Use the Force for the impossible shot".to_string(),
            icon: "✨".to_string(),
            cost: Some(20), // force points
            requirement: Some("20 Force Points required".to_string()),
            available: game.commander.force_points >= 20,
        },
        GameOption {
            id: 3,
            text: "Massive Coordinated Assault".to_string(),
            description: "Launch all available fighters".to_string(),
            icon: "🚀".to_string(),
            cost: None,
            requirement: Some("6+ ships and 10+ pilots required".to_string()),
            available: game.ships_available >= 6 && game.pilots_available >= 10,
        },
        GameOption {
            id: 4,
            text: "Desperate Last Chance".to_string(),
            description: "High-risk kamikaze run".to_string(),
            icon: "🎲".to_string(),
            cost: None,
            requirement: Some("50/50 chance regardless of preparation".to_string()),
            available: true,
        },
    ];
    
    (options, description)
}

// api endpoint handlers
pub async fn create_game(
    request: CreateGameRequest,
    storage: GameStorage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let game_id = "current".to_string(); // simplified for demo
    let game = GameState::new(request.commander_name);
    
    // store game in memory
    {
        let mut games = storage.lock().unwrap();
        games.insert(game_id.clone(), game.clone());
    }
    
    let api_state = ApiGameState::from(&game);
    let response = ApiResponse {
        success: true,
        data: Some(api_state),
        error: None,
    };
    
    Ok(warp::reply::json(&response))
}

pub async fn get_game_state(
    game_id: String,
    storage: GameStorage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let games = storage.lock().unwrap();
    
    match games.get(&game_id) {
        Some(game) => {
            let api_state = ApiGameState::from(game);
            let response = ApiResponse {
                success: true,
                data: Some(api_state),
                error: None,
            };
            Ok(warp::reply::json(&response))
        }
        None => {
            let response: ApiResponse<ApiGameState> = ApiResponse {
                success: false,
                data: None,
                error: Some("Game not found".to_string()),
            };
            Ok(warp::reply::json(&response))
        }
    }
}

pub async fn make_choice(
    game_id: String,
    request: MakeChoiceRequest,
    storage: GameStorage,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut games = storage.lock().unwrap();
    
    match games.get_mut(&game_id) {
        Some(game) => {
            // process the choice using existing game logic
            let result = process_game_choice(game, request.choice);
            
            let mut api_state = ApiGameState::from(&*game);
            api_state.last_action_result = Some(result);
            
            let response = ApiResponse {
                success: true,
                data: Some(api_state),
                error: None,
            };
            
            Ok(warp::reply::json(&response))
        }
        None => {
            let response: ApiResponse<ApiGameState> = ApiResponse {
                success: false,
                data: None,
                error: Some("Game not found".to_string()),
            };
            Ok(warp::reply::json(&response))
        }
    }
}

// process player choice using existing game logic
fn process_game_choice(game: &mut GameState, choice: u32) -> String {
    match game.current_phase {
        1 => process_phase_1_choice(game, choice),
        2 => process_phase_2_choice(game, choice),
        3 => process_phase_3_choice(game, choice),
        4 => process_phase_4_choice(game, choice),
        _ => "Invalid phase".to_string(),
    }
}

fn process_phase_1_choice(game: &mut GameState, choice: u32) -> String {
    match choice {
        1 => {
            if game.commander.reputation >= 40 {
                game.leia_rescued = true;
                game.obi_wan_alive = false;
                game.commander.gain_reputation(20);
                game.commander.gain_force_points(10);
                game.current_phase = 2;
                "✅ Mission success! Princess Leia rescued, but Obi-Wan sacrifices himself...".to_string()
            } else {
                game.commander.lose_reputation(10);
                "❌ Disguise failed! Low reputation made guards suspicious.".to_string()
            }
        },
        2 => {
            if game.commander.spend_credits(50) {
                game.leia_rescued = true;
                game.ships_available += 1;
                game.commander.gain_reputation(15);
                game.current_phase = 2;
                "✅ Han Solo: 'I've got a bad feeling about this...' Mission successful!".to_string()
            } else {
                "❌ Not enough credits to hire smugglers!".to_string()
            }
        },
        3 => {
            if game.commander.reputation >= 70 {
                game.leia_rescued = true;
                game.ships_available -= 2;
                game.pilots_available -= 3;
                game.commander.gain_reputation(30);
                game.current_phase = 2;
                "✅ Massive battle but Leia is rescued! Heavy casualties sustained.".to_string()
            } else {
                "❌ Insufficient reputation! Rebels refuse dangerous mission.".to_string()
            }
        },
        4 => {
            "📊 Intel gathered. Death Star defenses are extremely heavy. Success depends on reputation and strategy.".to_string()
        },
        _ => "Invalid choice for Phase 1".to_string(),
    }
}

fn process_phase_2_choice(game: &mut GameState, choice: u32) -> String {
    if !game.leia_rescued {
        game.commander.alive = false;
        return "💀 Without Princess Leia, the Death Star destroys Yavin 4! Game Over!".to_string();
    }
    
    match choice {
        1 => {
            game.death_star_plans = true;
            game.commander.gain_reputation(10);
            game.current_phase = 3;
            "✅ C-3PO successfully decodes the plans! Weakness discovered: thermal exhaust port!".to_string()
        },
        2 => {
            if game.commander.force_points >= 5 {
                game.commander.force_points -= 5;
                game.death_star_plans = true;
                game.current_phase = 3;
                if game.obi_wan_alive {
                    game.commander.gain_reputation(25);
                    game.commander.gain_force_points(10);
                    "✅ Obi-Wan guides your meditation! Perfect understanding achieved!".to_string()
                } else {
                    game.commander.gain_reputation(15);
                    "✅ Obi-Wan's spirit helps from beyond! Plans decoded!".to_string()
                }
            } else {
                "❌ Insufficient Force Points! You need at least 5.".to_string()
            }
        },
        3 => {
            if game.commander.spend_credits(30) {
                game.death_star_plans = true;
                game.ships_available += 1;
                game.current_phase = 3;
                "✅ Expert analysis complete! Weakness identified and ship upgraded!".to_string()
            } else {
                "❌ Not enough credits for technicians!".to_string()
            }
        },
        4 => {
            if game.commander.reputation >= 60 {
                game.death_star_plans = true;
                game.current_phase = 3;
                "✅ Quick but accurate decode! Your reputation attracted the best analysts!".to_string()
            } else {
                game.death_star_plans = false;
                game.commander.lose_reputation(15);
                game.current_phase = 3;
                "❌ Rushed analysis produced incomplete data! You'll attack without full intel...".to_string()
            }
        },
        _ => "Invalid choice for Phase 2".to_string(),
    }
}

fn process_phase_3_choice(game: &mut GameState, choice: u32) -> String {
    // Phase 3 requires exactly 2 preparations before advancing to Phase 4
    if game.preparations_made >= 2 && choice != 5 {
        return "✅ All preparations complete! Ready for final assault.".to_string();
    }
    
    let result = match choice {
        1 => {
            if game.preparations_made < 2 {
                game.pilots_available += 2;
                game.preparations_made += 1;
                format!("✅ Pilot training complete! +2 experienced pilots gained. (Preparation {}/2)", game.preparations_made)
            } else {
                "✅ Training already completed!".to_string()
            }
        },
        2 => {
            if game.preparations_made < 2 && game.commander.spend_credits(40) {
                game.ships_available += 1;
                game.preparations_made += 1;
                format!("✅ Ship upgrades complete! Fleet effectiveness increased. (Preparation {}/2)", game.preparations_made)
            } else if game.preparations_made >= 2 {
                "✅ Upgrades already completed!".to_string()
            } else {
                "❌ Not enough credits for upgrades!".to_string()
            }
        },
        3 => {
            if game.preparations_made < 2 {
                if game.commander.reputation >= 50 {
                    game.pilots_available += 3;
                } else {
                    game.pilots_available += 1;
                }
                game.preparations_made += 1;
                format!("✅ Volunteers recruited! (Preparation {}/2)", game.preparations_made)
            } else {
                "✅ Recruitment already completed!".to_string()
            }
        },
        4 => {
            if game.preparations_made < 2 {
                game.commander.gain_force_points(15);
                if game.obi_wan_alive {
                    game.commander.gain_force_points(10);
                }
                game.preparations_made += 1;
                format!("✅ Force training complete! (Preparation {}/2)", game.preparations_made)
            } else {
                "✅ Training already completed!".to_string()
            }
        },
        5 => {
            format!("📋 Battle Readiness: {} ships, {} pilots, Plans: {}, Reputation: {}/100, Force: {} points, Preparations: {}/2", 
                game.ships_available, 
                game.pilots_available,
                if game.death_star_plans { "Complete" } else { "Incomplete" },
                game.commander.reputation,
                game.commander.force_points,
                game.preparations_made
            )
        },
        _ => "Invalid choice for Phase 3".to_string(),
    };
    
    // Auto-advance to Phase 4 when 2 preparations are complete
    if game.preparations_made >= 2 && choice != 5 {
        game.current_phase = 4;
    }
    
    result
}

fn process_phase_4_choice(game: &mut GameState, choice: u32) -> String {
    let mut success_chance = 30;
    if game.death_star_plans { success_chance += 30; }
    if game.ships_available >= 8 { success_chance += 20; }
    if game.pilots_available >= 12 { success_chance += 15; }
    if game.commander.reputation >= 70 { success_chance += 20; }
    
    match choice {
        1 => {
            if game.death_star_plans {
                if success_chance >= 60 {
                    game.game_over = true;
                    game.commander.gain_reputation(50);
                    "🌟 PERFECT VICTORY! Direct hit on the exhaust port! The Death Star explodes!".to_string()
                } else {
                    game.game_over = true;
                    game.commander.alive = false;
                    "💀 Shot missed the exhaust port! Death Star destroys Yavin 4! Defeat!".to_string()
                }
            } else {
                "❌ You don't have complete Death Star plans for precision targeting!".to_string()
            }
        },
        2 => {
            if game.commander.force_points >= 20 {
                game.game_over = true;
                game.commander.gain_reputation(75);
                "🌟 LEGENDARY FORCE VICTORY! The Force guides your shot perfectly! You are now a true Jedi!".to_string()
            } else {
                "❌ Insufficient Force Points! You need at least 20 for this ultimate technique.".to_string()
            }
        },
        3 => {
            if game.ships_available >= 6 && game.pilots_available >= 10 {
                if success_chance >= 50 {
                    game.game_over = true;
                    game.pilots_available -= 5;
                    game.ships_available -= 3;
                    "⚔️ COSTLY VICTORY! Overwhelming firepower succeeds, but many brave pilots were lost...".to_string()
                } else {
                    game.game_over = true;
                    game.commander.alive = false;
                    "💀 Not enough firepower to penetrate Death Star defenses! Defeat!".to_string()
                }
            } else {
                "❌ Insufficient ships or pilots for massive assault! Need 6+ ships and 10+ pilots.".to_string()
            }
        },
        4 => {
            // 50/50 chance based on reputation parity
            if game.commander.reputation % 2 == 0 {
                game.game_over = true;
                game.commander.alive = false;
                "🏆 HEROIC SACRIFICE! Your kamikaze run destroys the Death Star! You die a hero!".to_string()
            } else {
                game.game_over = true;
                game.commander.alive = false;
                "💀 Desperate attack fails! All is lost! The Empire wins!".to_string()
            }
        },
        _ => "Invalid choice for Phase 4".to_string(),
    }
}

// setup web server routes
pub fn create_routes(
    storage: GameStorage,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["GET", "POST"]);

    let create_game_route = warp::path("api")
        .and(warp::path("game"))
        .and(warp::path("create"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_storage(storage.clone()))
        .and_then(create_game);

    let get_state_route = warp::path("api")
        .and(warp::path("game"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(warp::get())
        .and(with_storage(storage.clone()))
        .and_then(get_game_state);

    let make_choice_route = warp::path("api")
        .and(warp::path("game"))
        .and(warp::path::param::<String>())
        .and(warp::path("choice"))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and(with_storage(storage.clone()))
        .and_then(make_choice);

    // add a test route to check if server is working
    let test_route = warp::path("api")
        .and(warp::path("test"))
        .and(warp::path::end())
        .and(warp::get())
        .map(|| {
            let response = ApiResponse {
                success: true,
                data: Some("API is working! Use POST /api/game/create to start".to_string()),
                error: None,
            };
            warp::reply::json(&response)
        });

    // serve static files for frontend
    let static_files = warp::path("static")
        .and(warp::fs::dir("web/dist"));

    test_route
        .or(create_game_route)
        .or(get_state_route)
        .or(make_choice_route)
        .or(static_files)
        .with(cors)
}

fn with_storage(
    storage: GameStorage,
) -> impl Filter<Extract = (GameStorage,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || storage.clone())
}
