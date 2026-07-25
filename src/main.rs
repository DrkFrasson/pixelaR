use std::{
    collections::HashMap,
    thread,
    time
};

#[derive(Default)]
pub struct Text{
    l1: String,
    l2: String,
    l3: String,
    l4: String,
    l5: String,
    l6: String,
    l7: String,
    l8: String,
}

impl Text{
    fn draw_line(&self) {
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", self.l8);
    }
}

pub struct Screen{
    l1: Text,
    l2: Text,
    l3: Text,
    l4: Text,
}

impl Screen{
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

    fn draw(&self) {
        let empty: String = empty();
        println!("\x1b[48;5;248m                    {}\x1b[0m", empty);
        println!("\x1b[48;5;248m                  {}\x1b[48;5;240m  \x1b[0m", empty);
        println!("\x1b[48;5;248m    \x1b[48;5;245m            {}\x1b[48;5;240m    \x1b[0m", empty);
        println!("\x1b[48;5;248m    \x1b[48;5;245m            {}\x1b[48;5;240m    \x1b[0m", empty);
        println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m    {}\x1b[48;5;245m    \x1b[48;5;248m\x1b[48;5;240m    \x1b[0m", empty);
        println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
        self.l1.draw_line();
        println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
        self.l2.draw_line();
        println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
        self.l3.draw_line();
        println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
        self.l4.draw_line();
        println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  \x1b[48;5;17m{}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
        println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m{}    \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
        println!("\x1b[48;5;248m    \x1b[48;5;245m  {}          \x1b[48;5;240m    \x1b[0m", empty);
        println!("\x1b[48;5;248m    \x1b[48;5;245m  {}          \x1b[48;5;240m    \x1b[0m", empty);
        println!("\x1b[48;5;248m  \x1b[48;5;240m    {}              \x1b[0m", empty);
        println!("\x1b[48;5;240m                    {}\x1b[0m", empty);
    }

    fn put(&mut self, message: String, line: u8) {
        match line {
            1 => self.l1 = compose_pixels(compose_line(message)),
            2 => self.l2 = compose_pixels(compose_line(message)),
            3 => self.l3 = compose_pixels(compose_line(message)),
            4 => self.l4 = compose_pixels(compose_line(message)),
            _ => panic!("Unvalid Line!"),
        }
    }

    fn from(line1: String, line2: String, line3: String, line4: String) -> Screen {
        Screen{
            l1: compose_pixels(compose_line(line1)),
            l2: compose_pixels(compose_line(line2)),
            l3: compose_pixels(compose_line(line3)),
            l4: compose_pixels(compose_line(line4)),
        }
    }
}

fn main() {
    let mut i: u16 = 50000;
    print!("\x1b[s\x1b[?47h\x1b[2J");
    let now = time::SystemTime::now();
    let mut screen: Screen =
        Screen::from(
            "Hello, World".to_string(),
            "LCD screen".to_string(),
            "--> Rust".to_string(),
            "".to_string());
    print!("\x1b[H");
    screen.draw();
//    let mut screen_average: Vec<time::Duration> = vec![];
    loop{
        if i == 0 {break;}
//        let now_draw = time::SystemTime::now();
        screen = screen.push(compose_pixels(compose_line(("screen number: ".to_owned() + &(i).to_string() as &str ).to_string())));
        print!("\x1b[H");
        screen.draw();
/*        match now_draw.elapsed() {
            Ok(t) => screen_average.push(t),
            Err(e) => println!("Error: {e}"),
        }*/
        i -= 1;
    }
    print!("\x1b[?47l\x1b[u");
    match now.elapsed() {
        Ok(t) => println!("50000 screens in {t:#?}"),
        Err(e) => println!("Error: {e}"),
    }
    /*
    let sum: time::Duration = ;
    for s in screen_average {
        sum += s;
    }
    sum /= screen_average.len();
    println!("average: {sum:#?}");
*/
}

pub fn compose_screen(l1: Text, l2: Text, l3: Text, l4: Text) -> Screen{ Screen{ l1, l2, l3, l4, }}

pub fn compose_pixels(line: Text) -> Text
{
    let mut response: Text = Default::default();

    let mut buff: char = Default::default();
    for c in line.l1.chars() {
        if buff == c {
            response.l1 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l1 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    buff = Default::default();

    for c in line.l2.chars() {
        if buff == c {
            response.l2 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l2 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    buff = Default::default();

    for c in line.l3.chars() {
        if buff == c {
            response.l3 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l3 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    buff = Default::default();

    for c in line.l4.chars() {
        if buff == c {
            response.l4 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l4 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    buff = Default::default();

    for c in line.l5.chars() {
        if buff == c {
            response.l5 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l5 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    buff = Default::default();

    for c in line.l6.chars() {
        if buff == c {
            response.l6 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l6 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    buff = Default::default();

    for c in line.l7.chars() {
        if buff == c {
            response.l7 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l7 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    buff = Default::default();

    for c in line.l8.chars() {
        if buff == c {
            response.l8 += match c {
                '0' => "┼┼",
                '1' => "▯▯",
                ' ' => "  ",
                _ => todo!(),
            };
            continue;
        }
        response.l8 += match c {
            '0' => "\x1b[38;5;25m\x1b[48;5;21m┼┼",
            '1' => "\x1b[39m\x1b[48;5;32m▯▯",
            ' ' => "\x1b[48;5;17m  ",
            _ => todo!(),
        };
        buff = c;
    }
    return response;
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
