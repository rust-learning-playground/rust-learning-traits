use std::fmt;
use rand::Rng;

enum AnimalColor {
    White,
    Brown,
    Gray,
    Cream,
    Black,
}

impl fmt::Display for AnimalColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnimalColor::White => { write!(f, "White") },
            AnimalColor::Brown => { write!(f, "Brown") },
            AnimalColor::Gray  => { write!(f, "Gray") }, 
            AnimalColor::Cream => { write!(f, "Cream") },
            AnimalColor::Black => { write!(f, "Black") },
        }
    }
}

pub trait Sound {
    fn sound(&self) -> &str;
}

pub trait Info {
    fn info(&self) -> String;
}

struct Fly {
    sound: String,
    legs: u8,
}

impl Fly {
    fn new() -> Fly {
        Fly {
            sound: "bzbzbzbzbz".to_string(),
            legs: 6,
        }
    }
}

struct Horse {
    name: String,
    sound: String,
    color: AnimalColor,
    legs: u8,
}

impl Horse {
    pub fn new(name: &str, color: AnimalColor) -> Horse {
        Horse {
            name: name.to_string(),
            sound: "Hihihihi".to_string(),
            color,
            legs: 4,
        }
    }
}

struct Cow {
    name: String,
    sound: String,
    color: AnimalColor,
    legs: u8,
}

impl Cow {
    pub fn new(name: &str, color: AnimalColor) -> Cow {
        Cow {
            name: name.to_string(),
            sound: "Muuuuuuu".to_string(),
            color,
            legs: 4,
        }
    }
}

struct Animal<T>
{
    animal : T
}

impl<T: Info> Animal<T> {
    fn info (&self) -> String {
        self.animal.info()
    }
}

impl Sound for Horse {
    fn sound (&self) -> &str {
        &self.sound
    }
}

impl Sound for Cow {
    fn sound (&self) -> &str {
        &self.sound
    }
}

impl Sound for Fly {
    fn sound (&self) -> &str {
        &self.sound
    }
}

impl Info for Horse {
    fn info (&self) -> String {
        return format!("I am a horse, my name is:{}, I have:{} legs, I make:\"{}\" sound \
                        and my color is:{}", self.name, self.legs, self.sound(),
                       self.color);
    }
}

impl Info for Cow {
    fn info (&self) -> String {
        return format!("I am a cow, my name is:{}, I have:{} legs, I make:\"{}\" sound \
                        and my color is:{}", self.name, self.legs, self.sound(),
                       self.color);
    }
}

impl Info for Fly {
    fn info (&self) -> String {
        return format!("I am a fly, I have:{} legs, I make:\"{}\" sound",
                       self.legs, self.sound());
    }
}

fn random_color() -> AnimalColor {
    match rand::thread_rng().gen_range(1..6) {
        1 => AnimalColor::White,
        2 => AnimalColor::Brown,
        3 => AnimalColor::Gray,
        4 => AnimalColor::Cream,
        5 => AnimalColor::Black,
        _ => panic!(),
    }
}

fn go() {
    let rocinante = Animal::<Horse>{ animal : Horse::new("Rocinante", random_color()) };
    let florinda = Animal::<Cow>{ animal : Cow::new("Florinda", random_color()) };
    let fly1 = Animal::<Fly>{ animal : Fly::new() };
    println!("{}", rocinante.info());
    println!("{}", florinda.info());
    println!("{}", fly1.info());    
}

fn main() {
    go();
}
