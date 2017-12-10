use file;

pub fn run() {
    let inputs = file::read_inputs("Day7.txt");

    let test = "pbga (66)
    xhth (57)
    ebii (61)
    havc (66)
    ktlj (57)
    fwft (72) -> ktlj, cntj, xhth
    qoyq (66)
    padx (45) -> pbga, havc, qoyq
    tknk (41) -> ugml, padx, fwft
    jptl (61)
    ugml (68) -> gyxo, ebii, jptl
    gyxo (61)
    cntj (57)";

    solve(&test.to_string());
    solve(&inputs.to_string());
}

#[derive(Clone, Debug, PartialEq)]
struct Program<'a> {
    name: String,
    weight: u32,
    combined_weight: u32,
    above_str: Vec<&'a str>,
    above: Vec<Box<Program<'a>>>,
}

fn solve(inputs: &str) -> u32 {
    let lines: Vec<Vec<&str>> = inputs
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().collect())
        .collect();

   // println!("Inputs {:?}", lines);

    let mut programs: Vec<Program> = Vec::with_capacity(lines.len());
    let mut program_tree: Program;

    for line in lines.iter() {
        let t: Vec<&str> = line[1].split('(').collect();
        let u: Vec<&str> = t[1].split(')').collect();
        let mut program = Program {
            name: line[0].to_string(),
            weight: u[0].parse::<u32>().unwrap(),
            combined_weight: 0,
            above_str: Vec::new(),
            above: Vec::new()
        };

        for i in 3..line.len() {
        let program_match;
        if i == line.len() - 1 {
            program_match = line[i];
        } else {
            program_match = line[i].split_at(line[i].len() - 1).0;
        }
            program.above_str.push(program_match);
        }

       // println!("Program {}, {}, {:?}", program.name, program.weight, program.above_str);
        programs.push(program.clone());
    }

    let mut programs_copy: Vec<Program> = programs.clone();

    for i in 0..programs.len() {
        for above in &programs[i].above_str {
            let mut index = 0;
            'program_search: for (j, find_prog) in programs_copy.iter().enumerate() {
                if (*find_prog.name).eq(*above) {
                    index = j;

                    break 'program_search;
                }
            }

            programs_copy.remove(index);
        }
    }

    let mut index = 0;
    for i in 0..programs.len() {
        if programs[i].name == programs_copy[0].name {
            index = i;
            break;
        }
    }

    program_tree = programs[index].clone();
    println!("Start {}", program_tree.name);

    program_tree.build_tree(&mut programs);
    program_tree.calculate_weights()
}

impl<'a> Program<'a> {
    pub fn build_tree(&mut self, programs: &'a Vec<Program<'a>>) {
      /*  println!("Found name {}, {}", self.name, self.above_str.len());
        let mut index = 0;
        for i in 0..programs.len() {
            if self.name == programs[i].name {
                index = i;

                break;
            }
        }*/

       // programs.remove(index);

        if self.above_str.len() == 0 {
           // println!("GO BACK");
            return;
        }

        for above in &self.above_str {
            let mut index = 0;
            'program_search: for (j, find_prog) in programs.iter().enumerate() {
                if (*find_prog.name).eq(*above) {
                    index = j;
                    break 'program_search;
                }
            }

            let mut new_prog = Box::new(Program {
                name: programs[index].name.clone(),
                weight: programs[index].weight,
                combined_weight: 0,
                above_str: programs[index].above_str.clone(),
                above: Vec::new()
            });
         //   println!("Found {}, {}", new_prog.name, new_prog.above_str.len());

            new_prog.build_tree(programs);

            self.above.push(new_prog.clone());
        }

        return;
    }

    pub fn calculate_weights(&mut self) -> u32 {
        let mut weights = Vec::new();
        for i in 0..self.above.len() {
            let weight = self.above[i].calculate_weights();
            weights.push(weight);
            self.combined_weight += weight;
        }
        if self.above.len() > 0 {
          //  println!("\nNAME {}, above {}", self.name, self.above.len());
        }

        let mut index = 0;
        let mut wrong_index = 0;
        for i in 0..weights.len() {
           // print!(" {} ", weights[i]);
            if self.combined_weight % weights[i] != 0 {
                if weights[index] != weights[i] {
                    wrong_index = i;
                }

                if wrong_index > 0 {
                    if weights[wrong_index] == weights[i] {
                        wrong_index = index;
                        index = i;
                    }
                }
            }
        }

        if index > 0 {
            let diff: i32 = weights[index] as i32 -weights[wrong_index] as i32;
            println!("This {}, {}", diff, self.name);

            println!("Above {} {}", self.above[wrong_index].weight as i32 + diff as i32, self.above[wrong_index].name);

        }

       // println!("{}, {}", self.name, self.weight + self.combined_weight);

        return self.weight + self.combined_weight;
    }
}