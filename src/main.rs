extern crate ggez;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate rand;
#[macro_use]
extern crate text_io;


/// 
fn main() {
    pretty_env_logger::init();
    
    println!(". . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .");
    println!(". Buenas! Bienvenido al experimento para intentar ganar en el fulbito .");
    println!(". . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .");
    println!("");
    // El output a stderr no está line-buffereado
    eprint!("Presioná enter cuando quieras comenzar...");
    let _text_input_before_enter: String = read!("{}\n");


    info!(
        "Generando semilla para el rng. (Por ahora una hardcodeta para ver siempre \
        el mismo output)"
    );
    let seed: &[_] = &[1, 2, 3, 4];
    let mut rng: rand::StdRng = rand::SeedableRng::from_seed(seed);


    info!(
        "Creando ventana para ggez"
    );
    let mut c = ggez::conf::Conf::new();
    let (window_width, window_height) = sim_consts::FIELD_SIZE;
    c.window_mode = c.window_mode.dimensions(window_width as u32, window_height as u32);
    let mut ctx = ggez::Context::load_from_conf("fulbito", "nik", c).unwrap();


    info!(
        "Entrando en loop principal"
    );
    loop {
        let simulation_state = generate_simulation_state(&mut rng);
        show_state(&simulation_state, &mut ctx);

        let next_simulation_state = ask_for_team_directions(&simulation_state, &mut ctx);
        break;
    }
}


/// Acá vamos a generar un posible estado de la partida. Para eso primero fijamos algunos criterios
/// de cosas que no podrían pasar. Luego simplemente tiramos posibilidades al azar rechazando las
/// imposibles.
///
fn generate_simulation_state<RNG>(rng: &mut RNG) -> SimState
where RNG: rand::Rng {
    info!(
        "Generando un posible estado de la partida"
    );

    trace!("Creando un array para almacenar a los jugadores mientras los creamos");
    let mut players = Vec::new();

    trace!("Definimos las distribuciones de donde pueden estar los jugadores y la pelota");
    let x_dist = rand::distributions::Range::new(
        sim_consts::PLAYER_RADIUS, sim_consts::FIELD_SIZE.0 - sim_consts::PLAYER_RADIUS
    );
    let y_dist = rand::distributions::Range::new(
        sim_consts::PLAYER_RADIUS, sim_consts::FIELD_SIZE.1 - sim_consts::PLAYER_RADIUS
    );
    use std::f32::consts::PI;
    let d_dist = rand::distributions::Range::new(0.0, 2.0*PI);

    trace!("Mientras que no tenga a todos los jugadores");
    while players.len() < 6 {
        trace!("Itero para crear un jugador que no se solape con los que ya estan creados");
        use rand::distributions::IndependentSample;
        let player = Player {
            position: (x_dist.ind_sample(rng), y_dist.ind_sample(rng)),
            direction: d_dist.ind_sample(rng),
        };

        if players.iter().any(|other| are_too_close(&other, &player)) {
            trace!("Changos, no me sirve");
            continue;
        }

        trace!("Este va bien");
        players.push(player);
    }

    let ball = loop {
        trace!("Itero para intentar crear una pelota que no se solape con nadie");
        use rand::distributions::IndependentSample;
        let ball = Ball {
            position: (x_dist.ind_sample(rng), y_dist.ind_sample(rng)),
            direction: d_dist.ind_sample(rng),
        };

        if players.iter().any(|p| ball_overlaps_player(&p, &ball)) {
            trace!("Changos, se pisa con alguien");
            continue;
        }
        
        trace!("Me sirve");
        break ball
    };

    trace!("Devuelvo el estado creado");
    SimState {
        ball,
        ball_in_posesion: false,
        my_team: [
            players.pop().unwrap(),
            players.pop().unwrap(),
            players.pop().unwrap(),
        ],
        other_team: [
            players.pop().unwrap(),
            players.pop().unwrap(),
            players.pop().unwrap(),
        ],
    }
}

fn are_too_close(a: &Player, b: &Player) -> bool {
    let x = a.position.0 - b.position.0;
    let y = a.position.1 - b.position.1;

    x.powi(2) + y.powi(2) < (2.0 * sim_consts::PLAYER_RADIUS).powi(2)
}

fn ball_overlaps_player(p: &Player, b: &Ball) -> bool {
    let x = p.position.0 - b.position.0;
    let y = p.position.1 - b.position.1;

    x.powi(2) + y.powi(2) < (sim_consts::PLAYER_RADIUS + sim_consts::BALL_RADIUS).powi(2)
}

struct SimState {
    pub ball: Ball,
    pub ball_in_posesion: bool,
    pub my_team: [Player; 3],
    pub other_team: [Player; 3],
}

struct Player {
    pub position: (f32, f32),
    pub direction: f32,
}

struct Ball {
    pub position: (f32, f32),
    pub direction: f32,
}

mod sim_consts {
    pub const PLAYER_RADIUS: f32 = 10.0;
    pub const BALL_RADIUS: f32 = 5.0;
    pub const GATE_WIDTH: f32 = 80.0;
    pub const FIELD_SIZE: (f32, f32) = (300.0, 500.0);
}


/// Acá vamos a mostrar gráficamente el estado de la partida en pantalla. Porque en variables
/// claramente es muy poco claro que está pasando. 
///
fn show_state(state: &SimState, ctx: &mut ggez::Context) {
    info!(
        "Mostrando el estado posible de la partida"
    );

    let mut screen = ShowScreen { state };

    if let Err(e) = ggez::event::run(ctx, &mut screen) {
        panic!(
            "Mira, paso esto: {}", e
        );
    }
}

struct ShowScreen<'a> {
    state: &'a SimState,
}

impl<'a> ggez::event::EventHandler for ShowScreen<'a> {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        trace!("Entrando en rutina para dibujar la pantalla");
        use ggez::graphics as g;
        g::clear(ctx);

        draw_simulation_state(&self.state, ctx);

        trace!("Basta de dibujar por ahora, a mostrarlo en pantalla");
        g::present(ctx);
        Ok(())
    }
}

fn draw_simulation_state(simulation_state: &SimState, ctx: &mut ggez::Context) {
    use ggez::graphics as g;
    trace!("Dibujando a mi equipo");
    g::set_color(ctx, ggez_consts::BLUE).unwrap();
    for n in 0..3 {
        let p = &simulation_state.my_team[n];
        g::circle(
            ctx,
            g::DrawMode::Line(2.0),
            g::Point2::new(p.position.0, p.position.1),
            sim_consts::PLAYER_RADIUS,
            ggez_consts::TOLERANCE
        ).unwrap();
    }

    trace!("Dibujando al equipo enemigo");
    g::set_color(ctx, ggez_consts::RED).unwrap();
    for n in 0..3 {
        let p = &simulation_state.other_team[n];
        g::circle(
            ctx,
            g::DrawMode::Line(2.0),
            g::Point2::new(p.position.0, p.position.1),
            sim_consts::PLAYER_RADIUS,
            ggez_consts::TOLERANCE
        ).unwrap();
    }

    trace!("Dibujando la pelota");
    g::set_color(ctx, ggez_consts::WHITE).unwrap();
    let b = &simulation_state.ball;
    g::circle(
        ctx,
        g::DrawMode::Line(2.0),
        g::Point2::new(b.position.0, b.position.1),
        sim_consts::BALL_RADIUS,
        ggez_consts::TOLERANCE
    ).unwrap();
}

mod ggez_consts {
    use ggez::graphics as g;
    pub const TOLERANCE: f32 = 1.0;
    pub const BLUE:           g::Color = g::Color { r: 0.1, g: 0.15, b: 1.0, a: 1.0, };
    pub const SELECTION_BLUE: g::Color = g::Color { r: 0.5, g: 0.63, b: 1.0, a: 0.7, };
    pub const RED:            g::Color = g::Color { r: 1.0, g: 0.15, b: 0.1, a: 1.0, };
    pub const WHITE:          g::Color = g::WHITE;
}


/// Acá aparte de mostrar el estado actual vamos a mostrar una flecha que apunte en la dirección
/// que debe ir el jugador.
///
fn ask_for_team_directions(simulation_state: &SimState, ctx: &mut ggez::Context) -> SimState {

    for player in simulation_state.my_team.iter() {
        ask_for_player_direction(&simulation_state, player.position, ctx);
    }
    panic!("No deberías haber llegado hasta acá")
}

fn ask_for_player_direction(simulation_state: &SimState, position: (f32, f32), ctx: &mut ggez::Context) -> f32 {
    info!(
        "Preguntando por la dirección donde debería ir un jugador"
    );

    let mut screen = PromptDirection { simulation_state, position, direction: None };

    if let Err(e) = ggez::event::run(ctx, &mut screen) {
        panic!(
            "Mira, paso esto: {}", e
        );
    }

    screen.direction.unwrap()
}

struct PromptDirection<'a> {
    pub simulation_state: &'a SimState,
    pub position: (f32, f32),
    pub direction: Option<f32>,
}

impl<'a> ggez::event::EventHandler for PromptDirection<'a> {
    fn update(&mut self, _ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        trace!("Entrando en rutina para dibujar la pantalla");
        use ggez::graphics as g;
        g::clear(ctx);

        draw_simulation_state(&self.simulation_state, ctx);

        let t = time(ctx);
        g::set_color(ctx, ggez_consts::SELECTION_BLUE).unwrap();
        g::circle(
            ctx,
            g::DrawMode::Line(4.0),
            g::Point2::new(self.position.0, self.position.1),
            sim_consts::PLAYER_RADIUS + (10.0*t).sin() * 3.0,
            ggez_consts::TOLERANCE
        ).unwrap();

        trace!("Basta de dibujar por ahora, a mostrarlo en pantalla");
        g::present(ctx);
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut ggez::Context, button: ggez::event::MouseButton, x: i32, y: i32) {
        info!("Mouse button released: {:?}, x: {}, y: {}", button, x, y);
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _state: ggez::event::MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        info!(
            "Mouse motion, x: {}, y: {}, relative x: {}, relative y: {}",
            x, y, xrel, yrel
        );
    }
}

fn time(ctx: &ggez::Context) -> f32 {
    let d = ggez::timer::get_time_since_start(ctx);
    d.as_secs() as f32 + d.subsec_nanos() as f32 * 1e-9
}
