use std::collections::{HashMap, VecDeque};
use num_integer::Integer;

pub fn part1(input: &str) -> String {
    let mut network = parse_network(input);
    let mut lows = 0;
    let mut highs = 0;
    for _ in 0..1000 {
        lows += 1;
        network.press_button(|_, _, pulse| {
            if pulse {
                highs += 1;
            }
            else {
                lows += 1;
            }
        });
    }
    let product = lows * highs;
    return format!("Product of pulses: {}", product);
}

pub fn part2(input: &str) -> String {
    let mut network = parse_network(input);
    let rx_conjunction = network.rx_conjunction;
    let mut presses = 0;
    let mut searching = true;
    let mut cycles = vec!();
    while searching {
        presses += 1;
        network.press_button(|cable_i, module_i, pulse| {
            if module_i == rx_conjunction && pulse {
                if matches!(cycles.first(), Some(&(i, _)) if i == cable_i) {
                    searching = false;
                }
                else {
                    cycles.push((cable_i, presses));
                }
            }
        });
    }
    let presses = cycles.iter().fold(1usize, |p, (_, c)| p.lcm(c));
    return format!("Button presses: {}", presses);
}

fn parse_network(text: &str) -> Network {
    let mut cables = vec!();
    let mut modules = vec!();
    let mut module_map = HashMap::new();
    let mut broadcaster = 0;
    let mut rx_conjunction = 0;
    for line in text.lines() {
        let (description, outputs_text) = line.split_once(" -> ").unwrap();
        let (state, name) = match &description[0..1] {
            "%" => (State::FlipFlop(false), &description[1..]),
            "&" => (State::Conjunction(0), &description[1..]),
            _ => (State::Broadcaster, description),
        };
        let mut outputs = vec!();
        for output in outputs_text.split(", ") {
            let cable_i = cables.len();
            cables.push(false);
            let output_i = *module_map.entry(output).or_insert(modules.len());
            if output_i == modules.len() {
                modules.push(Module::default());
            }
            modules[output_i].inputs.push(cable_i);
            outputs.push((cable_i, output_i));
        }
        let i = *module_map.entry(name).or_insert(modules.len());
        if i == modules.len() {
            modules.push(Module::default());
        }
        if let State::Broadcaster = state {
            broadcaster = i;
        }
        if outputs_text == "rx" {
            rx_conjunction = i;
        }
        let module = &mut modules[i];
        module.state = state;
        module.outputs = outputs;
    }
    return Network { cables, modules, broadcaster, rx_conjunction };
}

struct Network {
    cables: Vec<bool>,
    modules: Vec<Module>,
    broadcaster: usize,
    rx_conjunction: usize,
}

impl Network {
    fn press_button<F>(&mut self, mut process_pulse: F)
    where F: FnMut(usize, usize, bool)
    {
        let mut pulses = VecDeque::new();
        for &(cable_i, module_i) in &self.modules[self.broadcaster].outputs {
            pulses.push_back((cable_i, module_i, false));
        }
        while let Some((cable_i, module_i, pulse)) = pulses.pop_front() {
            process_pulse(cable_i, module_i, pulse);
            let module = &mut self.modules[module_i];
            let mut next = None;
            if let State::FlipFlop(on) = module.state {
                if !pulse {
                    module.state = State::FlipFlop(!on);
                    next = Some(!on);
                }
            }
            if let State::Conjunction(mut memory) = module.state {
                memory = memory.checked_add_signed(match (self.cables[cable_i], pulse) {
                    (false, true) => 1,
                    (true, false) => -1,
                    _ => 0,
                }).unwrap();
                module.state = State::Conjunction(memory);
                self.cables[cable_i] = pulse;
                next = Some(memory != module.inputs.len());
            }
            if let Some(pulse) = next {
                for &(cable_i, module_i) in &module.outputs {
                    pulses.push_back((cable_i, module_i, pulse));
                }
            }
        }
    }
}

#[derive(Default)]
struct Module {
    state: State,
    inputs: Vec<usize>,
    outputs: Vec<(usize, usize)>,
}

#[derive(Default)]
enum State {
    #[default]
    Untyped,
    FlipFlop(bool),
    Conjunction(usize),
    Broadcaster,
}
