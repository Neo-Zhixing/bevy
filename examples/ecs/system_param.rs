use bevy::{ecs::SystemParam, prelude::*};

/// This example creates a SystemParam struct that counts the number of players
fn main() {
    App::build()
        .insert_resource(PlayerCount(0))
        .add_startup_system(spawn.system())
        .add_system(count_players.system())
        .run();
}

struct Player;
struct PlayerCount(usize);

/// The SystemParam struct can contain any types that can also be included in a
/// system function signature.
///
/// In this example, it includes a query and a mutable resource.
#[derive(SystemParam)]
pub struct PlayerCounter<'a> {
    players: Query<'a, &'a Player>,
    count: ResMut<'a, PlayerCount>,
}

impl<'a> PlayerCounter<'a> {
    fn count(&mut self) {
        self.count.0 = self.players.iter().len();
    }
}

/// Spawn some players to count
fn spawn(commands: &mut Commands) {
    commands.spawn((Player,));
    commands.spawn((Player,));
    commands.spawn((Player,));
}

/// The SystemParam can be used directly in a system argument.
fn count_players(mut counter: PlayerCounter) {
    counter.count();

    println!("{} players in the game", counter.count.0);
}
