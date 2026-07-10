
#[derive(Default)]
struct Text {
    l1: String,
    l2: String,
    l3: String,
    l4: String,
    l5: String,
    l6: String,
    l7: String,
    l8: String,
}

struct Screen {
    l1: Text,
    l2: Text,
    l3: Text,
    l4: Text,
}

fn main() {
    let line_one: Text = compose_line("hola cet dos".to_string());
    let line_two: Text = compose_line("soy el timbre".to_string());
    let line_three: Text = compose_line("rust".to_string());
    let line_four: Text = compose_line("linux".to_string());

    let screen: Screen = compose_screen(line_one, line_two, line_three, line_four);

    draw_screen(screen);
}

fn compose_screen(l1: Text, l2: Text, l3: Text, l4: Text) -> Screen
{
    Screen {
        l1,
        l2,
        l3,
        l4,
    }
}

fn compose_pixels


fn compose_line(mut message: String) -> Text
{
    if message.len() != 20 {
        if message.len() > 20 {
            panic!("Too large line!");
        }else{
            while message.len() < 20 {
                message += " ";
            }
        }
    }

    let abc: Vec<Text> = vec!{
        Text{ // "a"
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "11110".to_string(),
            l4: "00001".to_string(),
            l5: "11111".to_string(),
            l6: "10001".to_string(),
            l7: "11111".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "b"               
            l1: "00000".to_string(),
            l2: "10000".to_string(),
            l3: "11110".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "10001".to_string(),
            l7: "11110".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "c"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "01111".to_string(),
            l4: "10000".to_string(),
            l5: "10000".to_string(),
            l6: "10000".to_string(),
            l7: "01111".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // "d"
            l1: "00000".to_string(),
            l2: "00001".to_string(),
            l3: "01111".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "10001".to_string(),
            l7: "01111".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "e"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "01110".to_string(),
            l4: "10001".to_string(),
            l5: "11111".to_string(),
            l6: "10000".to_string(),
            l7: "01111".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "f"               
            l1: "00000".to_string(),
            l2: "00111".to_string(),
            l3: "01000".to_string(),
            l4: "01110".to_string(),
            l5: "01000".to_string(),
            l6: "01000".to_string(),
            l7: "01000".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // "g"
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "11111".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "11111".to_string(),
            l7: "00001".to_string(),
            l8: "11110".to_string(),
        },                         
        Text{ // "h"               
            l1: "00000".to_string(),
            l2: "10000".to_string(),
            l3: "10000".to_string(),
            l4: "11110".to_string(),
            l5: "10001".to_string(),
            l6: "10001".to_string(),
            l7: "10001".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "i"               
            l1: "00000".to_string(),
            l2: "00100".to_string(),
            l3: "00000".to_string(),
            l4: "11100".to_string(),
            l5: "00100".to_string(),
            l6: "00100".to_string(),
            l7: "11111".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // "j"
            l1: "00000".to_string(),
            l2: "00001".to_string(),
            l3: "00000".to_string(),
            l4: "00111".to_string(),
            l5: "00001".to_string(),
            l6: "00001".to_string(),
            l7: "00001".to_string(),
            l8: "11110".to_string(),
        },                         
        Text{ // "k"               10
            l1: "00000".to_string(),
            l2: "10000".to_string(),
            l3: "10001".to_string(),
            l4: "10010".to_string(),
            l5: "11100".to_string(),
            l6: "10010".to_string(),
            l7: "10001".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "l"               
            l1: "00000".to_string(),
            l2: "11000".to_string(),
            l3: "01000".to_string(),
            l4: "01000".to_string(),
            l5: "01000".to_string(),
            l6: "01000".to_string(),
            l7: "00111".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // "m"
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "10000".to_string(),
            l4: "11111".to_string(),
            l5: "10101".to_string(),
            l6: "10101".to_string(),
            l7: "10101".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "n"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "10110".to_string(),
            l4: "11001".to_string(),
            l5: "10001".to_string(),
            l6: "10001".to_string(),
            l7: "10001".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "ñ"               
            l1: "01110".to_string(),
            l2: "00000".to_string(),
            l3: "10110".to_string(),
            l4: "11001".to_string(),
            l5: "10001".to_string(),
            l6: "10001".to_string(),
            l7: "10001".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // "o"
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "00000".to_string(),
            l4: "01110".to_string(),
            l5: "10001".to_string(),
            l6: "10001".to_string(),
            l7: "01110".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "p"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "11110".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "11110".to_string(),
            l7: "10000".to_string(),
            l8: "10000".to_string(),
        },                         
        Text{ // "q"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "01111".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "01111".to_string(),
            l7: "00001".to_string(),
            l8: "00001".to_string(),
        },
        Text{ // "r"
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "11110".to_string(),
            l4: "10001".to_string(),
            l5: "10000".to_string(),
            l6: "10000".to_string(),
            l7: "00000".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "s"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "01111".to_string(),
            l4: "10000".to_string(),
            l5: "01110".to_string(),
            l6: "00001".to_string(),
            l7: "11110".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "t"               
            l1: "00000".to_string(),
            l2: "00100".to_string(),
            l3: "11111".to_string(),
            l4: "00100".to_string(),
            l5: "00100".to_string(),
            l6: "00100".to_string(),
            l7: "00011".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // "u"
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "10001".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "10001".to_string(),
            l7: "01111".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "v"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "10001".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "01010".to_string(),
            l7: "00100".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "w"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "10001".to_string(),
            l4: "10001".to_string(),
            l5: "10101".to_string(),
            l6: "01010".to_string(),
            l7: "01010".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // "x"
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "10001".to_string(),
            l4: "01010".to_string(),
            l5: "00100".to_string(),
            l6: "01010".to_string(),
            l7: "10001".to_string(),
            l8: "00000".to_string(),
        },                         
        Text{ // "y"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "10001".to_string(),
            l4: "10001".to_string(),
            l5: "10001".to_string(),
            l6: "01111".to_string(),
            l7: "00001".to_string(),
            l8: "11110".to_string(),
        },                         
        Text{ // "z"               
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "11111".to_string(),
            l4: "00010".to_string(),
            l5: "00100".to_string(),
            l6: "01000".to_string(),
            l7: "11111".to_string(),
            l8: "00000".to_string(),
        },
        Text{ // " "
            l1: "00000".to_string(),
            l2: "00000".to_string(),
            l3: "00000".to_string(),
            l4: "00100".to_string(),
            l5: "00100".to_string(),
            l6: "00000".to_string(),
            l7: "00000".to_string(),
            l8: "00000".to_string(),
        },
    };

    println!("{}", abc.len());
    let mut blocks: Vec<u8> = vec![];
    for char in message.chars() {
        println!("[{char}]");
        blocks.push( match char {
            'a' => 0 ,
            'b' => 1 ,
            'c' => 2 ,
            'd' => 3 ,
            'e' => 4 ,
            'f' => 5 ,
            'g' => 6 ,
            'h' => 7 ,
            'i' => 8 ,
            'j' => 9 ,
            'k' => 10 ,
            'l' => 11 ,
            'm' => 12 ,
            'n' => 13 ,
            'ñ' => 14 ,
            'o' => 15 ,
            'p' => 16 ,
            'q' => 17 ,
            'r' => 18 ,
            's' => 19 ,
            't' => 20 ,
            'u' => 21 ,
            'v' => 22 ,
            'w' => 23 ,
            'x' => 24 ,
            'y' => 25 ,
            'z' => 26 ,
            ' ' => 27 ,
            _ => todo!("No a printable character!, {}", char),
        });
    }

    let mut message_composed: Text = Default::default();

    for c in blocks {
        println!("{c}");
        message_composed.l1 += &(" ".to_owned() + &abc[c as usize].l1.clone());
        message_composed.l2 += &(" ".to_owned() + &abc[c as usize].l2.clone());
        message_composed.l3 += &(" ".to_owned() + &abc[c as usize].l3.clone());
        message_composed.l4 += &(" ".to_owned() + &abc[c as usize].l4.clone());
        message_composed.l5 += &(" ".to_owned() + &abc[c as usize].l5.clone());
        message_composed.l6 += &(" ".to_owned() + &abc[c as usize].l6.clone());
        message_composed.l7 += &(" ".to_owned() + &abc[c as usize].l7.clone());
        message_composed.l8 += &(" ".to_owned() + &abc[c as usize].l8.clone());
    }
    return message_composed;
}

fn empty() -> String {
    let mut responce: String = Default::default();
    for _ in 0..=240 {
        responce += " ";
    }
    return responce;
}

fn draw_screen(message_composed: Screen) // 36 * 120 pixels (4 * 20 characters)
{
    let empty: String = empty();
    print!("\n");
    println!("\x1b[48;5;248m                                                                                                                                        \x1b[0m");
    println!("\x1b[48;5;248m                                                                                                                                      \x1b[48;5;240m  \x1b[0m");
    println!("\x1b[48;5;248m    \x1b[48;5;245m                                                                                                                                \x1b[48;5;240m    \x1b[0m");
    println!("\x1b[48;5;248m    \x1b[48;5;245m                                                8 * 49 Screen                                                                   \x1b[48;5;240m    \x1b[0m");
    println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m                                                                                                                        \x1b[48;5;245m    \x1b[48;5;248m\x1b[48;5;240m    \x1b[0m");

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l1.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l2.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l3.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l1);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l2);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l3);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l4);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l5);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l6);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l7);
            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", message_composed.l4.l8);

            println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m  {}\x1b[48;5;239m  \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m", empty);
    println!("\x1b[48;5;248m    \x1b[48;5;245m    \x1b[48;5;239m                                                                                                                        \x1b[48;5;245m    \x1b[48;5;240m    \x1b[0m");
    println!("\x1b[48;5;248m    \x1b[48;5;245m                                                                                                                                \x1b[48;5;240m    \x1b[0m");
    println!("\x1b[48;5;248m    \x1b[48;5;245m                                                                                                                                \x1b[48;5;240m    \x1b[0m");
    println!("\x1b[48;5;248m  \x1b[48;5;240m                                                                                                                                      \x1b[0m");
    println!("\x1b[48;5;240m                                                                                                                                        \x1b[0m");
}

/*
fn sleeping()
{
    // Like in suspention mode, without ringing, but buzzing.
}*/
