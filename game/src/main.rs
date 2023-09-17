mod stage_one;
mod stage_two;

use crate::{stage_one::StageOnePlugin, stage_two::StageTwoPlugin};
use bevy::prelude::*;
use clap::Parser;

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
        1 => app.add_plugins(StageOnePlugin),
        2 => app.add_plugins(StageTwoPlugin),
        _ => panic!("Unknown stage: {}", args.stage),
    })
    .insert_resource(GameInput(args.input))
    .add_plugins(DefaultPlugins)
    .run();
}
