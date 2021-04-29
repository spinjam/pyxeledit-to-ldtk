use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Read, Write};
use crate::ldtk::*;
use std::fs::File;
use std::collections::HashMap;
use image::imageops::tile;
use crate::SharedData;
use serde_json::{Map, Value};

type JsonMap = HashMap<String, serde_json::Value>;

pub struct Coord {
    pub x: i64,
    pub y: i64,
}

// ------------------------------------------------------
pub fn get_pyxel_json_map(path: &Path) -> JsonMap {
    let mut json_docdata = "".to_owned();
    let mut docdata_filename = "".to_owned();

    let pyxel = fs::File::open(path).unwrap();
    let mut archive = zip::ZipArchive::new(pyxel).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let filename = file.name().clone();
        if filename.ends_with("json") {
            docdata_filename.push_str(file.name().clone());
            file.read_to_string(&mut json_docdata).expect("Read to string error");
        }
    }
    println!("JSON: {:#?}\n", docdata_filename);
    // println!("JSON data: {:#?}",json_data);

    serde_json::from_str(&json_docdata).expect("JSON not valid")
}

fn _json_create_example() {
    // define a raw string literal
    // https://rahul-thakoor.github.io/rust-raw-string-literals/
    let _data = r#"
        {
            "description": desc,
            "title": title
        }
    "#;
}

fn build_ldtk_layer_instance() -> LayerInstance {
    println!(">>> building layer instance...");
    LayerInstance {
        c_wid: 0, // map_w,
        c_hei: 0, // map_h,
        grid_size: 0, //tile_w,
        identifier: "Level1".to_owned(),
        opacity: 1.,
        px_total_offset_x: 0,
        px_total_offset_y: 0,
        tileset_def_uid: Option::None,
        tileset_rel_path: Some("".to_owned()),
        /// Layer type (possible values: IntGrid, Entities, Tiles or AutoLayer)
        layer_instance_type: "Tiles".to_owned(),
        auto_layer_tiles: vec![],
        entity_instances: vec![],
        grid_tiles: vec![],
        int_grid: Option::None,
        int_grid_csv: vec![],
        layer_def_uid: 1,
        level_id: 0,
        override_tileset_uid: Option::None,
        px_offset_x: 0,
        px_offset_y: 0,
        seed: 4592355,
        visible: true,
    }
}

fn build_ldtk_level(uid:usize) -> Level {
    println!(">>> building level...");
    Level {
        bg_color: "".to_owned(),
        bg_pos: Option::None,
        neighbours: vec![],
        level_bg_color: Option::None,
        bg_pivot_x: 0.,
        bg_pivot_y: 0.,
        level_bg_pos: Option::None,
        bg_rel_path: Option::None,
        external_rel_path: Option::None,
        field_instances: vec![],
        identifier: "Level1".to_owned(),
        layer_instances: Option::Some(vec![]),
        px_wid: 0, //canvas_width,
        px_hei: 0, //canvas_height,
        uid: uid as i64,
        world_x: 0,
        world_y: 0,
    }
}

fn build_ldtk(tileset:TilesetDefinition) -> Ldtk {
    let tile_grid_size = tileset.tile_grid_size;

    let intGridValDef = IntGridValueDefinition {
      value: 1,
        identifier: Option::None,
        color: "#000000".to_string(),
    };

    let layerDef = LayerDefinition {
        layer_definition_type: "Tiles".to_string(),
        identifier: "Tiles".to_string(),
        uid: 2,
        grid_size: tile_grid_size,
        display_opacity: 1.0,
        px_offset_x: 0,
        px_offset_y: 0,
        required_tags: vec![],
        excluded_tags: vec![],
        int_grid_values: vec![],
        auto_tileset_def_uid: Option::None,
        auto_rule_groups: vec![],
        auto_source_layer_def_uid: Option::None,
        tileset_def_uid: Some(2),
        tile_pivot_x: 0.,
        tile_pivot_y: 0.,
        purple_type: Type::Tiles,
    };

    let defs = Definitions {
        entities: vec![],
        enums: vec![],
        external_enums: vec![],
        layers: vec![layerDef],
        level_fields: vec![],
        tilesets: vec![tileset],
    };

    Ldtk {
        backup_limit: 3,
        backup_on_save: true,
        bg_color: "#000000".to_string(),
        default_grid_size: tile_grid_size,
        default_level_bg_color: "#333333".to_string(),
        default_level_height: 256,
        default_level_width: 256,
        default_pivot_x: 0.,
        default_pivot_y: 0.,
        defs,
        export_png: false,
        export_tiled: false,
        external_levels: false,
        flags: vec![],
        json_version: "0.8.1".to_owned(),
        levels: vec![],
        minify_json: false,
        next_uid: 1,
        png_file_pattern: Option::None,
        world_grid_width: 128,
        world_grid_height: 128,
        world_layout: WorldLayout::Free,
    }
}

// -----------------------------------------------------
fn pyxel_tilerefs_to_ldtk(tilerefs: &Map<String, Value>, map_w: i64, map_h:i64) -> Vec<TileInstance> {

    let mut _pyxel_tilerefs: Vec<(i64, i64)> = vec![];
    let mut _pyxel_coords: Vec<(i64, i64)> = vec![];

    let mut grid_tiles = vec![];

    // iterate Pyxel Edit tile references
    for (key, value) in tilerefs {
        let tile_ref = value.as_object().unwrap();
        let tile_pos = key.parse::<i64>().unwrap();
        let _tile_index = tile_ref["index"].as_i64().unwrap();
        //print!("pos={} index={}", tile_pos, tile_index);
        //pyxel_tilerefs.push((pyxel_tile_pos, tile_index));

        let pos_x = tile_pos % map_w;
        let pos_y = tile_pos / map_w;
        //print!("x={} y={} ",pos_x,pos_y);
    }
    // println!("\n");

    for y in 0..map_w {
        for x in 0..map_h {
            grid_tiles.push(TileInstance {
                d: vec![],
                f: 0,
                px: vec![],
                src: vec![],
                t: 0,
            });
        }
    }

    grid_tiles
    /*
    for tilerefs in pyxel_tilerefs {
        let pos: i64 = tilerefs.0;
        let pos_x: i64 = pos % map_w;
        let pos_y = pos / map_w;
        pyxel_coords.push((pos_x, pos_y));
    }
    */
}

// -----------------------------------------------------
// Conversion from Pyxel Edit (Json) to LDtk
// -----------------------------------------------------
pub fn convert(path: &Path, data: &SharedData) {
    println!(">>>>>> convert > data: ");
    let mut tileset_filename: String = data.tileset_filename.to_owned();
    tileset_filename.push_str(".png");
    println!("tileset_filename: {}", tileset_filename);

    let json = get_pyxel_json_map(path);

    let ver = json["version"].as_str().unwrap();
    let pyxel_name = json["name"].as_str().unwrap();
    println!("--- pyxel edit data '{}' (ver {}) ---", pyxel_name, ver);

    // -- get info from pyxel edit file
    let canvas = json["canvas"].as_object().unwrap();
    let canvas_width = canvas["width"].as_i64().unwrap();
    let canvas_height = canvas["height"].as_i64().unwrap();
    let tile_w = canvas["tileWidth"].as_i64().unwrap();
    let tile_h = canvas["tileHeight"].as_i64().unwrap();
    let map_w = canvas_width / tile_w;
    let map_h = canvas_height / tile_h;
    //println!("canvas w={} h={}", canvas_width, canvas_height);
    //println!("map w={} h={}", map_w, map_h);

    let num_layers = canvas.get("numLayers").unwrap();
    //println!("num layers = {}", num_layers);

    // LDtk tileset definition
    let tileset = TilesetDefinition {
        identifier: "sokoban-6".to_owned(),
        uid: 2,
        //rel_path: "sokoban-6.png".to_owned(),
        rel_path: String::from(tileset_filename.to_owned()),
        px_wid: data.tileset_w,
        px_hei: data.tileset_h,
        tile_grid_size: data.tile_w,
        spacing: 0,
        padding: 0,
        saved_selections: vec![],
        cached_pixel_data: Option::None,
    };

    let mut ldtk:Ldtk = build_ldtk(tileset);

    let layers = canvas["layers"].as_object().unwrap();
    for (li,layer) in layers.iter().enumerate() {
        let l = layer.1.as_object().unwrap();
        let layer_type = l["type"].as_str().unwrap();
        let layer_name = l["name"].as_str().unwrap();
        //println!("#{}: name='{}' type={}", layer.0, layer_name, layer_type);

        let tile_refs = l["tileRefs"].as_object().unwrap();
        //println!("num tile refs {}", tile_refs.len());
        let grid_tiles = pyxel_tilerefs_to_ldtk(tile_refs, map_w, map_h);

        let mut layer_instance = build_ldtk_layer_instance();
        layer_instance.c_wid = map_w;
        layer_instance.c_hei = map_h;
        layer_instance.grid_size = tile_w;
        layer_instance.grid_tiles = grid_tiles;
        layer_instance.tileset_rel_path = Some(tileset_filename.to_owned());
        layer_instance.tileset_def_uid = Some(li as i64);

        let mut level = build_ldtk_level(li);

        level.layer_instances = Some(vec![layer_instance]);
        level.identifier = "Level".to_owned();
        level.identifier.push_str(format!("{}",li).as_str());
        level.px_wid = canvas_width;
        level.px_hei = canvas_height;
        println!("--- level identifier={}",level.identifier);

        ldtk.levels.push(level);
    } // -end-layer-

    let json_save = serde_json::to_string_pretty(&ldtk).unwrap();
    // println!("{}", json_save);

    // [] WRITE LDTK (json) file
    let mut ldtk_path = PathBuf::new();
    ldtk_path.push("target");
    ldtk_path.push(data.tileset_filename.to_owned());
    ldtk_path.set_extension("ldtk");
    let display = ldtk_path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&ldtk_path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(json_save.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}