//! Example to showcase hexgridspiral.
//!
//! See in spawn_tile_with_index the line that uses [HGSTile::new] for how to easily compute
//! the position of the hex tiles.
//!
//! See in highlight_movement_range_on_tile_click the usage of movement_range to get all reachable
//! tiles in two steps from the tile you clicked.
use bevy::color::palettes::css;
use bevy::prelude::*;
use hexgridspiral as hgs;

const LEVELMAP_TILE_CIRCUMRADIUS: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, MeshPickingPlugin))
        .add_systems(Startup, setup)
        .run();
}

/// Setup Bevy Camera.
/// Spawn a few hexagonal tiles.
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera with bluewhite background color.
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.9, 0.9, 1.0)),
            ..Default::default()
        },
    ));

    // Compute reusable shape
    let shape = meshes.add(RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6));

    // Compute step-size from tile center to tile center, given the circumradius of a tile.
    let tile_inradius = RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6).inradius();
    let step_size = 2. * tile_inradius + 3.;

    // Spawn some tiles with spiralling indices
    for tile_index in 1..36 {
        spawn_tile_with_index(
            &mut commands,
            &hgs::TileIndex::from(tile_index),
            step_size,
            &mut materials,
            shape.clone(),
        );
    }
}

#[derive(Component)]
struct TileMarker(hgs::TileIndex);

// This is a normal function, not a bevy system.
fn spawn_tile_with_index(
    commands: &mut Commands,
    tile_index: &hgs::TileIndex,
    step_size: f32,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    shape: Handle<Mesh>,
) {
    // Use hexgridspiral to compute the position of every hex tile.
    let t = hgs::HGSTile::new(*tile_index)
        .cc()
        .to_pixel((0., 0.), step_size.into());
    let position = Vec3::new(t.0 as f32, t.1 as f32, 0.);

    // Create a blue material that later gets modified to green on hover.
    let color_blue = Color::hsl(360. * 5. / 8. as f32, 0.95, 0.7);
    let color_handle = materials.add(color_blue);

    // Create a hexagonal tile with a text as child node.
    let mut tile_node = commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(color_handle.clone()),
        Transform::from_translation(position),
        TileMarker(*tile_index),
    ));
    tile_node.with_children(|parent| {
        parent.spawn((
            Text2d::new(format!("{}", tile_index)),
            TextColor(css::ALICE_BLUE.into()),
            // avoid z-fighting. The child transform is relative to the parent.
            Transform::from_xyz(0., 0., 0.0001),
        ));
    });

    // on hover, change color
    let mut color_handle1 = color_handle.clone();
    tile_node.observe(
        move |over: Trigger<Pointer<Over>>,
              mut q: Query<(&TileMarker,)>,
              mut materials: ResMut<Assets<ColorMaterial>>| {
            let (index,) = q
                .get_mut(over.entity())
                .expect("Entity that was hovered over no longer seems to exist...");
            log::debug!("Labelmap Tile {} hover", index.0);
            let color_green = Color::hsl(360. * 4. / 8. as f32, 0.95, 0.7);
            // Assumption that there is always a material associated with the TileMarker
            // Entity.
            let color_material = materials.get_mut(&mut color_handle1).unwrap();
            color_material.color = color_green;
        },
    );

    // on unhover, remove color change
    let mut color_handle2 = color_handle.clone();
    tile_node.observe(
        move |out: Trigger<Pointer<Out>>,
              mut q: Query<(&TileMarker,)>,
              mut materials: ResMut<Assets<ColorMaterial>>| {
            let (index,) = q
                .get_mut(out.entity())
                .expect("Entity that was hovered over no longer seems to exist...");
            log::debug!("Labelmap Tile {} hover", index.0);
            let color_material = materials
                .get_mut(&mut color_handle2)
                .expect("A tile without color?!");
            let color_blue = Color::hsl(360. * 5. / 8. as f32, 0.95, 0.7);
            color_material.color = color_blue;
        },
    );

    tile_node.observe(highlight_on_tile_click);
}

/// Forward the on_click trigger event, with forced order of execution.
fn highlight_on_tile_click(
    trigger: Trigger<Pointer<Click>>,
    mut q_all_tiles: Query<(&TileMarker, &mut MeshMaterial2d<ColorMaterial>)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    highlight_movement_range_on_tile_click(&trigger, &mut q_all_tiles, &mut materials);
    highlight_axes_on_tile_click(&trigger, &mut q_all_tiles, &mut materials);
}

fn highlight_movement_range_on_tile_click(
    trigger: &Trigger<Pointer<Click>>,
    q_all_tiles: &mut Query<(&TileMarker, &mut MeshMaterial2d<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Check the current tile's index
    let clicked = q_all_tiles.get(trigger.entity());
    let (clicked_tile, clicked_material) = clicked.expect("Nothing was clicked?!");
    let selected_index: hgs::TileIndex = clicked_tile.0;

    // Compute all reachable tiles, using hexgridspiral.
    let cctile = hgs::CCTile::new(selected_index);
    let movement_range: hgs::MovementRange = cctile.movement_range(2);

    // Reset all tiles' colors to plain blue
    // Except if they are in reachable range
    for (tile_marker, material_handle) in q_all_tiles.iter() {
        let tile_index = tile_marker.0;

        if movement_range.contains(&hgs::CCTile::new(tile_index)) {
            let color_reachable_yellow = Color::hsl(360. * 1. / 8. as f32, 0.95, 0.7);
            materials
                .get_mut(material_handle)
                .expect("A tile without color?!")
                .color = color_reachable_yellow;
        } else {
            let color_plain_blue = Color::hsl(360. * 5. / 8. as f32, 0.95, 0.7);
            materials
                .get_mut(material_handle)
                .expect("A tile without color?!")
                .color = color_plain_blue;
        }
    }

    // Set the clicked tile to yet another color
    let color_clicked_lime = Color::hsl(360. * 2. / 8. as f32, 0.95, 0.7);
    materials
        .get_mut(&clicked_material.0)
        .expect("A tile without color?!")
        .color = color_clicked_lime;
}

fn highlight_axes_on_tile_click(
    trigger: &Trigger<Pointer<Click>>,
    q_all_tiles: &mut Query<(&TileMarker, &mut MeshMaterial2d<ColorMaterial>)>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    // Check the current tile's index
    let clicked = q_all_tiles.get(trigger.entity());
    let (clicked_tile, _clicked_material) = clicked.expect("Nothing was clicked?!");
    let selected_index: hgs::TileIndex = clicked_tile.0;

    // Compute the CubeCoordinates
    let cctile = hgs::CCTile::new(selected_index);
    let (q, r, s) = cctile.into_qrs_tuple();

    // Reset all tiles' colors to plain blue: Happened already in
    // highlight_movement_range_on_tile_click.

    // Set all colors to a darker hue if they are reachable in a straight line from the
    // clicked tile.
    for (tile_marker, material_handle) in q_all_tiles.iter() {
        let (qq, rr, ss) = hgs::CCTile::new(tile_marker.0).into_qrs_tuple();

        if qq == q || rr == r || ss == s {
            let mat = materials
                .get_mut(material_handle)
                .expect("A tile without color?!");

            let previous_color: bevy::prelude::Laba = mat.color.into();
            let adjusted_color = bevy::prelude::Laba {
                lightness: 0.5,
                ..previous_color
            };
            mat.color = bevy::prelude::Color::Laba(adjusted_color);
        }
    }
}
