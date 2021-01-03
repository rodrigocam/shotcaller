use crate::*;

pub fn 
    spawn_creep_system(
    game_events: &Vec<GameEvent>,
     stat_def: &StatDefinitions<Stats>,
     entities: &mut Entities,
     positions: &mut Components<Point>,
     creeps: &mut Components<Creep>,
     simple_movements: &mut Components<SimpleMovement>,
     ai_destinations: &mut Components<AiDestination>,
     proximity_attacks: &mut Components<ProximityAttack>,
     stats: &mut Components<StatSet<Stats>>,
     ai_paths: &mut Components<AiPath>,
     teams: &mut Components<Team>,
     sprites: &mut Components<Sprite>,
     sprite_indices: &mut Components<SpriteIndex>,
     ) -> SystemResult{
        for ev in game_events.iter() {
            if let GameEvent::SpawnCreep(pos, team) = ev {
                let creep = entities.create();
                positions.insert(creep, pos.clone());
                creeps.insert(creep, Creep);
                simple_movements.insert(creep, SimpleMovement);
                ai_paths
                    .insert(creep, AiPath::new(NavigationPath::new()))
                    ;
                teams.insert(creep, *team);
                stats.insert(creep, stat_def.to_statset());
                proximity_attacks
                    .insert(creep, ProximityAttack::new(CREEP_ATTACK_RADIUS))
                    ;
                let bg = if *team == Team::Me {
                    RGBA::named(GREEN)
                } else {
                    RGBA::named(RED)
                };
                sprites
                    .insert(
                        creep,
                        Sprite {
                            glyph: to_cp437('c'),
                            fg: RGBA::named(YELLOW),
                            bg,
                        },
                    )
                    ;
                sprite_indices.insert(creep, SpriteIndex(9));
        }
    }
        Ok(())
     }
