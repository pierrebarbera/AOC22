use io;

type Uint = u128;

pub fn day11(args: &[String]) {
    if args.len() != 1 {
        panic!("Expecting exactly one arg to day11, which is a valid file path.");
    }

    let mut highscore = play_monkey_keepaway(&args[0], 20, true);
    print_monkeybusiness(&highscore);

    highscore = play_monkey_keepaway(&args[0], 10000, false);
    print_monkeybusiness(&highscore);
}

fn gcd(a: Uint, b: Uint) -> Uint {
    if a == 0 {
        return b;
    }
    gcd(b % a, a)
}

fn lcd(a: Uint, b: Uint) -> Uint {
    (a * b) / gcd(a, b)
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Score {
    score: Uint,
    id: usize,
}

fn print_monkeybusiness(highscore: &Vec<Score>) {
    // let mut monkeybusiness: u128 = u128::from(highscore[0].score) * u128::from(highscore[1].score);
    let mut monkeybusiness: Uint = 1;
    for score in highscore[0..2].iter() {
        monkeybusiness *= score.score;
    }
    println!("level of monkey business: {}", monkeybusiness);
}

fn play_monkey_keepaway(filename: &str, rounds: usize, worry_div_3: bool) -> Vec<Score> {
    println!("Playing {} rounds of keep away", rounds);
    let mut monkeys = parse_monkeys(filename);

    let mut div: Uint = 0;
    let manage_worry: Box<dyn Fn(Uint) -> Uint> = match worry_div_3 {
        true => Box::new(|worry| worry / 3),
        false => {
            // get the LCD
            div = lcd(monkeys[0].divisor, monkeys[1].divisor);
            for m in monkeys[2..].iter() {
                div = lcd(div, m.divisor);
            }
            Box::new(|worry| worry % div)
        }
    };

    for _ in 0..rounds {
        for m in 0..monkeys.len() {
            while !monkeys[m].items.is_empty() {
                let (item, target) = monkeys[m].inspect(&manage_worry);

                monkeys[target].items.push(item);
            }
        }
    }

    let mut highscore: Vec<Score> = Vec::new();
    for (i, m) in monkeys.iter().enumerate() {
        let score = Score {
            id: i,
            score: m.inspect_count,
        };
        println!("Monkey {} inspected items {} times.", i, score.score);
        highscore.push(score);
    }
    // sort descending
    highscore.sort_by(|a, b| b.cmp(&a));
    highscore
}

fn parse_monkeys(filename: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    io::foreach_nonempty_line(filename, |line| {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        if !(parts.len() >= 2) {
            panic!("Invalid line {}", line);
        }
        match parts[0] {
            "Monkey" => monkeys.push(Monkey::new()),
            "Starting" => monkeys.last_mut().unwrap().parse_items(&parts[2..]),
            "Operation:" => monkeys.last_mut().unwrap().parse_operation(&parts[3..]),
            "Test:" => monkeys.last_mut().unwrap().divisor = parts[3].parse::<Uint>().unwrap(),
            "If" => monkeys.last_mut().unwrap().parse_target(parts[1], parts[5]),
            _ => panic! {"Unrecognized start of string: {}", parts[0]},
        }
    });
    monkeys
}

#[derive(Clone)]
enum Operation {
    Add(Option<Uint>),
    Mul(Option<Uint>),
}

#[derive(Clone)]
struct Monkey {
    items: Vec<Uint>,
    op: Operation,
    divisor: Uint,
    true_targ: usize,
    false_targ: usize,
    inspect_count: Uint,
}

impl Monkey {
    fn new() -> Self {
        Monkey {
            items: Vec::new(),
            op: Operation::Add(None),
            divisor: 0,
            true_targ: 0,
            false_targ: 0,
            inspect_count: 0,
        }
    }
    fn parse_items(&mut self, items: &[&str]) {
        for i in items {
            let item = i.replace(',', "").parse::<Uint>().unwrap();
            self.items.push(item);
        }
    }
    fn parse_operation(&mut self, parts: &[&str]) {
        if parts.len() != 3 {
            panic!("Operation must consist of 3 parts: {:#?}", parts);
        }
        self.op = match parts[1] {
            "+" => match parts[2].parse::<Uint>() {
                Ok(n) => Operation::Add(Some(n)),
                Err(_) => Operation::Add(None),
            },
            "*" => match parts[2].parse::<Uint>() {
                Ok(n) => Operation::Mul(Some(n)),
                Err(_) => Operation::Mul(None),
            },
            _ => panic!("Invalid operation: {}", parts[1]),
        }
    }
    fn parse_target(&mut self, cond: &str, target: &str) {
        match cond {
            "true:" => self.true_targ = target.parse::<usize>().unwrap(),
            "false:" => self.false_targ = target.parse::<usize>().unwrap(),
            _ => panic!("Invalid conditional: {}", cond),
        }
    }
    fn operation(&self, old: Uint) -> Uint {
        match &self.op {
            Operation::Add(n) => match n {
                Some(n) => old + n,
                None => old + old,
            },
            Operation::Mul(n) => match n {
                Some(n) => old * n,
                None => old * old,
            },
        }
    }
    fn inspect<F>(&mut self, manage_worry: F) -> (Uint, usize)
    where
        F: Fn(Uint) -> Uint,
    {
        let mut item = self.items.pop().unwrap();
        self.inspect_count += 1;
        item = self.operation(item);
        item = manage_worry(item);
        let target_monkey = if item % Uint::from(self.divisor) == 0 {
            self.true_targ
        } else {
            self.false_targ
        };
        (item, target_monkey)
    }
}
