# SIMLO (A COMMAND BASED LOGIC SIM)

## Overview

I want to create a logic gate simulator because I find they're really helpful with learning about how computers work. Logisim exists but I wanted to try out the idea of one that you can use with just a keyboard as sometimes I am lazy and just resting my hands on a keyboard feels much easier than using a mouse and moving my whole arm about to do things. The way simlo will work is by enterring a command to create a new gate/node and the program will add it to the circuit.

## Stakeholders

The target group of this project is weird people like me who enjoy creating logic gate circuits for fun. I will likely ask for feedback on the project in an online forum for coding as there contains many like minded people whom I know and who trust me and my code.

## Analysis

There are only two apps that I have seen simulate logic circuits well and that is logisim and a game called scrap mechanic. Logisim has you place each logic gate and connect them with wires on a 2D surface. Scrap mechanic has you place logic gate blocks and connect them with wires in a 3D world. Both allow you to export a small section of a circuit and copy it multiple times if needed. The problems with these is that you have to manually connect the wires and place the logic gates. This is bad as if careful consideration does not go into the placement of the gates then circuits can quickly get overwhelmingly messy and hard to find out what connects to what (especially in scrap mechanic). Logisim has other built in components that are used in real life where scrap mechanic has just buttons and switches and a timer component which is alike to a clock but not quite the same. Both of the apps implement the logic gates: And, Or, Xor; and their inverted counter parts: Nand, Nor, Nxor. Scrap mechanic does not inplement a Not gate as a Nor gate can be used in it's place by only giving it a single input. Logic gates light up when active in both pieces of software.

Unlike real life, scrap mechanic has logic gates update once per game tick which can cause circuits to form into an unrealistic flickering loop. Simlo will have this "issue" but it only applies to certain circumstances where gates are connected in a loop.

Logisim also has a bussing feature that allows multiple bits to travel on the same line which is very useful when you have a collection of visible wires that need to get from one side of the "map" to another but if you don't have physical wires then it isn't an issue! To put it simply, we've had 2D and even 3D logic simulators so maybe it's time for a one-dimensional logic gate simulator. That's what simlo is. Of course a feature to display the circuits in 2D would be useful sometimes but that would be auto-generated in some way.

## Design


## The code

Firstly thing needed is logic gate identifiers.

```rs
#[derive(Clone, Copy, Debug, PartialEq)]
enum GateType {
    Input,
    Output,
    Not,
    And,
    Or,
    Nand,
    Nor,
    Xor,
    Nxor,
}
```

Using an enum because it just makes life that much easier. 

Then a logic gate object is needed.

```rs
#[derive(Clone, Debug)]
struct Gate {
    state: bool,
    label: Option<String>,
    gate_type: GateType,
    id: u32,
    inputs: Vec<(usize, u32)>, // (Index, ID)
}
```

Vectors don't implement the Copy trait so it cannot be derived on the struct. `state` is the output of the current gate. `label` is an optional note that can be added to a logic gate to make it easier to identify its use. `gate_type` just identifies the logic gate used. `id` is the identifier of the object when it is in the collected array and it is like this because there are no physical wires, it is also only 32 bits as I doubt someone would add 4294967295 logic gates in one circuit as that would be like trying to write windows without reusable code. Then there's `inputs` which has a comment next to it saying "`Index, ID`". The `inputs` `ID` is there to store the identifier of the input and `Index` is there to make things faster when looking for the input. This is because once an input is set the only direction its index in the circuit vector can go is down so we store the index of that specific node and check if the id is in that location and if it is not in that location then we decrease the index until it is. It is highly important, however, that items are **only added to the top** of the circuit vector, though Items can be removed from any location.

Then an object for the circuits:

```rs
struct Circuit {
    id: u32,
    name: Option<String>,
    counter: u32,
    gates: Vec<Gate>,
}
```

`id` references the circuit identifier so that exported circuits can be imported into another through a circuit id. `name` is an optional feature to add information to the circuit in the catalogue. `counter` is kind of unnecessary but it was there to give a unique id to new gates. `gates` stores the logic gates in the object. Now to remove the `counter` property.

```rs
struct Circuit {
    id: u32,
    name: Option<String>,
    // counter: u32,
    gates: Vec<Gate>,
}
```

The previous version of that was actually correct because if a gate is deleted then the ID count goes down which is not good as it may then overlap with another id and cause problems but I am renaming it to be more clear that it's counting the id.

```rs
struct Circuit {
    id: u32,
    name: Option<String>,
    id_counter: u32,
    gates: Vec<Gate>,
}
```

The structure of the program revolves around a recursive main function which collects up the circuits in the end. In this function there is an option to create a new circuit which just calls the function and begins recursion. Compiling a circuit pushes a clone of it onto a "catalogue" or vector containing circuits which is shared throughout the recursion.

