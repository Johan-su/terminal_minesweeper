use std::io;
use rand::Rng;

enum Actions
{
    Up,
    Left,
    Down,
    Right,
    Flag,
    Sweep,
    Quit,
}


enum VisualState
{
    Unknown,
    Flagged,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
}


enum State
{
    Empty,
    Bomb,
}


enum GameState
{
    Stopped,
    Started,
    Running,
    Lost,
    Won,
}


struct Point2i
{
    x: i32,
    y: i32,
}


struct Game
{
    visual_grid: Vec<VisualState>,
    grid: Vec<State>,
    game_state: GameState,
    player_pos: Point2i,
    bombs_left: u32,
}


const GRID_HEIGHT: i32 = 10;
const GRID_WIDTH:  i32 = 10;
const GRID_SIZE:   i32 = GRID_WIDTH * GRID_HEIGHT;


const BOMB_AMOUNT: u32 = 10;

fn main()
{
    if (BOMB_AMOUNT >= GRID_SIZE as u32)
    {
        panic!("BOMB_AMOUNT: {} cannot be greater than or equal to GRID_SIZE: {}", GRID_SIZE, BOMB_AMOUNT);
    }

    let mut game = Game {
        visual_grid: Vec::new(),
        grid       : Vec::new(),
        player_pos : Point2i {x: 0, y: 0},
        game_state : GameState::Stopped,
        bombs_left : BOMB_AMOUNT,
    };

    first_init_state(&mut game);
    loop 
    {
        print_state(&game);

        let mut buffer: String = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let b = buffer.chars().nth(0);

        match game.game_state
        {
            GameState::Stopped => 
            {
                match b.unwrap()
                {
                    'p' => game.game_state = GameState::Started,
                    'q' => do_action(Actions::Quit,  &mut game),
                    _   => println!("Error invalid input"),
                }
            },


            GameState::Started | GameState::Running =>
            {
                match b.unwrap()
                {
                    'w' => do_action(Actions::Up,    &mut game),
                    'a' => do_action(Actions::Left,  &mut game),
                    's' => do_action(Actions::Down,  &mut game),
                    'd' => do_action(Actions::Right, &mut game),
                    'f' => do_action(Actions::Flag,  &mut game),
                    'p' => do_action(Actions::Sweep, &mut game),
                    'q' => do_action(Actions::Quit,  &mut game),
                    _   => println!("Error invalid input"),
                }
            },


            GameState::Won | GameState::Lost => 
            {
                match b.unwrap()
                {
                    'p' => do_action(Actions::Sweep, &mut game),
                    'q' => do_action(Actions::Quit,  &mut game),
                    _   => println!("Error invalid input"),
                }
                game.game_state = GameState::Stopped;
                first_init_state(&mut game);
            },
        }
    }
}

// initalize all the squares
fn first_init_state(game: &mut Game)
{
    game.grid.clear();
    game.visual_grid.clear();

    for _ in 0..GRID_SIZE
    {
        game.grid.push(State::Empty);
        game.visual_grid.push(VisualState::Unknown);
    }
}

// initalize all bombs
fn init_bombs(game: &mut Game)
{

    let mut bombs_left = BOMB_AMOUNT;

    game.bombs_left = bombs_left;

    let mut rng = rand::thread_rng();

    let rand_val = GRID_SIZE / BOMB_AMOUNT as i32;
    
    while (bombs_left != 0)
    {
        for i in 0..GRID_SIZE
        {
            let player_pos_as_index = game.player_pos.x + game.player_pos.y * GRID_HEIGHT;
            if(i == player_pos_as_index)
            {
                continue;
            }

            match game.grid[i as usize]
            {
                State::Empty =>
                {
                    if (rng.gen_range(0..=rand_val) == 0 && bombs_left != 0)
                    {
                        game.grid[i as usize] = State::Bomb;
                        bombs_left -= 1;
                    }
                },
                State::Bomb => {},
            }
        }

    }
}


fn print_state(game: &Game) // add check if flags are incorrectly placed and keep flags that are correctly placed
{
    let mut counter: i32 = 0;

    for v_state in game.visual_grid.iter()
    {
        
        let is_player_square: bool = (game.player_pos.x == counter % GRID_WIDTH) && (game.player_pos.y == counter / GRID_HEIGHT);


        print!(" {}", if (is_player_square) {'['} else {' '});


        let mut character: char = match v_state
        {
            VisualState::Unknown => '.',
            VisualState::Flagged => 'F',
            VisualState::Zero    => ' ',
            VisualState::One     => '1',
            VisualState::Two     => '2',
            VisualState::Three   => '3',
            VisualState::Four    => '4',
            VisualState::Five    => '5',
            VisualState::Six     => '6',
            VisualState::Seven   => '7',
            VisualState::Eight   => '8',
        };


        // if lost game, show all bombs
        match game.game_state
        {
            GameState::Lost => 
            {
                match game.visual_grid[counter as usize]
                {
                    VisualState::Flagged => 
                    {
                        match game.grid[counter as usize]
                        {
                            State::Empty => character = 'x',
                            _ => {},
                        }
                    },
                    _ => 
                    {
                        match game.grid[counter as usize]
                        {
                            State::Bomb => character = '*',
                            _ => {},
                        }
                    },
                }
            },
            _ => {},
        }

        print!("{}", character);

        print!("{} ", if (is_player_square) {']'} else {' '});
    
        if (counter % GRID_WIDTH == 9) // 
        {
            print!("\n");
        }
        
        counter += 1;
    }
    println!("\n\n"); // flush to stdout and add \n
}


fn do_action(action: Actions, game: &mut Game)
{
    match action
    {
        Actions::Up => 
        {
            game.player_pos.y += -1;
        },


        Actions::Left =>
        {
            game.player_pos.x += -1;
        },


        Actions::Down =>
        {
            game.player_pos.y += 1;
        },


        Actions::Right => 
        {
            game.player_pos.x += 1;
        },


        Actions::Flag => 
        {
            let player_pos_as_index = (game.player_pos.x + game.player_pos.y * GRID_HEIGHT) as usize;
            match game.visual_grid[player_pos_as_index]
            {
                VisualState::Unknown =>
                {
                    game.visual_grid[player_pos_as_index] = VisualState::Flagged;
                    match (game.grid[player_pos_as_index])
                    {
                        State::Bomb => 
                        {
                            game.bombs_left -= 1;
                            if(game.bombs_left == 0)
                            {
                                game.game_state = GameState::Won;
                                println!("YOU WON! :)");

                            }
                        },
                        _ => {},
                    }
                },


                VisualState::Flagged => 
                {
                    game.visual_grid[player_pos_as_index] = VisualState::Unknown;
                    match (game.grid[player_pos_as_index])
                    {
                        State::Bomb => game.bombs_left += 1,
                        _ => {},
                    }
                },

                VisualState::Zero | VisualState::One | VisualState::Two | VisualState::Three | VisualState::Four | VisualState::Five | VisualState::Six | VisualState::Seven | VisualState::Eight =>
                {},


            }
        },


        Actions::Sweep =>
        {
            match game.game_state
            {
                GameState::Started => 
                {
                    init_bombs(game);
                    game.game_state = GameState::Running;
                },
                _ => {}
            }
            sweep_at_player(game);
        },

        Actions::Quit => std::process::exit(0),
    }

    game.player_pos.x = (game.player_pos.x + GRID_WIDTH) % GRID_WIDTH; // keep player inside the grid
    game.player_pos.y = (game.player_pos.y + GRID_WIDTH) % GRID_HEIGHT;
}



fn sweep_at_player(game: &mut Game)
{
    sweep_at_point(game.player_pos.x, game.player_pos.y, game);
}


fn sweep_at_point(x_p: i32, y_p: i32, game: &mut Game)
{
    let pos = x_p + y_p * GRID_HEIGHT;

    if (pos < 0 || pos >= GRID_SIZE)
    {
        return;
    }

    match game.visual_grid[pos as usize]
    {
        VisualState::Unknown => {},
        VisualState::Flagged => {},
        _ => return,
    }

    let val = calc_at_point(x_p, y_p, game);

    match val
    {
        -1 => 
        {
            game.game_state = GameState::Lost;
            println!("YOU LOST! :(");
        },
        0  =>
        {
            game.visual_grid[pos as usize] = VisualState::Zero;
            let start_x = x_p - 1;
            let start_y = y_p - 1;


            for y in 0..=2
            {
                for x in 0..=2
                {
                    if (start_x + x < 0 || start_y + y < 0 || start_x + x >= GRID_WIDTH || start_y + y >= GRID_HEIGHT)
                    {
                        continue;
                    }
                    let pos_as_index = (start_x + x) + (start_y + y) * GRID_HEIGHT;
                    if (pos != pos_as_index)
                    {
                        sweep_at_point(start_x + x, start_y + y, game);
                    }
                }
            }
        },
        1 => game.visual_grid[pos as usize] = VisualState::One,
        2 => game.visual_grid[pos as usize] = VisualState::Two,
        3 => game.visual_grid[pos as usize] = VisualState::Three,
        4 => game.visual_grid[pos as usize] = VisualState::Four,
        5 => game.visual_grid[pos as usize] = VisualState::Five,
        6 => game.visual_grid[pos as usize] = VisualState::Six,
        7 => game.visual_grid[pos as usize] = VisualState::Seven,
        8 => game.visual_grid[pos as usize] = VisualState::Eight,

        _ => panic!("Unreachable"),
    }
}


// assumes x_p and y_p are inside the grid
fn calc_at_point(x_p: i32, y_p: i32, game: &Game) -> i8
{
    let state: &State = &game.grid[(x_p + y_p * GRID_HEIGHT) as usize];
    match (state) 
    {
        State::Bomb => return -1,
        _ =>
        {
            let start_x = x_p - 1;
            let start_y = y_p - 1;

            let mut bomb_count: i8 = 0;

            for y in 0..=2
            {
                for x in 0..=2
                {
                    if (start_x + x < 0 || start_y + y < 0 || start_x + x >= GRID_WIDTH || start_y + y >= GRID_HEIGHT)
                    {
                        continue;
                    }
                    let pos_as_index = (start_x + x) + (start_y + y) * GRID_HEIGHT;
                    match game.grid[pos_as_index as usize]
                    {
                        State::Bomb => bomb_count += 1,
                        _ => {},
                    }
                }
            }
            return bomb_count;
        },
    }
}




fn absi(a: i32) -> i32
{
    if a < 0
    {
        return -a;
    }
    else
    {
        return a;
    }
}