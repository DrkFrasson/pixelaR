use std::{
    collections::HashMap,
    thread,
    time
};

#[derive(Default)]
pub struct Text {
    l1: String,
    l2: String,
    l3: String,
    l4: String,
    l5: String,
    l6: String,
    l7: String,
    l8: String,
}

pub struct Screen {
    l1: Text,
    l2: Text,
    l3: Text,
    l4: Text,
}

impl Screen {
    fn new() -> Screen{
        Screen{
            l1: compose_pixels(compose_line(" ".to_string())),
            l2: compose_pixels(compose_line(" ".to_string())),
            l3: compose_pixels(compose_line(" ".to_string())),
            l4: compose_pixels(compose_line(" ".to_string())),
        }
    }

    fn push(self, message: Text) -> Screen{
        Screen{
            l1: self.l2,
            l2: self.l3,
            l3: self.l4,
            l4: message,
        }
    }
}

fn main() {
    let mut i: u16 = 10;
    let mut screen: Screen = Screen::new();
    let delay = time::Duration::from_millis(1000);
    print!("\x1b[s\x1b[?47h");
    print!("\x1b[2J");
    loop{
        if i == 0 {
            screen = screen.push(compose_pixels(compose_line("BOOOOOOOOM!".to_string())));
            print!("\x1b[H");
            draw_screen(&screen);
            thread::sleep(time::Duration::from_millis(2000));
            print!("\x1b[?47l\x1b[u");
            std::process::exit(0);
        }else{
            screen = screen.push(compose_pixels(compose_line(("autodestruction: ".to_owned() + &(i).to_string() as &str ).to_string())));
            print!("\x1b[H");
            thread::sleep(delay);
            draw_screen(&screen);
            i -= 1;
        }
    }

}

pub fn compose_screen(l1: Text, l2: Text, l3: Text, l4: Text) -> Screen{ Screen{ l1, l2, l3, l4, }}

pub fn compose_pixels(line: Text) -> Text
{
    let mut buff: Text = Default::default();

    for i in line.l1.chars() {
        buff.l1 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    for i in line.l2.chars() {
        buff.l2 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    for i in line.l3.chars() {
        buff.l3 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    for i in line.l4.chars() {
        buff.l4 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    for i in line.l5.chars() {
        buff.l5 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    for i in line.l6.chars() {
        buff.l6 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    for i in line.l7.chars() {
        buff.l7 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    for i in line.l8.chars() {
        buff.l8 += match i {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        }
    }
    return buff;
}


pub fn compose_line(mut message: String) -> Text
{
    let mut line: Vec<char> = message.chars().collect();
    if line.len() != 20 {
        if line.len() > 20 {
            panic!("Too large line!");
        }else{
            while line.len() < 20 {
                message += " ";
                line = message.chars().collect();
            }
        }
    }

    let mut abc: HashMap<char, Text> = HashMap::new();
    abc.insert( 'a', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01110".to_string(),
        l4: "00001".to_string(),
        l5: "01111".to_string(),
        l6: "10001".to_string(),
        l7: "01111".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'b', Text{
        l1: "10000".to_string(),
        l2: "10000".to_string(),
        l3: "11100".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "10010".to_string(),
        l7: "11100".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'c', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01110".to_string(),
        l4: "10000".to_string(),
        l5: "10000".to_string(),
        l6: "10000".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'd', Text{
        l1: "00010".to_string(),
        l2: "00010".to_string(),
        l3: "01110".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "10010".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'e', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01100".to_string(),
        l4: "10010".to_string(),
        l5: "11110".to_string(),
        l6: "10000".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'f', Text{
        l1: "00110".to_string(),
        l2: "01000".to_string(),
        l3: "11110".to_string(),
        l4: "01000".to_string(),
        l5: "01000".to_string(),
        l6: "01000".to_string(),
        l7: "01000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'g', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01110".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "01110".to_string(),
        l7: "00010".to_string(),
        l8: "11100".to_string(),
    });
    abc.insert( 'h', Text{
        l1: "10000".to_string(),
        l2: "10000".to_string(),
        l3: "11100".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "10010".to_string(),
        l7: "10010".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'i', Text{
        l1: "00100".to_string(),
        l2: "00000".to_string(),
        l3: "01100".to_string(),
        l4: "00100".to_string(),
        l5: "00100".to_string(),
        l6: "00100".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'j', Text{
        l1: "00001".to_string(),
        l2: "00000".to_string(),
        l3: "00111".to_string(),
        l4: "00001".to_string(),
        l5: "00001".to_string(),
        l6: "00001".to_string(),
        l7: "00001".to_string(),
        l8: "01110".to_string(),
    });
    abc.insert( 'k', Text{
        l1: "10000".to_string(),
        l2: "10001".to_string(),
        l3: "10010".to_string(),
        l4: "10100".to_string(),
        l5: "11100".to_string(),
        l6: "10010".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'l', Text{
        l1: "11000".to_string(),
        l2: "01000".to_string(),
        l3: "01000".to_string(),
        l4: "01000".to_string(),
        l5: "01000".to_string(),
        l6: "01000".to_string(),
        l7: "00110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'm', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "11110".to_string(),
        l4: "10101".to_string(),
        l5: "10101".to_string(),
        l6: "10101".to_string(),
        l7: "10101".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'n', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "10110".to_string(),
        l4: "11001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'ñ', Text{
        l1: "01110".to_string(),
        l2: "00000".to_string(),
        l3: "10110".to_string(),
        l4: "11001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'o', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01110".to_string(),
        l4: "10001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'p', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "11100".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "11100".to_string(),
        l7: "10000".to_string(),
        l8: "10000".to_string(),
    });
    abc.insert( 'q', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01110".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "10010".to_string(),
        l7: "01110".to_string(),
        l8: "00010".to_string(),
    });
    abc.insert( 'r', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "11100".to_string(),
        l4: "10010".to_string(),
        l5: "10000".to_string(),
        l6: "10000".to_string(),
        l7: "10000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 's', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01110".to_string(),
        l4: "10000".to_string(),
        l5: "01100".to_string(),
        l6: "00010".to_string(),
        l7: "11100".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 't', Text{
        l1: "01000".to_string(),
        l2: "01000".to_string(),
        l3: "11110".to_string(),
        l4: "01000".to_string(),
        l5: "01000".to_string(),
        l6: "01000".to_string(),
        l7: "00110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'u', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "10010".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "10010".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'v', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "10001".to_string(),
        l4: "10001".to_string(),
        l5: "01010".to_string(),
        l6: "01010".to_string(),
        l7: "00100".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'w', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "10001".to_string(),
        l4: "10001".to_string(),
        l5: "10101".to_string(),
        l6: "01010".to_string(),
        l7: "01010".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'x', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "10001".to_string(),
        l4: "01010".to_string(),
        l5: "00100".to_string(),
        l6: "01010".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'y', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "10010".to_string(),
        l4: "10010".to_string(),
        l5: "10010".to_string(),
        l6: "01110".to_string(),
        l7: "00010".to_string(),
        l8: "11100".to_string(),
    });
    abc.insert( 'z', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "11111".to_string(),
        l4: "00010".to_string(),
        l5: "00100".to_string(),
        l6: "01000".to_string(),
        l7: "11111".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( ' ', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "00000".to_string(),
        l4: "00000".to_string(),
        l5: "00000".to_string(),
        l6: "00000".to_string(),
        l7: "00000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'A', Text{
        l1: "00100".to_string(),
        l2: "00100".to_string(),
        l3: "01010".to_string(),
        l4: "01010".to_string(),
        l5: "01110".to_string(),
        l6: "10001".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'B', Text{
        l1: "11110".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "11110".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "11110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'C', Text{
        l1: "01110".to_string(),
        l2: "10001".to_string(),
        l3: "10000".to_string(),
        l4: "10000".to_string(),
        l5: "10000".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'D', Text{
        l1: "11110".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "10001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "11110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'E', Text{
        l1: "11111".to_string(),
        l2: "10000".to_string(),
        l3: "10000".to_string(),
        l4: "11111".to_string(),
        l5: "10000".to_string(),
        l6: "10000".to_string(),
        l7: "11111".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'F', Text{
        l1: "11111".to_string(),
        l2: "10000".to_string(),
        l3: "10000".to_string(),
        l4: "11110".to_string(),
        l5: "10000".to_string(),
        l6: "10000".to_string(),
        l7: "10000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'G', Text{
        l1: "01110".to_string(),
        l2: "10001".to_string(),
        l3: "10000".to_string(),
        l4: "10111".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'H', Text{
        l1: "10001".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "11111".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'I', Text{
        l1: "01110".to_string(),
        l2: "00100".to_string(),
        l3: "00100".to_string(),
        l4: "00100".to_string(),
        l5: "00100".to_string(),
        l6: "00100".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'J', Text{
        l1: "00001".to_string(),
        l2: "00001".to_string(),
        l3: "00001".to_string(),
        l4: "00001".to_string(),
        l5: "00001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'K', Text{
        l1: "10001".to_string(),
        l2: "10010".to_string(),
        l3: "10100".to_string(),
        l4: "11000".to_string(),
        l5: "10100".to_string(),
        l6: "10010".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'L', Text{
        l1: "10000".to_string(),
        l2: "10000".to_string(),
        l3: "10000".to_string(),
        l4: "10000".to_string(),
        l5: "10000".to_string(),
        l6: "10000".to_string(),
        l7: "11111".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'M', Text{
        l1: "10001".to_string(),
        l2: "11011".to_string(),
        l3: "10101".to_string(),
        l4: "10001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'N', Text{
        l1: "10001".to_string(),
        l2: "11001".to_string(),
        l3: "10101".to_string(),
        l4: "10101".to_string(),
        l5: "10101".to_string(),
        l6: "10011".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'Ñ', Text{
        l1: "01110".to_string(),
        l2: "10001".to_string(),
        l3: "11001".to_string(),
        l4: "10101".to_string(),
        l5: "10101".to_string(),
        l6: "10011".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'O', Text{
        l1: "01110".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "10001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'P', Text{
        l1: "11110".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "11110".to_string(),
        l5: "10000".to_string(),
        l6: "10000".to_string(),
        l7: "10000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'Q', Text{
        l1: "01110".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "10001".to_string(),
        l5: "10001".to_string(),
        l6: "10101".to_string(),
        l7: "01110".to_string(),
        l8: "00001".to_string(),
    });
    abc.insert( 'R', Text{
        l1: "11110".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "11110".to_string(),
        l5: "10100".to_string(),
        l6: "10010".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'S', Text{
        l1: "01110".to_string(),
        l2: "10001".to_string(),
        l3: "10000".to_string(),
        l4: "01110".to_string(),
        l5: "00001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'T', Text{
        l1: "11111".to_string(),
        l2: "00100".to_string(),
        l3: "00100".to_string(),
        l4: "00100".to_string(),
        l5: "00100".to_string(),
        l6: "00100".to_string(),
        l7: "00100".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'U', Text{
        l1: "10001".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "10001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'V', Text{
        l1: "10001".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "01010".to_string(),
        l5: "01010".to_string(),
        l6: "01010".to_string(),
        l7: "00100".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'W', Text{
        l1: "10001".to_string(),
        l2: "10001".to_string(),
        l3: "10001".to_string(),
        l4: "10101".to_string(),
        l5: "10101".to_string(),
        l6: "01010".to_string(),
        l7: "01010".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'X', Text{
        l1: "10001".to_string(),
        l2: "10001".to_string(),
        l3: "01010".to_string(),
        l4: "00100".to_string(),
        l5: "01010".to_string(),
        l6: "10001".to_string(),
        l7: "10001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'Y', Text{
        l1: "10001".to_string(),
        l2: "10001".to_string(),
        l3: "01010".to_string(),
        l4: "01010".to_string(),
        l5: "00100".to_string(),
        l6: "00100".to_string(),
        l7: "00100".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'Z', Text{
        l1: "11111".to_string(),
        l2: "00001".to_string(),
        l3: "00010".to_string(),
        l4: "00100".to_string(),
        l5: "01000".to_string(),
        l6: "10000".to_string(),
        l7: "11111".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '.', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "00000".to_string(),
        l4: "00000".to_string(),
        l5: "00000".to_string(),
        l6: "00000".to_string(),
        l7: "01000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( ',', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "00000".to_string(),
        l4: "00000".to_string(),
        l5: "00000".to_string(),
        l6: "00000".to_string(),
        l7: "01000".to_string(),
        l8: "10000".to_string(),
    });
    abc.insert( ':', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01000".to_string(),
        l4: "00000".to_string(),
        l5: "00000".to_string(),
        l6: "00000".to_string(),
        l7: "01000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( ';', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "01000".to_string(),
        l4: "00000".to_string(),
        l5: "00000".to_string(),
        l6: "00000".to_string(),
        l7: "01000".to_string(),
        l8: "10000".to_string(),
    });
    abc.insert( '"', Text{
        l1: "01010".to_string(),
        l2: "01010".to_string(),
        l3: "00000".to_string(),
        l4: "00000".to_string(),
        l5: "00000".to_string(),
        l6: "00000".to_string(),
        l7: "00000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '(', Text{
        l1: "00010".to_string(),
        l2: "00100".to_string(),
        l3: "01000".to_string(),
        l4: "01000".to_string(),
        l5: "01000".to_string(),
        l6: "00100".to_string(),
        l7: "00010".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( ')', Text{
        l1: "01000".to_string(),
        l2: "00100".to_string(),
        l3: "00010".to_string(),
        l4: "00010".to_string(),
        l5: "00010".to_string(),
        l6: "00100".to_string(),
        l7: "01000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '{', Text{
        l1: "00011".to_string(),
        l2: "00100".to_string(),
        l3: "00010".to_string(),
        l4: "01100".to_string(),
        l5: "00010".to_string(),
        l6: "00100".to_string(),
        l7: "00011".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '}', Text{
        l1: "11000".to_string(),
        l2: "00100".to_string(),
        l3: "01000".to_string(),
        l4: "00110".to_string(),
        l5: "01000".to_string(),
        l6: "00100".to_string(),
        l7: "11000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '-', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "00000".to_string(),
        l4: "00000".to_string(),
        l5: "01110".to_string(),
        l6: "00000".to_string(),
        l7: "00000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '_', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "00000".to_string(),
        l4: "00000".to_string(),
        l5: "00000".to_string(),
        l6: "00000".to_string(),
        l7: "00000".to_string(),
        l8: "01110".to_string(),
    });
    abc.insert( '!', Text{
        l1: "01000".to_string(),
        l2: "01000".to_string(),
        l3: "01000".to_string(),
        l4: "01000".to_string(),
        l5: "01000".to_string(),
        l6: "01000".to_string(),
        l7: "00000".to_string(),
        l8: "01000".to_string(),
    });
    abc.insert( '>', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "10000".to_string(),
        l4: "01100".to_string(),
        l5: "00011".to_string(),
        l6: "01100".to_string(),
        l7: "10000".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '<', Text{
        l1: "00000".to_string(),
        l2: "00000".to_string(),
        l3: "00001".to_string(),
        l4: "00110".to_string(),
        l5: "11000".to_string(),
        l6: "00110".to_string(),
        l7: "00001".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( 'ó', Text{
        l1: "00110".to_string(),
        l2: "00000".to_string(),
        l3: "01110".to_string(),
        l4: "10001".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });

    abc.insert( '0', Text{
        l1: "00000".to_string(),
        l2: "01110".to_string(),
        l3: "10001".to_string(),
        l4: "10101".to_string(),
        l5: "10101".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '1', Text{
        l1: "00000".to_string(),
        l2: "00100".to_string(),
        l3: "01100".to_string(),
        l4: "00100".to_string(),
        l5: "00100".to_string(),
        l6: "00100".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '2', Text{
        l1: "00000".to_string(),
        l2: "01110".to_string(),
        l3: "10001".to_string(),
        l4: "00010".to_string(),
        l5: "00100".to_string(),
        l6: "01000".to_string(),
        l7: "11111".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '3', Text{
        l1: "00000".to_string(),
        l2: "11111".to_string(),
        l3: "00001".to_string(),
        l4: "00110".to_string(),
        l5: "00001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '4', Text{
        l1: "00000".to_string(),
        l2: "00100".to_string(),
        l3: "01000".to_string(),
        l4: "10010".to_string(),
        l5: "11111".to_string(),
        l6: "00010".to_string(),
        l7: "00010".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '5', Text{
        l1: "00000".to_string(),
        l2: "11111".to_string(),
        l3: "10000".to_string(),
        l4: "11110".to_string(),
        l5: "00001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '6', Text{
        l1: "00000".to_string(),
        l2: "00100".to_string(),
        l3: "01000".to_string(),
        l4: "10000".to_string(),
        l5: "11110".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '7', Text{
        l1: "00000".to_string(),
        l2: "11111".to_string(),
        l3: "10001".to_string(),
        l4: "00010".to_string(),
        l5: "00010".to_string(),
        l6: "00100".to_string(),
        l7: "00100".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '8', Text{
        l1: "00000".to_string(),
        l2: "01110".to_string(),
        l3: "01010".to_string(),
        l4: "01110".to_string(),
        l5: "10001".to_string(),
        l6: "10001".to_string(),
        l7: "01110".to_string(),
        l8: "00000".to_string(),
    });
    abc.insert( '9', Text{
        l1: "00000".to_string(),
        l2: "01110".to_string(),
        l3: "10001".to_string(),
        l4: "10001".to_string(),
        l5: "01111".to_string(),
        l6: "00001".to_string(),
        l7: "00001".to_string(),
        l8: "00000".to_string(),
    });

    let mut message_composed: Text = Default::default();

    for c in message.chars() {
        let letter: &Text = match abc.get(&c) {
            Some(c) => c,
            _ => {println!("\x1b[1;3;31merror:\x1b[22;39m Not a printable character: {:?}", c); std::process::exit(1);},
        };
        message_composed.l1 += &(" ".to_owned() + &letter.l1);
        message_composed.l2 += &(" ".to_owned() + &letter.l2);
        message_composed.l3 += &(" ".to_owned() + &letter.l3);
        message_composed.l4 += &(" ".to_owned() + &letter.l4);
        message_composed.l5 += &(" ".to_owned() + &letter.l5);
        message_composed.l6 += &(" ".to_owned() + &letter.l6);
        message_composed.l7 += &(" ".to_owned() + &letter.l7);
        message_composed.l8 += &(" ".to_owned() + &letter.l8);
    }
    message_composed.l1 += " ";
    message_composed.l2 += " ";
    message_composed.l3 += " ";
    message_composed.l4 += " ";
    message_composed.l5 += " ";
    message_composed.l6 += " ";
    message_composed.l7 += " ";
    message_composed.l8 += " ";

    return message_composed;
}

fn empty() -> String {
    let mut responce: String = Default::default();
    for _ in 0..=120 {
        responce += "  ";
    }
    return responce;
}

pub fn draw_screen(message_composed: &Screen) // 36 * 120 pixels (4 * 20 characters)
{
    let empty: String = empty();
    print!("\n");
    println!("\x1b[48;5;248m                    {}\x1b[0m", empty);
    println!("\x1b[48;5;248m                  {}\x1b[48;5;240m  \x1b[0m", empty);
    println!("\x1b[48;5;248m    \x1b[48;5;245m            {}\x1b[48;5;240m    \x1b[0m", empty);
    println!("\x1b[48;5;248m    \x1b[48;5;245m            {}\x1b[48;5;240m    \x1b[0m", empty);
    println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m    {}\x1b[48;5;245m    \x1b[48;5;248m\x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
    println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m{}    \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
    println!("\x1b[48;5;248m    \x1b[48;5;245m  {}          \x1b[48;5;240m    \x1b[0m", empty);
    println!("\x1b[48;5;248m    \x1b[48;5;245m  {}          \x1b[48;5;240m    \x1b[0m", empty);
    println!("\x1b[48;5;248m  \x1b[48;5;240m    {}              \x1b[0m", empty);
    println!("\x1b[48;5;240m                    {}\x1b[0m", empty);
}
