use bevy::color::palettes::css;
use bevy::prelude::*;
use hexgridspiral as hgs;

const LEVELMAP_TILE_CIRCUMRADIUS: f32 = 50.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

/// Setup Bevy Camera
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2d,
        Camera {
            // clear the whole viewport with the given color
            clear_color: ClearColorConfig::Custom(Color::srgb(0.9, 0.9, 1.0)),
            ..Default::default()
        },
    ));

    let shape = meshes.add(RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6));
    let color_blue = Color::hsl(360. * 5. / 8. as f32, 0.95, 0.7);
    let color_handle = materials.add(color_blue);

    // Compute step-size from tile center to tile center, given the circumradius of a tile.
    let tile_inradius = RegularPolygon::new(LEVELMAP_TILE_CIRCUMRADIUS, 6).inradius();
    let step_size = 2. * tile_inradius + 3.;

    // Spawn some tiles with spiralling indices
    for tile_index in 1..36 {
        spawn_tile_with_index(
            &mut commands,
            &hgs::TileIndex::from(tile_index),
            step_size,
            color_handle.clone(),
            shape.clone(),
        );
    }
}

#[derive(Component)]
struct LevelmapTileMarker(hgs::TileIndex);

// This is a normal function, not a bevy system.
fn spawn_tile_with_index(
    commands: &mut Commands,
    tile_index: &hgs::TileIndex,
    step_size: f32,
    material_handle: Handle<ColorMaterial>,
    shape: Handle<Mesh>,
) {
    let t = hgs::HGSTile::new(*tile_index)
        .cc()
        .to_pixel((0., 0.), step_size.into());
    let position = Vec3::new(t.0 as f32, t.1 as f32, 0.);

    let mut tile_node = commands.spawn((
        Mesh2d(shape),
        MeshMaterial2d(material_handle),
        Transform::from_translation(position),
        LevelmapTileMarker(*tile_index),
    ));
    tile_node.with_children(|parent| {
        parent.spawn((
            Text2d::new(format!("{}", tile_index)),
            TextColor(css::ALICE_BLUE.into()),
            // avoid z-fighting. The child transform is relative to the parent.
            Transform::from_xyz(0., 0., 0.0001),
        ));
    });

    /*
    // on hover, add outline
    tile_node.observe(
        |over: Trigger<Pointer<Over>>, mut q: Query<(&mut bpl::Stroke, &LevelmapTileMarker)>| {
            let (mut stroke, ref index) = q
                .get_mut(over.entity())
                .expect("Entity that was hovered over no longer seems to exist...");
            log::debug!("Labelmap Tile {} hover", index.0);
            stroke.color = CC::GREENBLUEISH;
        },
    );

    // on unhover, remove outline
    tile_node.observe(
        |out: Trigger<Pointer<Out>>, mut q: Query<(&mut bpl::Stroke, &LevelmapTileMarker)>| {
            let res = q
                .get_mut(out.entity())
                .expect("Entity that was hovered over no longer seems to exist...");
            let mut stroke = res.0;
            log::debug!("Labelmap Tile {} unhover", res.1 .0);
            stroke.color = CC::COOLORS_BIOLET;
        },
    );

    tile_node.observe(observer_on_tile_click);
    */
}
