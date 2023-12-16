use bot::{self, ChessEngine, Move, Board, Perft};
use std::io;

const START_POS: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const DEFAULT_DEPTH: u8 = 6;

pub struct Uci {
    engine: ChessEngine,
    position: String,
    ready: bool
}

impl Uci {
    pub fn new() -> Self {
        Self {
            engine: ChessEngine::new(),
            position: String::from(START_POS),
            ready: true
        }
    }

    pub fn get_header() -> String {
        format!("{} v{} - {} ({})", bot::NAME, bot::VERSION, bot::AUTHOR, bot::DATE)
    }

    pub fn run(&mut self) {
        println!("{}", Self::get_header());
        loop {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Could not read line.");

            let args: Vec<&str> = input.split_ascii_whitespace().collect();
            
            match args[0] {
                "help"       => self.help(),
                "uci"        => self.uci(),
                "isready"    => self.isready(),
                "setoption"  => self.setoption(args),
                "ucinewgame" => self.ucinewgame(),
                "position"   => self.position(args),
                "go"         => self.go(args),
                "d"          => self.d(),
                "quit"       => break,
                other => println!("Unknown command: '{}'. Type 'help' for a list of commands.", other)
            }
        }
    }

    fn help(&self) {
        println!("{}
List of known commands:
- help       Show this message
- uci        Switch to uci mode
- isready    Check wether engine is ready
- setoption  Change an engine option
- ucinewgame Start a new game
- position   Set a position
- go         Start thinking
- d          Print current board
- quit       Quit.", 
            Self::get_header()
        );
    }

    fn uci(&self) {
        println!("id name {} v{}
id author {}

uciok",
            bot::NAME, bot::VERSION,
            bot::AUTHOR
        );
    }

    fn isready(&self) {
        while !self.ready {}
        println!("readyok");
    }

    fn setoption(&mut self, args: Vec<&str>) {
        self.ready = false;

        self.ready = true;
    }

    fn ucinewgame(&mut self) {
        self.ready = false;

        self.ready = true;
    }

    fn position(&mut self, args: Vec<&str>) {
        self.ready = false;

        let moves_index = args.clone().into_iter().position(|s| s == "moves");
        
        let start_fen = if let Some(pos_type) = args.get(1) {
            match *pos_type {
                "startpos" => {
                    String::from(START_POS)
                },
                "fen" => args[2..moves_index.unwrap_or(args.capacity())].join(" "),
                _ => return
            }
        } else {
            println!("No position given!");
            return;
        };

        let mut board = match Board::try_from_fen(&start_fen) {
            Ok(b) => b,
            Err(_) => {
                println!("Invalid position!");
                return;
            }
        };

        if let Some(moves_index) = moves_index {
            for mv_str in &args[moves_index + 1..] {
                let mv = match Move::try_from_str(mv_str, &board) {
                    Ok(mv) => mv,
                    Err(e) => {
                        println!("Error parsing move {}: {}", mv_str, e);
                        return;
                    }
                };
                board.make_move(&mv);
            }
        }

        self.position = board.get_fen();
        self.ready = true;
    }

    fn go(&mut self, args: Vec<&str>) {
        let board = match Board::try_from_fen(&self.position) {
            Ok(b) => b,
            Err(_) => {
                println!("Invalid position!");
                return;
            }
        };

        let mut depth = DEFAULT_DEPTH;

        if let Some(s) = args.get(1) {
            if let Some(d) = args.get(2) {
                let d = d.parse::<u8>().unwrap_or(1);
                match *s {
                    "perft" => {
                        Perft::new(board).verb_perft(d, true, false);
                        return;
                    },
                    "depth" => depth = d,
                    _ => ()
                }
            }
        }
        println!("bestmove {}", self.engine.search(board, depth).0);
    }

    fn d(&self) {
        let board =  match Board::try_from_fen(&self.position) {
            Ok(b) => b,
            Err(_) => {
                println!("Invalid position!");
                return;
            }
        };
        println!("{}", board);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        Uci::new().run()
    }
}
