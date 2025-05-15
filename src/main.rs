use std::{fmt::Display, fs::File, io::{Read, Write}};

const ORCODE: &str = "OR";
const ANDCODE: &str = "AND";
const NOTCODE: &str = "NOT";
const NORCODE: &str = "NOR";
const XORCODE: &str = "XOR";
const NANDCODE: &str = "NAND";
const NXORCODE: &str = "NXOR";
const INPUTCODE: &str = "INPUT";
const OUTPUTCODE: &str = "OUTPUT";
const BUFFERCODE: &str = "BUFFER";

const DELETECOMPONENTCODE: &str = "DEL";
const MANUALSETSTATECODE: &str = "SET";
const NEWCIRCUITCODE: &str = "NEW";
const COMPILECIRCUITCODE: &str = "COMPILE";
const IMPORTCIRCUITCODE: &str = "IMPORT";
const EDITCOMPONENTCODE: &str = "EDIT";
const NAMECIRCUITCODE: &str = "NAME";
const SAVECIRCUITCODE: &str = "SAVE";
const LOADCIRCUITCODE: &str = "LOAD";
const LOADICCODE: &str = "IC";

const ENDPOINT: u8 = ';' as u8;
const WHITESPACE: u8 = ' ' as u8;

fn main() {
    create_circuit(&mut 0, &mut Vec::new());
}

fn create_circuit(id: &mut u32, catalogue: &mut Vec<Circuit>) {

    *id += 1;

    let mut circuit = Circuit::new(*id);

    'game: loop {
        circuit.step();

        let command = input(&format!("{}>", circuit.id));
        if command == "HLT" {
            break 'game;
        }
        if command == "DISPLAY" {
            circuit.display();
        }
        if command == "DISPLIO" {
            circuit.displio();
        }
        if command == "CATALOGUE" {
            for c in catalogue.iter() {
                if let Some(name) = &c.name {
                    println!("[{}] {} - {} Gates(s)", c.id, name, c.gates.len())
                }
                else {
                    println!("[{}] - {} Gate(s)", c.id, c.gates.len())
                }
            }
        }
        if command == "HELP" {
            println!("DEL [id..]                             - Deletes the given components");
            println!("SET [id..] (TRUE/FALSE)                - Sets the state of the given components");
            println!("NEW                                    - Starts a new circuit");
            println!("COMPILE                                - Adds the circuit to the catalogue");
            println!("IMPORT [id]                            - Adds a circuit to the current circuit");
            println!("EDIT [id] [Gate Type] [id...]; [Label] - Swaps components with a new one");
            println!("NAME [name]                            - Sets the name of the circuit");
            println!("SAVE [file path]                       - Saves a circuit to the given location");
            println!("LOAD [file path]                       - Loads the circuit from a file into the catalogue");
            println!("HLT                                    - Quits the current circuit and goes back to the previous one");
            println!("DISPLAY                                - Shows the status of all the gates in the circuit");
            println!("DISPLIO                                - Shows the status of all the input and output components in the circuit");
            println!("CATALOGUE                              - Shows the circuits in the catalogue");
        }


        // New component format is [Gate Type] [Inputs]; [LABEL]
        let mut buffer = Reader::new(command);
        let mut char = buffer.pop();
        let mut word: Vec<u8> = Vec::new();
        
        let mut sentence: Vec<Vec<u8>> = Vec::new();
        let mut note: Vec<Vec<u8>> = Vec::new();

        let mut note_mode = false;

        while let Some(c) = char {
            match c {
                ENDPOINT    => if note_mode {word.push(c)} else {sentence.push(word.clone()); word.clear(); note_mode = true},
                WHITESPACE  => { if word.len() > 0 { if !note_mode {sentence.push(word.clone())} else {note.push(word.clone());}; word.clear() }},
                _           => word.push(c),
            }
            char = buffer.pop();
        }

        if !note_mode {sentence.push(word.clone())} else {note.push(word.clone());}; word.clear();

        if sentence.len() > 0 {
            let word = sentence.remove(0);
            if let Ok(t) = String::from_utf8(word) {

                if t == MANUALSETSTATECODE {
                    let mut stuff_to_set = Vec::new();
                    for word in sentence[0..(sentence.len()-1)].iter() {
                        if let Ok(s) = String::from_utf8(word.to_vec()) {
                            if let Ok(id) = u32::from_str_radix(&s, 10) {
                                stuff_to_set.push(id);
                            }
                        }
                    }
                    let states = String::from_utf8(sentence.last().unwrap().to_vec());

                    if let Ok(s) = states {
                        let state = if [String::from("TRUE"), String::from("ON")].contains(&s) {
                            Some(true)
                        } else if [String::from("FALSE"), String::from("OFF")].contains(&s) {
                            Some(false)
                        } else { println!("Invalid state: \"{}\"", s); Option::None};
                        if let Some(state) = state {
                            for id in stuff_to_set {
                                if !circuit.set_component(id, state) {
                                    print!("Component {} does not exist\n", id);
                                }
                                else {
                                    println!("Set {} to {}", id, state);
                                }
                            }
                        } else {println!("Invalid state\n")}
                    } else {println!("{:?}", states)}

                }
                else if t == NEWCIRCUITCODE {
                    println!("New circuit");
                    create_circuit(id, catalogue);
                }
                else if t == COMPILECIRCUITCODE { 
                    circuit.normalize(); 
                    catalogue.push(circuit.clone()); 
                    println!("Compiled Circuit")
                }
                else if t == LOADCIRCUITCODE {
                    sentence.append(&mut note);
                    let fpbytes = sentence.join(&WHITESPACE);
                    if let Ok(fp) = String::from_utf8(fpbytes) {
                        if fp.len() == 0 {
                            println!("Enter a file path for the circuit");
                            continue;
                        }
                        
                        *id += 1;
                        let mut new_circuit = Circuit::new(*id);
                        match new_circuit.load_from_file(&fp) {
                            Ok(()) => {
                                println!("Loaded Circuit from {}", fp);
                                catalogue.push(new_circuit);
                            }
                            Err(e) => {
                                println!("Failed to load circuit from {}\n{}", fp, e);
                                *id -= 1;                            
                            }
                        }
                    } else {
                        println!("Problem with path");
                    }
                }
                else if t == SAVECIRCUITCODE {
                    sentence.append(&mut note);
                    let fpbytes = sentence.join(&WHITESPACE);
                    if let Ok(fp) = String::from_utf8(fpbytes) {
                        if fp.len() == 0 {
                            println!("Enter a file path for the circuit");
                            continue;
                        }
                        match circuit.save_to_file(&fp) {
                            Ok(()) => println!("Saved Circuit to {}", fp),
                            Err(e) => println!("Failed to save circuit to {}\n{}", fp, e),
                        }
                    } else {
                        println!("Problem with path");
                    }
                }
                else if t == DELETECOMPONENTCODE { 
                    for word in sentence[0..sentence.len()].iter() {
                        if let Ok(s) = String::from_utf8(word.to_vec()) {
                            if let Ok(id) = u32::from_str_radix(&s, 10) {
                                if !circuit.delete_component(id) {
                                    println!("Component {} does not exist in this circuit", id);
                                }
                            }
                        }
                    }
                    
                }
                else if t == EDITCOMPONENTCODE {
                    if sentence.len() < 2 {
                        println!("Not enough parameters");
                        continue;
                    }
                    let word = sentence.remove(0);
                    if let Ok(s) = String::from_utf8(word.to_vec()) {
                        if let Ok(id) = u32::from_str_radix(&s, 10) {
                            
                            let gate_type_key = sentence.remove(0);
                            if let Ok(token) = String::from_utf8(gate_type_key) {
                                let new_gate = parse_gate(token, sentence, note);
                                if let Some(gate) = new_gate {
                                    circuit.edit_component(id, gate.0, gate.1, gate.2);
                                    continue;
                                }
                            }
                        }
                    }
                    println!("There is an issue with the command");
                }
                else if t == IMPORTCIRCUITCODE {
                    let word = sentence.remove(0);
                    if let Ok(s) = String::from_utf8(word.to_vec()) {
                        if let Ok(id) = u32::from_str_radix(&s, 10) {
                            
                            for item in catalogue.iter() {
                                if item.id == id {
                                    circuit.import_circuit(item.clone());
                                    break;
                                }
                            }
                        }
                    }
                }
                else if t == NAMECIRCUITCODE {
                    sentence.append(&mut note);
                    if let Ok(name) = String::from_utf8(sentence.join(&WHITESPACE)) {
                        circuit.name = Some(name.clone());
                        println!("Changed name of current circuit to {}", name);
                    }
                }
                else if t == LOADICCODE {
                    let word = sentence.remove(0);
                    if let Ok(s) = String::from_utf8(word.to_vec()) {
                        if let Ok(id) = u32::from_str_radix(&s, 10) {
                            
                            for item in catalogue.iter() {
                                if item.id == id {
                                    circuit.add_intergrated_circuit(item.clone(), Vec::new(), Vec::new());
                                    break;
                                }
                            }
                        }
                    }
                }
                else {
                    let new_gate = parse_gate(t, sentence, note);
                    if let Some(gate) = new_gate {
                        circuit.add_component(gate.0, gate.1, gate.2);
                    }
                }
            }
        }



    }
}

fn parse_gate(gate_key: String, sentence: Vec<Vec<u8>>, note: Vec<Vec<u8>>) -> Option<(GateType, Vec<u32>, Option<String>)> {
    let mut gate_type: Option<GateType> = Option::None;

    if gate_key == ANDCODE { gate_type = Some(GateType::And) }
    else if gate_key == ORCODE { gate_type = Some(GateType::Or) }
    else if gate_key == NOTCODE { gate_type = Some(GateType::Not) }
    else if gate_key == NORCODE { gate_type = Some(GateType::Nor) }
    else if gate_key == XORCODE { gate_type = Some(GateType::Xor) }
    else if gate_key == NANDCODE { gate_type = Some(GateType::Nand) }
    else if gate_key == NXORCODE { gate_type = Some(GateType::Nxor) }
    else if gate_key == INPUTCODE { gate_type = Some(GateType::Input) }
    else if gate_key == OUTPUTCODE { gate_type = Some(GateType::Output) }
    else if gate_key == BUFFERCODE { gate_type = Some(GateType::Buffer) }



    if let Some(gate_type) = gate_type {
        let mut inputs: Vec<u32> = Vec::new();
        let mut sentence = sentence.clone();
        while sentence.len() > 0 {
            if let Ok(val) = String::from_utf8(sentence.remove(0)) {
                if let Ok(id) = u32::from_str_radix(&val, 10) {
                    inputs.push(id);
                }
            }
        }

        let note: Option<String> = {
            let note_bytes = note.join(&WHITESPACE);
            if note_bytes.len() > 0 {
                if let Ok(n) = String::from_utf8(note_bytes) {
                    Some(n)
                }
            else { Option::None } } else { Option::None }
        };

        Some((gate_type, inputs, note))
    }
    else {
        Option::None
    }
}

struct Reader {
    buf: String
}

impl Reader {
    fn new(buf: String) -> Self {
        Self { buf }
    }
    fn pop(&mut self) -> Option<u8> {
        if !self.buf.is_empty() {
            return Some(self.buf.remove(0) as u8)
        }
        Option::None
    }
}

#[derive(Clone, Debug, PartialEq)]
enum GateType {
    Input,
    Output,
    Buffer,
    Not,
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Nxor,
}


#[derive(Clone, Debug, PartialEq)]
struct Circuit {
    id: u32,
    name: Option<String>,
    id_counter: u32,
    gates: Vec<Gate>,
    intergrated_circuits: Vec<IC>,
}
impl Circuit {
    fn new(id: u32) -> Self {
        Self { name: Option::None, id, id_counter: 0, gates: Vec::new(), intergrated_circuits: Vec::new() }
    }
    fn add_component(&mut self, gate_type: GateType, input_ids: Vec<u32>, label: Option<String>) {
        let inputs = input_ids.iter().map(|id| {return (self.gates.len(), *id)}).collect();
        self.gates.push(Gate::new(gate_type, self.id_counter, inputs, label));
        self.id_counter += 1;
    }
    fn step(&mut self) {
        let previous_state = self.gates.clone();
        for gate in self.gates.iter_mut() {
            let mut inputs = vec![false; gate.inputs.len()];
            let mut updated_inputs = gate.inputs.clone();

            for (i, data) in gate.inputs.iter().enumerate() {
                if data.0 >= previous_state.len() {continue}
                for test_index in (0..=data.0.min(previous_state.len()-1)).rev() {
                    if previous_state[test_index].id == data.1 {
                        inputs[i] = previous_state[test_index].state;
                        updated_inputs[i] = (test_index, previous_state[test_index].id);
                        break
                    }
                    else if test_index == 0 {
                        updated_inputs[i] = (test_index, previous_state.len() as u32 - 1);
                    }
                }
            }

            let new_state = gate.get_new_state(inputs);
            gate.update_state(new_state);

            gate.inputs = updated_inputs;
        }
        for ic in self.intergrated_circuits.iter_mut() {

        }
    }
    fn set_component(&mut self, id: u32, state: bool) -> bool {
        for gate in self.gates.iter_mut() {
            if gate.id == id {
                gate.state = state;
                return true
            }
        }
        false
    }
    fn display(&self) {
        for gate in self.gates.iter() {
            println!("{}", gate);
        }
    }
    fn displio(&self) {
        for gate in self.gates.iter() {
            if [GateType::Input, GateType::Output].contains(&gate.gate_type) {
                println!("{}", gate);
            }
        }
    }
    fn delete_component(&mut self, id: u32) -> bool {
        for (i, gate) in self.gates.iter().enumerate() {
            if gate.id == id {
                self.gates.remove(i);
                return true
            }
        }
        false
    }
    fn edit_component(&mut self, id: u32, gate_type: GateType, input_ids: Vec<u32>, label: Option<String>) -> bool {
        for (i, gate) in self.gates.iter().enumerate() {
            if gate.id == id {
                let inputs = input_ids.iter().map(|id| {return (self.gates.len(), *id)}).collect();
                let new_gate = Gate::new(gate_type, id, inputs, label);
                self.gates[i] = new_gate;
                return true
            }
        }
        false
    }
    fn normalize(&mut self) {
        let mut table: Vec<(u32, u32)> = self.gates.iter().map(|g| (g.id, 0)).collect();

        for (next, gate) in self.gates.iter_mut().enumerate() {
            table[next] = (gate.id, next as u32);
            gate.id = next as u32;
        }
        println!("{:?}", table);
        for gate in self.gates.iter_mut() {
            for (i, input) in gate.clone().inputs.iter().enumerate() {
                for modifier in table.iter() {
                    if input.1 == modifier.0 {
                        println!("-{:?}", gate.inputs[i]);
                        gate.inputs[i] = (input.0, modifier.1);
                        println!("+{:?}", gate.inputs[i]);
                    }
                }
            }
        }
    }
    fn import_circuit(&mut self, other_circuit: Self) {
        let gates: Vec<(GateType, Vec<u32>, Option<String>)> = other_circuit.gates.iter().map(|gate| {
            let gate_data = gate.data();
            let inputs = gate_data.1.iter().map(|i| i+self.id_counter).collect();
            (gate_data.0, inputs, gate_data.2)
        }).collect();
        for gate in gates {
            self.add_component(gate.0, gate.1, gate.2);
        }
    } 
    fn save_to_file(&self, fp: &str) -> std::io::Result<()> {
        const NEWLINE: u8 = '\n' as u8;
        let mut buf: Vec<u8> = Vec::new();
        if let Some(name) = &self.name {
            buf.push('#' as u8);
            buf.extend(name.as_bytes());
            buf.push(NEWLINE);
        }

        for gate in self.gates.iter() {
            let data = gate.data();
            if let Some(label) = data.2 {
                write!(buf, "{:?}{:?}{}\n", data.0, data.1, label)?;
            }
            else {
                write!(buf, "{:?}{:?}\n", data.0, data.1)?;
            }
        }
        let mut file = File::create(fp)?;        
        file.write_all(&buf)?;
        Ok(())
    }
    fn load_from_file(&mut self, fp: &str) -> std::io::Result<()> {
        let mut file = File::open(fp)?;
        let mut buffer: Vec<u8> = Vec::new();
        file.read_to_end(&mut buffer)?;
        println!("{}", buffer.len());

        let mut current_line: Vec<u8> = Vec::new();

        const NEWLINE: u8 = '\n' as u8;
        const NAMEBREAK: u8 = '#' as u8;

        for bit in buffer {
            if bit == NEWLINE {
                // Process line
                if current_line.len() > 0 {
                    if current_line[0] == NAMEBREAK {
                        current_line.remove(0);
                        if let Ok(name) = String::from_utf8(current_line.clone()) {
                            self.name = Some(name);
                        }
                        current_line.clear();
                        continue;
                    }


                    const OPENBRACE: u8 = '[' as u8;
                    const CLOSEBRACE: u8 = ']' as u8;
                    const SPACER: u8 = ',' as u8;
                    

                    let mut word: Vec<u8> = Vec::new();
                    let mut gate_type: Option<GateType> = Option::None;
                    let mut inputs: Vec<u32> = Vec::new();
                    let mut label: Option<String> = Option::None;
                    
                    while !current_line.is_empty() {
                        let char = current_line.remove(0);
                        
                        if char == ' ' as u8 {
                            continue;
                        }

                        if char == OPENBRACE {
                            if let Ok(gate) = String::from_utf8(word.clone()) {
                                if gate == String::from("Input") {gate_type = Some(GateType::Input)}
                                if gate == String::from("Output") {gate_type = Some(GateType::Output)}
                                if gate == String::from("And") {gate_type = Some(GateType::And)}
                                if gate == String::from("Or") {gate_type = Some(GateType::Or)}
                                if gate == String::from("Not") {gate_type = Some(GateType::Not)}
                                if gate == String::from("Buffer") {gate_type = Some(GateType::Buffer)}
                                if gate == String::from("Nand") {gate_type = Some(GateType::Nand)}
                                if gate == String::from("Nor") {gate_type = Some(GateType::Nor)}
                                if gate == String::from("Nxor") {gate_type = Some(GateType::Nxor)}
                                if gate == String::from("Xor") {gate_type = Some(GateType::Xor)}
                            }
                            word.clear();
                        }
                        else if char == SPACER || char == CLOSEBRACE {
                            if let Ok(val) = String::from_utf8(word.clone()) {
                                if let Ok(id) = u32::from_str_radix(&val, 10) {
                                    inputs.push(id);
                                }
                            }
                            if char == CLOSEBRACE {
                                if let Ok(_label) = String::from_utf8(current_line.clone()) {
                                    label = Some(_label);
                                }
                                break;
                            }
                            word.clear();
                        }
                        else {
                            word.push(char);
                        }
                    }
                    if let Some(gate) = gate_type {
                        self.add_component(gate, inputs, label);
                    }
                }


                current_line.clear();
            }
            else {
                current_line.push(bit);
            }
        }

        Ok(())
    }
    fn add_intergrated_circuit(&mut self, circuit: Circuit, input_ids: Vec<u32>, output_ids: Vec<u32>) {
        let mut complete_inputs = Vec::new();
        let mut complete_outputs = Vec::new();
        for (i, internal_id) in input_ids.iter().enumerate() {
            self.add_component(
                GateType::Buffer, 
                Vec::new(), 
                Some(if let Some(n) = circuit.name.clone() {format!("IC INPUT {} {}", i, n)} else {format!("IC INPUT {}", i)}));
            let last_index = self.gates.len() - 1;
            let external_id = self.gates[last_index].id;
            complete_inputs.push([(self.gates.len(), external_id), (circuit.gates.len(), *internal_id)]);
        }
        for (i, internal_id) in output_ids.iter().enumerate() {
            self.add_component(
                GateType::Buffer, 
                Vec::new(), 
                Some(if let Some(n) = circuit.name.clone() {format!("IC OUTPUT {} FOR {}", i, n)} else {format!("IC OUTPUT {}", i)}));
            let last_index = self.gates.len() - 1;
            let external_id = self.gates[last_index].id;
            complete_outputs.push([(self.gates.len(), external_id), (circuit.gates.len(), *internal_id)]);
        }
        let new_ic = IC::new(circuit, complete_inputs, complete_outputs);
        self.intergrated_circuits.push(new_ic);
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Gate {
    state: bool,
    label: Option<String>,
    gate_type: GateType,
    id: u32,
    inputs: Vec<(usize, u32)>, // Index; ID
}

impl Gate {
    fn new(gate_type: GateType, id: u32, inputs: Vec<(usize, u32)>, label: Option<String>) -> Self {
        Self { state: false, label, gate_type, id, inputs }
    }
    fn get_new_state(&self, inputs: Vec<bool>) -> bool {
        if inputs.len() == 0 && self.gate_type != GateType::Input {return false}
        match self.gate_type {
            GateType::Input => return self.state,
            GateType::Output => return inputs[0],
            GateType::And  => if inputs.contains(&false) {return false} else {return true},
            GateType::Nand => if inputs.contains(&true) {return false} else {return true},
            GateType::Nor  => if inputs.contains(&true) {return true} else {return false},
            GateType::Not  => return !inputs[0],
            GateType::Nxor => if inputs[0] != inputs[1] {return false} else {return true},
            GateType::Or   => if inputs.contains(&true) {return true} else {return false},
            GateType::Xor  => if inputs[0] != inputs[1] {return true} else {return false},
            GateType::Buffer => return inputs[0],
        }
    }
    fn update_state(&mut self, new_state: bool) {
        self.state = new_state;
    }
    fn data(&self) -> (GateType, Vec<u32>, Option<String>) {
        (self.gate_type.clone(), self.inputs.iter().map(|i| i.1).collect(), self.label.clone())
    }
}

fn input(prompt: &str) -> String {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("{}", prompt);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    s
}

impl Display for Gate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.gate_type {
            GateType::Input => {
                if let Some(n) = &self.label {
                    write!(f, "Input:{} On:{}\t{}", self.id, self.state, n)
                } else {
                    write!(f, "Input:{} On:{}", self.id, self.state)
                }
            }
            GateType::Output => {
                if self.inputs.len() > 0 {
                    if let Some(n) = &self.label {
                        write!(f, "Ouput:{} Source: {} On:{}\t{}", self.id, self.inputs[0].1, self.state, n)
                    } else {write!(f, "Ouput:{} Source: {} On:{}", self.id, self.inputs[0].1, self.state)}
                } else {write!(f, "Ouput:{} On:{}", self.id, self.state)}
            }
            _=> {
                if let Some(n) = &self.label {
                    write!(f, "{:?}{:?}:{} {}\t{}", self.gate_type, self.inputs.iter().map(|d| d.1).collect::<Vec<u32>>(), self.id, self.state, n)
                }
                else {
                    write!(f, "{:?}{:?}:{} {}", self.gate_type, self.inputs.iter().map(|d| d.1).collect::<Vec<u32>>(), self.id, self.state)
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct IC {
    circuit: Circuit,
    inputs: Vec<[(usize, u32); 2]>,    // External, Internal
    outputs: Vec<[(usize, u32); 2]>,            // External, Internal
}

impl IC {
    fn new (circuit: Circuit, inputs: Vec<[(usize, u32); 2]>, outputs: Vec<[(usize, u32); 2]>) -> Self {
        Self { circuit, inputs, outputs }
    }
}
