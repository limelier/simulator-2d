use simulator_2d::sim::{ Simulation, Instruction, TurnDirection };

fn main() {
    let instructions = vec![
        Instruction::Place,
        Instruction::Move,
        Instruction::Place,
        Instruction::Move,
        Instruction::Turn(TurnDirection::Right),
        Instruction::Move,
        Instruction::Place,
        Instruction::Move,
        Instruction::Place
    ];

    let mut simulation = Simulation::new(2, &instructions);
    simulation.run();

    println!("{:?}", simulation.board);
}









