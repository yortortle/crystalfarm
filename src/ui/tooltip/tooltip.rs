use bevy::picking::hover::Hovered;
use bevy::prelude::*;

#[derive(Component)]
pub struct HoverTooltip(pub &'static str);

#[derive(Resource)]
pub struct TooltipUi {
    pub panel: Entity,
    pub text: Entity,
}

pub struct TooltipPlugin;

impl Plugin for TooltipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_tooltip_ui)
           .add_systems(Update, tooltip_on_hover);
    }
}

pub fn spawn_tooltip_ui(mut commands: Commands) {
    let mut text_entity = Entity::PLACEHOLDER;

    let panel = commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(16.0),
                top: Val::Px(16.0),
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.35)),
            Visibility::Hidden,
        ))
        .with_children(|parent| {
            text_entity = parent
                .spawn((
                    Text::new(""),
                    TextFont {
                        font_size: 16.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ))
                .id();
        })
        .id();

    commands.insert_resource(TooltipUi {
        panel,
        text: text_entity,
    });
}

pub fn tooltip_on_hover(
    ui: Res<TooltipUi>,
    windows: Query<&Window>,
    mut panel_vis: Query<&mut Visibility>,
    mut text_q: Query<&mut Text>,
    mut node_q: Query<&mut Node>,
    hovered_q: Query<(&Hovered, &HoverTooltip), Changed<Hovered>>,
) {
    let Ok(window) = windows.single() else { return; };
    let cursor = window.cursor_position();

    for (hovered, tooltip) in &hovered_q {
        let show = hovered.get();

        if let Ok(mut v) = panel_vis.get_mut(ui.panel) {
            *v = if show { Visibility::Visible } else { Visibility::Hidden };
        }

        if let Ok(mut text) = text_q.get_mut(ui.text) {
            *text = Text::new(if show { tooltip.0 } else { "" });
        }

        if show {
            if let Some(cursor_pos) = cursor {
                if let Ok(mut node) = node_q.get_mut(ui.panel) {
                    node.left = Val::Px(cursor_pos.x + 12.0);
                    node.top = Val::Px(cursor_pos.y + 12.0);
                }
            }
        }
    }
}