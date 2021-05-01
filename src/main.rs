mod ldtk;
mod convert;

use std::fs;
use std::io;
use std::env;
use std::path::{Path, PathBuf};
use image::{RgbaImage, GenericImage, open};
use convert::convert;
use crate::convert::get_pyxel_json_map;

pub struct SharedData {
    tileset_filename: String,
    tileset_w: i64,
    tileset_h: i64,
    tile_w: i64,
    tile_h: i64,
}
// ---------------------------------------
fn main() {
    std::process::exit(real_main());
}

fn build_tileset_image(path: &Path) -> SharedData {
    let temp_dir = env::temp_dir();
    println!("Temporary directory: {}", temp_dir.display());

    let mut tiles: Vec<&str> = Vec::new(); // tile filenames (no folder)

    let source_filename = path.to_str().unwrap();
    let source_file = path.file_name().unwrap().to_os_string().into_string().unwrap();
    let source_name = path.file_stem().unwrap().to_os_string().into_string().unwrap();

    println!("source file = {}",source_file);
    println!("source name (no ext) = {}", source_name); // no extension
    //let parent = path.parent().unwrap();
    //println!("parent = {:?}", parent.as_os_str());
    print!(">>> Reading file {:?}\n", path.as_os_str());

    // [1] Open pyxel archive file (pyxel extension - a zip file)
    let pyxel = fs::File::open(&path).unwrap();
    let archive = zip::ZipArchive::new(pyxel).unwrap();

    // [3] Store tiles' names
    let all_files: Vec<&str> = archive.file_names().collect();
    for current in all_files {
        if current.starts_with("tile") {
            tiles.push(current.clone());
        }
    }

    // [2] Sort tiles by number
    tiles.sort_by(|a, b| {
        // filename is "tileXXX.png", so compare only the numeric part
        let a1 = a.split('.').next().unwrap_or("");
        let b1 = b.split('.').next().unwrap_or("");
        let a2: i32 = a1[4..].parse().unwrap();
        let b2: i32 = b1[4..].parse().unwrap();
        a2.cmp(&b2)
    });

    // [3] UNZIP files to a temp folder
    let mut path_dest = PathBuf::new();
    path_dest.push(temp_dir.clone());
    path_dest.push("tmp-pyxel");
    fs::create_dir_all(path_dest.clone()).unwrap();

    let file = fs::File::open(path).unwrap();
    let mut archive2 = zip::ZipArchive::new(file).unwrap();
    for ti in 0..tiles.len() {
        let tile = tiles[ti];
        let mut file = archive2.by_name(tile).unwrap();

        let mut outp = path_dest.clone();
        match file.enclosed_name() {
            Some(path) => outp.push(path.to_owned()),
            None => continue,
        };
        {
            let mut outfile = fs::File::create(&outp).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }
    }

    // get tile width and height from Pyxel Edit (json)
    let json = get_pyxel_json_map(path);
    let tileset = json["tileset"].as_object().unwrap();
    let tiles_per_row = tileset["tilesWide"].as_u64().unwrap();
    let canvas = json["canvas"].as_object().unwrap();
    let tile_w = canvas["tileWidth"].as_u64().unwrap();
    let tile_h = canvas["tileHeight"].as_u64().unwrap();
    print!("pyxel tile width={} height={}\n",tile_w,tile_h);

    // [4] CREATE DEST image
    let num_tiles: u32 = tiles.len() as u32;
    let dest_x = tiles_per_row * tile_w;
    let dest_y= ((num_tiles / tiles_per_row as u32) + 1) * tile_w as u32;
    println!("dest image w={} h={}", dest_x, dest_y);
    let mut dest_img: RgbaImage = RgbaImage::new(dest_x as u32, dest_y as u32);

    // [5] READING files from TEMP folder
    let mut x = 0;
    let mut y= 0;
    for ti in 0..tiles.len() {
        let mut path = PathBuf::new();
        path.push(path_dest.clone());
        path.push(tiles[ti]);
        let curr_tile = open(&path).unwrap().into_rgba8();
        // println!("dimensions x={} y={}", curr_tile.width(), curr_tile.height());
        dest_img.copy_from(&curr_tile, x, y).expect("copy_from error");
        x += tile_w as u32;
        if x >= dest_x as u32 {
            y += tile_w as u32;
            x = 0;
        }
    }

    // [6] DESTINATION PATH
    let mut dest_path = PathBuf::new();
    dest_path.push("target");
    dest_path.push(source_name.to_owned());
    dest_path.set_extension("png");
    println!(">>> SAVING tileset image={}", dest_path.to_string_lossy());

    // [7] SAVE image
    dest_img.save(dest_path).unwrap();

    // [8] remove temp dir
    fs::remove_dir_all(path_dest).expect("Remove dir with error");

    //(dest_x, dest_y)
    SharedData {
        tileset_filename: source_name.clone(),
        tileset_w: dest_x as i64,
        tileset_h: dest_y as i64,
        tile_w: tile_w as i64,
        tile_h: tile_h as i64,
    }
}

// -----------------------------------------
fn real_main() -> i32 {
    let args: Vec<String> = env::args().collect();
    println!("args = {:?}",args);
    if args.len() < 2 {
        println!("No arguments");
        return 1;
    }
    let source_pyxeledit = &args[1];

    let source_path = Path::new(source_pyxeledit);
    let data = build_tileset_image(&source_path);
    convert(&source_path, &data);

    return 0;
}