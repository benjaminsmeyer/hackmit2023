mod stage_one;

use crate::stage_one::StageOnePlugin;
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

    let plugin = match args.stage {
        1 => StageOnePlugin,
        _ => panic!("Unknown stage: {}", args.stage),
    };

    App::new()
        .insert_resource(GameInput(args.input))
        .add_plugins(DefaultPlugins)
        .add_plugins(plugin)
        .run();
}
