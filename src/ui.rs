use crate::currency::Currency;
use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, make_ui);
    }
}

#[derive(Component)]
pub struct CurrencyCounter;

fn make_ui(mut commands: Commands, currency: Res<Currency>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                TextBundle::from_section(format!("Wood: {}", currency.wood), TextStyle::default()),
                Label,
                CurrencyCounter,
            ));
        });
}
