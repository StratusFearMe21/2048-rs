use termion;
use board;
use game;
use std::io::Write;
use game::GameStatus;

fn header<W>(out: &mut W, score: i32)
    where W: Write
{
    write!(out,
           "2048-rs [pierrec.tech]{num:>pad$}\r\n",
           num = score,
           pad = 11)
        .unwrap();
}

fn footer<W>(out: &mut W, status: GameStatus)
    where W: Write
{
    let text = match status {
        GameStatus::ongoing => "    [ ← ↑ → ↓ ], q for quit\r\n",
        GameStatus::lost => "    [  🎮 ⛔  ], q for quit\r\n",
        GameStatus::interrupted => "    [  🎮 🚦  ], quit? (y/n)\r\n",
        GameStatus::won => "    [ 🎉🎉🎉 ], quit? (y/n)\r\n",
    };
    write!(out, "{}", text).unwrap();
}

fn clear<W>(out: &mut W)
    where W: Write
{
    write!(out,
           "{}{}{}",
           termion::clear::All,
           termion::cursor::Hide,
           termion::cursor::Goto(1, 1))
        .unwrap();
}

pub fn display_game<W>(out: &mut W, board: &board::Board, game: &game::Game)
    where W: Write
{
    clear(out);
    header(out, game.score());
    board.print(game.data(), out);
    footer(out, game.status());
}
