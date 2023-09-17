mod stage_one;
mod stage_two;
mod stage_zero;

use bevy::prelude::*;
use clap::Parser;
use stage_one::StageOnePlugin;
use stage_two::StageTwoPlugin;
use stage_zero::StageZeroPlugin;

#[derive(Parser, Debug)]
struct Args {
    #[arg(long)]
    input: String,

    #[arg(long, short)]
    stage: usize,
}

#[derive(Resource)]
struct GameInput(String);

fn main() {
    let args = Args::parse();

    let mut app = App::new();

    (match args.stage {
        0 => app.add_plugins(StageZeroPlugin),
        1 => app.add_plugins(StageOnePlugin),
        2 => app.add_plugins(StageTwoPlugin),
        _ => panic!("Unknown stage: {}", args.stage),
    })
    .insert_resource(GameInput(args.input))
    .add_plugins(DefaultPlugins)
    .run();
}
