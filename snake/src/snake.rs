extern crate alloc;

use alloc::collections::LinkedList;
use alloc::string::String;
use lazy_static::lazy_static;
use spin::Mutex;

const WIDTH: u16 = 80;
const HEIGHT: u16 = 25;
const SIZE: u16 = WIDTH*HEIGHT;
const START_X: u8 = 20;
const START_Y: u8 = 13;

lazy_static! {
    pub static ref SNAKE: Mutex<LinkedList<Coordinate>> = {
        let mut snake = LinkedList::new();

        snake.push_back(Coordinate {
            x: START_X,
            y: START_Y,
        });

        Mutex::new(snake)
    };
}


#[derive(Debug)]
pub struct Coordinate {
    x: u8,
    y: u8,
}

pub fn snake_game() {
    let snake1 = LinkedList::from([Coordinate {x: START_X, y: START_Y}]);
}

pub fn new_snake() -> LinkedList<Coordinate> {
    let mut newsnake = LinkedList::new();

    newsnake.push_back(Coordinate {
        x: START_X,
        y: START_Y,
    });

    return newsnake;
}

pub fn update_display() -> String {
    let snake = SNAKE.lock();
    let mut display = String::from("");
    let mut my_buf: [u8; 4] = [0; 4];
    for row in 1..HEIGHT+1 {
        for col in 1..WIDTH+1 {
            match (row, col) {
                (1, 1) | (HEIGHT, WIDTH) => { display += "/"  }
                (HEIGHT, 1) | (1, WIDTH) => { display += "\\" }
                (1, _) | (HEIGHT, _) => { display += "-" }
                (_, 1) | (_, WIDTH) => { display += "|" }
                _ if snake.iter().any(|segment| {
                        segment.y == row as u8 && segment.x == col as u8
                    }) => { display += "#" }
                
                _ => { display += " " }
            }
        }
    }
    return display;
}
