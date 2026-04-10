use macroquad::prelude::*;
use std::{time};
use std::thread::sleep;
use macroquad::ui::{hash, root_ui, widgets::InputText};
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use imageinfo::{ImageInfo};
const EDITOR_TOP_MARGIN: f32 = 10.0;
const EDITOR_HEIGHT: f32 = 40.0;

#[macroquad::main("M.I.C.A.")]

async fn main() {

    clear_background(WHITE);
    draw_text("LOADING", 10.0, 26.0, 32.0, BLACK);
    next_frame().await; //RENDER IT!

    //LOAD THINE ASSETS
    let logo: Texture2D = load_texture("mica_assets/menu/logo.png").await.unwrap();
    let paint: Texture2D = load_texture("mica_assets/image/paint.png").await.unwrap();
    let imageeditor: Texture2D = load_texture("mica_assets/menu/image.png").await.unwrap();
    let _liscense: Texture2D = load_texture("mica_assets/oneshot.png").await.unwrap();
    let rubber: Texture2D = load_texture("mica_assets/image/erase.png").await.unwrap();
    let colsel: Texture2D = load_texture("mica_assets/image/colour.png").await.unwrap();
    let trans: Texture2D = load_texture("mica_assets/image/trans.png").await.unwrap();
    let down: Texture2D = load_texture("mica_assets/image/down.png").await.unwrap();
    let plus: Texture2D = load_texture("mica_assets/image/plus.png").await.unwrap();
    let save: Texture2D = load_texture("mica_assets/image/save.png").await.unwrap();
    let mif: Texture2D = load_texture("mica_assets/image/MIF.png").await.unwrap();
    let _linedraw: Texture2D = load_texture("mica_assets/image/line.png").await.unwrap();
    let mif_load: Texture2D = load_texture("mica_assets/image/mif_load.png").await.unwrap();
    let import: Texture2D = load_texture("mica_assets/image/IMPORT.png").await.unwrap();

    //input variables
    let mut mousex: f32;
    let mut mousey: f32;
    //previous positions
    let mut mousx: f32;
    let mut mousy: f32;
    let mut liscense; //are you looking at that GNU gpl v3 again?
    let mut colmenu; //colour menu
    let mut credit; //credit menu
    //line drawing stuff
    let mut sdifx: f32;
    let mut sdify: f32;
    let mut stepy: f32;
    let mut stepx: f32;
    let mut step: i32;
    let mut x: f32;
    let mut y: f32;
    let mut size: i32  = 42; //life the universe and everything
    let (mut mouse_wheel_x, mut mouse_wheel_y);
    let _i: i128;
    let _i2: i64;
    //colour stuff
    let mut colour = Color::new(0.50, 0.5, 0.5, 1.00);
    let mut flipcol;
    let erase = Color::new(0.00, 0.0, 0.0, 0.00);
    let mut state = 0;
    let mut cr = 128;
    let mut cg = 128;
    let mut cb = 128;
    let mut ca = 255;
    let mut r: u8;
    let mut g: u8;
    let mut b: u8;
    let mut a: u8;


    let mut xsel;
    let mut ysel = 0;
    let mut datax = String::new();
    let mut datay = String::new();
    //saving stuff
    let mut fx: i32;
    let mut fy: i32;
    let mut go =1;
    let mut savepath = String::new();

    //mif stuff
    let _path = "save.mif";
    let mut loadpoint;
    let mut miftype;
    let mut mifdatay;
    let mut mifdatax;
    let _file_path = Path::new("save.mif");
    let mut currr: char;
    let mut currg: char;
    let mut currb: char;
    let mut curra: char;


    //st stuff
    let _path = "save.st";
    let mut decomp = String::new();
    let mut stdatax;





    //loop (woah no kidding)

    loop {


        (mousex, mousey) = mouse_position();//get mouse position
        //clears and draws menu
        clear_background(WHITE);
        draw_texture(&logo, screen_width()/2. - 250., 0., WHITE);
        draw_texture(&imageeditor, screen_width()/2. - 150., screen_height()/2., WHITE);

        //liscense button
        draw_rectangle(10., screen_height() - 19., 435., 13., GRAY);
        draw_rectangle(10., screen_height() - 19., 434., 12., LIGHTGRAY);
        draw_text("By using this software you agree to the terms of this liscense", 10.0, screen_height() - 10.0, 16.0, BLACK);

        //credits button
        draw_rectangle(10., screen_height() - 35., 50., 13., GRAY);
        draw_rectangle(10., screen_height() - 35., 50., 12., LIGHTGRAY);
        draw_text("Credits", 10.0, screen_height() - 26.0, 16.0, BLACK);
        //credits hitbox and function
        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 50.) && (mousey >= screen_height() - 35. && mousey <= screen_height() - 26.){
            credit = 1;
            println!("credits pressed!");
            while credit == 1 {
                (mousex, mousey) = mouse_position();//get mouse position

                clear_background(WHITE); //clear
                //logo & text
                draw_texture(&logo, 0., -30., WHITE);
                draw_text("M.I.C.A Milorad Image Creation Application.", 10.0, 140.0, 20.0, BLACK);
                draw_multiline_text("M.I.C.A. in rust \nCode by:\n Squirre and Mepm\nUi and icons:\n Squirrel\n\nM.I.C.A. in gamemaker (now deprecated):\nGamemaker code by:\n  Mepm\nGamemaker icons:\n Mepm and Squirrel", 20.0, 200.0, 20.0, Some(1.0), BLACK);
                //back button
                draw_rectangle(10., 530., 31., 11., GRAY);
                draw_rectangle(10., 530., 30., 10., LIGHTGRAY);
                draw_text("BACK", 10.0, 539.0, 16.0, BLACK);

                next_frame().await; //RENDER IT!
                //check if you pressed back
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 10. && mousex <= 80.) && (mousey >= 530. && mousey <= 550.){credit = 0;}
            }
        }

        //checks if you pressed image editor
        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= (screen_width()/2. - 150.) && mousex <= (screen_width()/2. + 150.)) && (mousey >= (screen_height()/2.) && mousey <= (screen_height()/2. + 100.)){

            xsel = 1;
            while xsel == 1 {
                (mousex, mousey) = mouse_position();//get mouse position
                clear_background(WHITE);
                draw_text("Input X size (draw area width) leave blank for window sized canvas", 10.0, 60.0, 16.0, BLACK);
                draw_rectangle(105., 19., 21., 13., GRAY);
                draw_rectangle(105., 19., 20., 12., LIGHTGRAY);
                draw_text("OK", 105.0, 30.0, 16.0, BLACK);
                //text input
                let window_id = hash!();
                root_ui().window(
                    window_id,
                    vec2(0.0, EDITOR_TOP_MARGIN),
                                 vec2(100.0, EDITOR_HEIGHT),
                                 |ui| {
                                     let input_text_id = hash!();
                                     InputText::new(input_text_id)
                                     .label("")
                                     .size(vec2(96.0, EDITOR_HEIGHT - 4.0))
                                     .ui(ui, &mut datax);

                                 },

                );

                if is_key_pressed(KeyCode::Enter){
                    println!("ok you pressed it!");
                    xsel = 0;
                    ysel = 1;
                    if datax == "" {
                        ysel = 0;
                    }
                }
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 105. && mousex <= 126.) && (mousey >= 19. && mousey <= 32.){
                    println!("ok you pressed it!");
                    xsel = 0;
                    ysel = 1;
                    if datax == "" {
                        ysel = 0;
                    }
                }
                next_frame().await; //RENDER IT!
            }
            while ysel == 1 {
                (mousex, mousey) = mouse_position();//get mouse position
                clear_background(WHITE);
                draw_text("Input Y size (draw area Height)", 10.0, 60.0, 16.0, BLACK);
                draw_rectangle(105., 19., 21., 13., GRAY);
                draw_rectangle(105., 19., 20., 12., LIGHTGRAY);
                draw_text("OK", 105.0, 30.0, 16.0, BLACK);
                //text input
                let window_id = hash!();
                root_ui().window(
                    window_id,
                    vec2(0.0, EDITOR_TOP_MARGIN),
                                 vec2(100.0, EDITOR_HEIGHT),
                                 |ui| {
                                     let input_text_id = hash!();
                                     InputText::new(input_text_id)
                                     .label("")
                                     .size(vec2(96.0, EDITOR_HEIGHT - 4.0))
                                     .ui(ui, &mut datay);

                                 },

                );

                if is_key_pressed(KeyCode::Enter){
                    println!("ok you pressed it!");
                    ysel = 0;
                }
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 105. && mousex <= 126.) && (mousey >= 19. && mousey <= 32.){
                    println!("ok you pressed it!");
                    ysel = 0;
                }
                next_frame().await; //RENDER IT!
            }


            println!("poraro");
            //init texture|
            let _texture = Texture2D::empty();
            //image shit for drawing
            let mut w: usize;
            let mut h: usize;
            //mif loader


            if datax == "" {
                h = screen_height() as usize;
                w = (screen_width() - 32.) as usize;
            }else{

                let wint: f32 = datax.parse().unwrap();
                let hint: f32 = datay.parse().unwrap();
                w = wint as usize;
                h = hint as usize;
            }

            let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
            let mut image2 = Texture2D::from_image(&image);
            let mut flip = Image::gen_image_color(w as u16, h as u16, WHITE);
            let _flip2 = Texture2D::from_image(&image);
            loop{
                mousx = mousex;
                mousy = mousey;
                (mousex, mousey) = mouse_position(); //get mouse pos

                //draw menu
                clear_background(WHITE);
                draw_texture(&trans, 0., 0., WHITE);

                (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();

                if mouse_wheel_y == -1. {size += -1;}
                if mouse_wheel_y == 1. {size += 1;}

                //draw code

                if is_mouse_button_down(MouseButton::Left) && state == 1 {
                    println!("down and in range!");
                    //line drawing code (sponsored by milorad)

                    step = ((mousex - mousx).abs()) as i32;
                    if step >= ((mousey - mousy).abs()) as i32 {
                    }else{
                        step = ((mousey - mousy).abs()) as i32;
                    }
                    if step == 0{
                        step = 1;
                    }
                    //centres the draw square
                    x = mousx - ((size / 2) + 32) as f32;
                    y = mousy - (size / 2) as f32;
                    sdifx = mousex - mousx;
                    sdify = mousey - mousy;
                    if sdifx ==0.{
                        sdifx = 1.;
                    }
                    if sdify ==0.{
                        sdify = 1.;
                    }

                    stepx = sdifx /(step) as f32;
                    stepy = sdify /(step) as f32;
                    if stepx == 0.{
                        stepx = 1.;
                    }
                    if stepx == 0.{
                        stepx = 1.;
                    }
                    for _ in 0..step{

                        x += stepx;
                        y += stepy;
                        for i2 in 0..size{
                            for i in 0..size{
                                if i + (x) as i32 >= w.try_into().unwrap() {
                                }else{
                                    if i2 + (y) as i32 >= h.try_into().unwrap() {
                                    }else{

                                        image.set_pixel((x + (i) as f32) as u32, (y + (i2) as f32) as u32, colour,);}}
                            }
                        }
                    }

                }

                //erase code
                if is_mouse_button_down(MouseButton::Left) && state == 2 {
                    println!("down and in range!");
                    //line drawing code (sponsored by milorad)

                    step = ((mousex - mousx).abs()) as i32;
                    if step >= ((mousey - mousy).abs()) as i32 {
                    }else{
                        step = ((mousey - mousy).abs()) as i32;
                    }
                    //centres the draw square
                    x = mousx - ((size / 2) + 32) as f32;
                    y = mousy - (size / 2) as f32;
                    sdifx = mousex - mousx;
                    sdify = mousey - mousy;
                    stepx = sdifx /(step) as f32;
                    stepy = sdify /(step) as f32;
                    for _ in 0..step{

                        x += stepx;
                        y += stepy;
                        for i2 in 0..size{
                            for i in 0..size{
                                if i + (x) as i32 >= w.try_into().unwrap() {
                                }else{
                                    if i2 + (y) as i32 >= h.try_into().unwrap() {
                                    }else{

                                        image.set_pixel((x + (i) as f32) as u32, (y + (i2) as f32) as u32, erase,);}}
                            }
                        }
                    }

                }






                //update image buffer
                image2.update(&image);
                draw_texture(&image2, 32., 0., WHITE);
                //draw the menu
                draw_rectangle(0., 0., 32., 1900., WHITE);
                draw_rectangle(32., 0., 2., 1900., GRAY);

                draw_text("Size:", 0.0, screen_height() - 26., 16.0, BLACK);
                draw_text(&size.to_string(), 10.0, screen_height() - 10., 16.0, BLACK);
                draw_texture(&paint, 0., 0., WHITE);
                draw_texture(&rubber, 0., 32., WHITE);
                draw_texture(&colsel, 0., 64., WHITE);
                draw_texture(&save, 0., 96., WHITE);
                draw_texture(&mif, 0., 128., WHITE);
                draw_texture(&mif_load, 0., 160., WHITE);
                draw_texture(&import, 0., 192., WHITE);

                //MIF loading
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 160. && mousey <= 182.){
                    let mut loadfile = String::new();
                    xsel = 1;
                    //mif stuff
                    mifdatay = String::new();
                    mifdatax = String::new();

                    while xsel == 1 {
                        (mousex, mousey) = mouse_position();//get mouse position
                        clear_background(WHITE);
                        draw_text("Input file path eg. /home/user/Pictures/loadfile.mif", 10.0, 60.0, 16.0, BLACK);
                        draw_rectangle(550., 19., 21., 13., GRAY);
                        draw_rectangle(550., 19., 20., 12., LIGHTGRAY);
                        draw_text("OK", 550.0, 30.0, 16.0, BLACK);
                        //text input
                        let window_id = hash!();
                        root_ui().window(
                            window_id,
                            vec2(0.0, EDITOR_TOP_MARGIN),
                                         vec2(500.0, EDITOR_HEIGHT),
                                         |ui| {
                                             let input_text_id = hash!();
                                             InputText::new(input_text_id)
                                             .label("")
                                             .size(vec2(496.0, EDITOR_HEIGHT - 4.0))
                                             .ui(ui, &mut loadfile);

                                         },

                        );

                        if is_key_pressed(KeyCode::Enter){
                            println!("ok you pressed it!");
                            xsel = 0;
                            draw_text("Loading", 10.0, 90.0, 16.0, BLACK);
                            if loadfile == "" {
                                draw_text("No Path", 80.0, 60.0, 64.0, BLACK);
                            }
                        }
                        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 550. && mousex <= 571.) && (mousey >= 19. && mousey <= 32.){
                            println!("ok you pressed it!");
                            println!("ok you pressed it!");
                            xsel = 0;
                            draw_text("Loading", 10.0, 90.0, 16.0, BLACK);
                            if loadfile == "" {
                                draw_text("No Path", 60.0, 60.0, 64.0, BLACK);
                            }
                        }
                        next_frame().await; //RENDER IT!
                    }
                    let contents = fs::read_to_string(Path::new(&loadfile)).expect("Should have been able to read the file");
                    // gets MIF type to identify if this in fact a MIF-rs and not any other file
                    miftype = String::from("");
                    for loadpoint in 0..8 {
                        miftype = miftype + &(&contents[loadpoint..loadpoint + 1]).to_string();
                    }
                    loadpoint = 8;
                    println!("{}", miftype);//prints MIF type
                    if miftype == "[MIF-rs]"{ // checks if it is compatible with MICA-rs
                        println!("found rust type mif, beggining loading sequence");
                        loadpoint += 1;//iterate to get to the start of the x value
                        mifdatax = mifdatax + &(&contents[loadpoint..loadpoint + 1]).to_string();
                        loadpoint += 1; //why can't rust just do foo ++; like C does
                        if &contents[loadpoint..loadpoint + 1] != ")" {
                            mifdatax = mifdatax + &(&contents[loadpoint..loadpoint + 1]).to_string();
                            loadpoint += 1; //why can't rust just do foo ++; like C does
                            if &contents[loadpoint..loadpoint + 1] != ")" {
                                mifdatax = mifdatax + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                loadpoint += 1; //why can't rust just do foo ++; like C does
                                if &contents[loadpoint..loadpoint + 1] != ")" {
                                    mifdatax = mifdatax + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                    loadpoint += 1; //why can't rust just do foo ++; like C does
                                    if &contents[loadpoint..loadpoint + 1] != ")" {
                                        mifdatax = mifdatax + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                        loadpoint += 1; //why can't rust just do foo ++; like C does
                                    }}}}
                                    println!("{}", mifdatax);


                                    loadpoint += 2;//iterate to get to the start of the y value
                                    mifdatay = mifdatay + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                    loadpoint += 1; //why can't rust just do foo ++; like C does
                                    if &contents[loadpoint..loadpoint + 1] != ">" {
                                        mifdatay = mifdatay + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                        loadpoint += 1; //why can't rust just do foo ++; like C does
                                        if &contents[loadpoint..loadpoint + 1] != ">" {
                                            mifdatay = mifdatay + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                            loadpoint += 1; //why can't rust just do foo ++; like C does
                                            if &contents[loadpoint..loadpoint + 1] != ">" {
                                                mifdatay = mifdatay + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                                loadpoint += 1; //why can't rust just do foo ++; like C does
                                                if &contents[loadpoint..loadpoint + 1] != ">" {
                                                    mifdatay = mifdatay + &(&contents[loadpoint..loadpoint + 1]).to_string();
                                                    loadpoint += 1; //why can't rust just do foo ++; like C does
                                                }}}}
                                                println!("{}", mifdatax);
                    }else{
                        println!("found incompatible MIF type (possibly MICA-gm / unimplemented type)")
                    }
                    //end of mif loader
                    //jk

                    println!("trying to set canvas size");
                    let wint: f32 = mifdatax.parse().unwrap();
                    let hint: f32 = mifdatay.parse().unwrap();
                    println!("{} {}", hint, wint);
                    w = wint as usize;
                    h = hint as usize;

                    image = Image::gen_image_color(w as u16, h as u16, WHITE);
                    image2 = Texture2D::from_image(&image);
                    flip = Image::gen_image_color(w as u16, h as u16, WHITE);
                    loadpoint += 1;
                    let contents_vec: Vec<char> = contents.chars().collect();
                    println!("{}", contents_vec.len());
                    for fy in 0..hint as i32 {
                        for fx in 0..wint as i32 {
                            currr = contents_vec[loadpoint];
                            loadpoint += 1;
                            currg = contents_vec[loadpoint];
                            loadpoint += 1;
                            currb = contents_vec[loadpoint];
                            loadpoint += 1;
                            curra = contents_vec[loadpoint];
                            loadpoint += 1;
                            colour = Color::from_rgba(currr as u8, currg as u8, currb as u8, curra as u8);
                            image.set_pixel(fx  as u32, fy as u32, colour);
                        }
                    }

                }


                //import menu
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 192. && mousey <= 224.){
                    let mut dothestinkyimportmenuwu = 1;
                    while dothestinkyimportmenuwu == 1 {
                        clear_background(WHITE);
                        draw_texture(&trans, 0., 0., WHITE);
                        image2.update(&image);
                        draw_texture(&image2, 32., 0., WHITE);
                        //draw the menu
                        draw_rectangle(0., 0., 32., 1900., WHITE);
                        draw_rectangle(32., 0., 2., 1900., GRAY);
                        draw_text("Size:", 0.0, screen_height() - 26., 16.0, BLACK);
                        draw_text(&size.to_string(), 10.0, screen_height() - 10., 16.0, BLACK);
                        draw_texture(&paint, 0., 0., WHITE);
                        draw_texture(&rubber, 0., 32., WHITE);
                        draw_texture(&colsel, 0., 64., WHITE);
                        draw_texture(&save, 0., 96., WHITE);
                        draw_texture(&mif, 0., 128., WHITE);
                        draw_texture(&mif_load, 0., 160., WHITE);
                        draw_texture(&import, 0., 192., WHITE);
                        draw_rectangle(34., 197., 80., 33., DARKGRAY);
                        draw_rectangle(34., 197., 79., 32., LIGHTGRAY);
                        draw_rectangle(35., 199., 77., 12., GRAY);
                        draw_rectangle(35., 199., 76., 11., WHITE);
                        draw_rectangle(35., 216., 77., 12., GRAY);
                        draw_rectangle(35., 216., 76., 11., WHITE);
                        draw_text("Load .ST", 36.0, 209.0, 16.0, BLACK);
                        draw_text("Load .PNG", 36.0, 226.0, 16.0, BLACK);
                        next_frame().await; //RENDER IT!
                        (mousex, mousey) = mouse_position(); //get mouse pos
                        //check if you are hovering over the menu if not stop drawing it

                        if !(mousex >= 0. && mousex <= 98.) && !(mousey >= 192. && mousey <= 240.){
                            println!("sop");
                            dothestinkyimportmenuwu = 0;
                        }
                        //png loader
                        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 34. && mousex <= 234.) && (mousey >= 210. && mousey <= 226.){

                            let mut loadfile = String::new();//init load string for png path
                            xsel = 1;
                            let mut error = false;
                            while xsel == 1{
                                while xsel == 1 {
                                    (mousex, mousey) = mouse_position();//get mouse position
                                    clear_background(WHITE);
                                    draw_text("Input file path eg. /home/user/Pictures/loadfile.png", 10.0, 60.0, 16.0, BLACK);
                                    draw_rectangle(550., 19., 21., 13., GRAY);
                                    draw_rectangle(550., 19., 20., 12., LIGHTGRAY);
                                    draw_text("OK", 550.0, 30.0, 16.0, BLACK);
                                    //text input
                                    let window_id = hash!();
                                    root_ui().window(
                                        window_id,
                                        vec2(0.0, EDITOR_TOP_MARGIN),
                                                    vec2(500.0, EDITOR_HEIGHT),
                                                    |ui| {
                                                        let input_text_id = hash!();
                                                        InputText::new(input_text_id)
                                                        .label("")
                                                        .size(vec2(496.0, EDITOR_HEIGHT - 4.0))
                                                        .ui(ui, &mut loadfile);

                                                    },

                                    );
                                    if error{
                                        draw_rectangle(64., 64., 300., 24., RED);
                                        draw_text("Incorrect path or incompatible file type", 79.0, 80.0, 16.0, WHITE);
                                        draw_text("Incorrect path or incompatible file type", 80.0, 79.0, 16.0, WHITE);
                                        draw_text("Incorrect path or incompatible file type", 81.0, 80.0, 16.0, WHITE);
                                        draw_text("Incorrect path or incompatible file type", 80.0, 81.0, 16.0, WHITE);
                                        draw_text("Incorrect path or incompatible file type", 80.0, 80.0, 16.0, BLACK);
                                    }
                                    if is_key_pressed(KeyCode::Enter){
                                        println!("ok you pressed it!");
                                        xsel = 0;
                                        draw_text("Loading", 10.0, 90.0, 16.0, BLACK);
                                        if loadfile == "" {
                                            draw_text("No Path", 80.0, 60.0, 64.0, BLACK);
                                        }
                                    }
                                    if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 550. && mousex <= 571.) && (mousey >= 19. && mousey <= 32.){
                                        println!("ok you pressed it!");
                                        println!("ok you pressed it!");
                                        xsel = 0;
                                        draw_text("Loading", 10.0, 90.0, 16.0, BLACK);
                                        if loadfile == "" {
                                            draw_text("No Path", 60.0, 60.0, 64.0, BLACK);
                                        }
                                    }
                                    next_frame().await; //RENDER IT!
                                }
                                match ImageInfo::from_file_path(&loadfile) {
                                    Ok(info) => {

                                        w = info.size.width as usize;
                                        h = info.size.height as usize;
                                        image = Image::gen_image_color(w as u16, h as u16, WHITE);
                                        image2 = Texture2D::from_image(&image);
                                    }
                                    Err(_err) => {
                                        error = true;
                                        xsel = 1;
                                    }
                                }
                                match fs::read(&loadfile) {
                                    Ok(bytes) => {
                                        image = Image::from_file_with_format(&bytes, Some(ImageFormat::Png)).expect("s");
                                    }
                                    Err(_err) => {
                                    println!("AARTSDTFDYTKYUERROR");
                                    error = true;
                                    xsel = 1;
                                    }
                                }
                            }
                        }
                        //benji's stupid .SHIT loading yes that is the actuall acronym
                        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 34. && mousex <= 234.) && (mousey >= 197. && mousey <= 209.){

                        println!("sorry this file format is too inefficient and hard to decode for us to be bothered JK we still do it");
                        let mut loadfile = String::new();//init load string for ST path
                        xsel = 1;
                        while xsel == 1 {
                            (mousex, mousey) = mouse_position();//get mouse position
                            clear_background(WHITE);
                            draw_text("Input file path eg. /home/user/Pictures/loadfile.st", 10.0, 60.0, 16.0, BLACK);
                            draw_rectangle(550., 19., 21., 13., GRAY);
                            draw_rectangle(550., 19., 20., 12., LIGHTGRAY);
                            draw_text("OK", 550.0, 30.0, 16.0, BLACK);
                            //text input
                            let window_id = hash!();
                            root_ui().window(
                                window_id,
                                vec2(0.0, EDITOR_TOP_MARGIN),
                                             vec2(500.0, EDITOR_HEIGHT),
                                             |ui| {
                                                 let input_text_id = hash!();
                                                 InputText::new(input_text_id)
                                                 .label("")
                                                 .size(vec2(496.0, EDITOR_HEIGHT - 4.0))
                                                 .ui(ui, &mut loadfile);

                                             },

                            );

                            if is_key_pressed(KeyCode::Enter){
                                println!("ok you pressed it!");
                                xsel = 0;
                                draw_text("Loading", 10.0, 90.0, 16.0, BLACK);
                                if loadfile == "" {
                                    draw_text("No Path", 80.0, 60.0, 64.0, BLACK);
                                }
                            }
                            if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 550. && mousex <= 571.) && (mousey >= 19. && mousey <= 32.){
                                println!("ok you pressed it!");
                                println!("ok you pressed it!");
                                xsel = 0;
                                draw_text("Loading", 10.0, 90.0, 16.0, BLACK);
                                if loadfile == "" {
                                    draw_text("No Path", 60.0, 60.0, 64.0, BLACK);
                                }
                            }
                            next_frame().await; //RENDER IT!
                        }
                        loadpoint = 1;
                        let contents = fs::read_to_string(Path::new(&loadfile)).expect("Should have been able to read the file");
                        let mut contents_vec: Vec<char> = contents.chars().collect();
                        let mut yes = 1;
                        stdatax = "".to_string();
                        if contents_vec[loadpoint] != '}' && yes == 1{
                            stdatax += &contents[loadpoint..loadpoint + 1];loadpoint += 1;
                        }else{yes=0;}

                        if contents_vec[loadpoint] != '}' && yes == 1{
                            stdatax += &contents[loadpoint..loadpoint + 1];loadpoint += 1;
                        }else{yes=0;}

                        if contents_vec[loadpoint] != '}' && yes == 1{
                            stdatax += &contents[loadpoint..loadpoint + 1];loadpoint += 1;
                        }else{yes=0;}

                        if contents_vec[loadpoint] != '}' && yes == 1{
                            stdatax += &contents[loadpoint..loadpoint + 1];loadpoint += 1;
                        }else{yes=0;}

                        if contents_vec[loadpoint] != '}' && yes == 1{
                            stdatax += &contents[loadpoint..loadpoint + 1];loadpoint += 1;
                        }else{yes=0;}

                        if contents_vec[loadpoint] != '}' && yes == 1{
                            stdatax += &contents[loadpoint..loadpoint + 1];loadpoint += 1;
                        }else{}
                        println!("{}", stdatax);
                        loadpoint += 1;
                        let mut did = 0;
                        while loadpoint <= contents_vec.len() - 1 {
                            //the following code is hella janky and could be written way better and simpler by just using a match case but I don't care

                            if contents_vec[loadpoint] == 'w'{
                                decomp += "255255255";did=1;
                            }
                            if contents_vec[loadpoint] == 'k'{
                                decomp += "000000000";did=1;
                            }
                            if contents_vec[loadpoint] == 'r'{
                                decomp += "255000000";did=1;
                            }
                            if contents_vec[loadpoint] == 'g'{
                                decomp += "255000000";did=1;
                            }
                            if contents_vec[loadpoint] == 'b'{
                                decomp += "255000000";did=1;
                            }
                            if contents_vec[loadpoint] == 'c'{
                                decomp += "000255255";did=1;
                            }
                            if contents_vec[loadpoint] == 'p'{
                                decomp += "255000255";did=1;
                            }
                            if contents_vec[loadpoint] == 'y'{
                                decomp += "255255000";did=1;
                            }

                            if did == 0 {

                                //insert numerical data (not chars)
                                if loadpoint != contents_vec.len() && (contents_vec[loadpoint] != 'w' || contents_vec[loadpoint] != 'k' || contents_vec[loadpoint] != 'r' || contents_vec[loadpoint] != 'g' || contents_vec[loadpoint] != 'b' || contents_vec[loadpoint] != 'c' || contents_vec[loadpoint] != 'p' || contents_vec[loadpoint] != 'y'){
                                    decomp += &(String::from(contents_vec[loadpoint]));

                                }
                            }else{did = 0;}
                            loadpoint += 1;

                        }
                        loadpoint = 0;
                        let mut cur: String;
                        let mut cug: String;
                        let mut cub: String;
                        let mut fy = 0;
                        let _fx = 0;
                        contents_vec = decomp.chars().collect();
                        println!("{}", decomp);
                        while contents_vec[loadpoint] != 'e'{
                            fy += 1;
                            for fx in 0..stdatax.parse::<i32>().unwrap(){
                                //load red channel
                                cur = contents_vec[loadpoint].to_string();
                                loadpoint += 1;
                                cur += &contents_vec[loadpoint].to_string();
                                loadpoint += 1;
                                cur += &contents_vec[loadpoint].to_string();
                                loadpoint += 1;

                                cug = contents_vec[loadpoint].to_string();
                                loadpoint += 1;
                                cug += &contents_vec[loadpoint].to_string();
                                loadpoint += 1;
                                cug += &contents_vec[loadpoint].to_string();
                                loadpoint += 1;

                                cub = contents_vec[loadpoint].to_string();
                                loadpoint += 1;
                                cub += &contents_vec[loadpoint].to_string();
                                loadpoint += 1;
                                cub += &contents_vec[loadpoint].to_string();
                                loadpoint += 1;


                                //set pixel colour
                                colour = Color::from_rgba(cur.parse::<u8>().unwrap(), cug.parse::<u8>().unwrap(), cub.parse::<u8>().unwrap(), 255);
                                image.set_pixel(fx  as u32, fy as u32, colour);
                            }
                        }
                    }


                    }
                }







                //MIF saving
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 128. && mousey <= 160.){

                    xsel = 1;
                    while xsel == 1 {
                        (mousex, mousey) = mouse_position();//get mouse position
                        clear_background(WHITE);
                        draw_text("Input file path eg. /home/user/Pictures/savefile.mif", 10.0, 60.0, 16.0, BLACK);
                        draw_rectangle(550., 19., 21., 13., GRAY);
                        draw_rectangle(550., 19., 20., 12., LIGHTGRAY);
                        draw_text("OK", 550.0, 30.0, 16.0, BLACK);
                        //text input
                        let window_id = hash!();
                        root_ui().window(
                            window_id,
                            vec2(0.0, EDITOR_TOP_MARGIN),
                                         vec2(500.0, EDITOR_HEIGHT),
                                         |ui| {
                                             let input_text_id = hash!();
                                             InputText::new(input_text_id)
                                             .label("")
                                             .size(vec2(496.0, EDITOR_HEIGHT - 4.0))
                                             .ui(ui, &mut savepath);

                                         },

                        );

                        if is_key_pressed(KeyCode::Enter){
                            println!("ok you pressed it!");
                            xsel = 0;
                            draw_text("Saving", 10.0, 90.0, 16.0, BLACK);
                            if savepath == "" {
                                draw_text("No Path", 80.0, 60.0, 64.0, BLACK);
                            }
                        }
                        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 550. && mousex <= 571.) && (mousey >= 19. && mousey <= 32.){
                            println!("ok you pressed it!");
                            println!("ok you pressed it!");
                            xsel = 0;
                            draw_text("Saving", 10.0, 90.0, 16.0, BLACK);
                            if savepath == "" {
                                draw_text("No Path", 60.0, 60.0, 64.0, BLACK);
                            }
                        }
                        next_frame().await; //RENDER IT!
                    }
                    println!("mi(l)f");
                    let output = File::create(&savepath);
                    let mut line: String = "[MIF-rs]".to_string();

                    // mif saving file creation
                    //resolution
                    line = line + "(";
                    line = line + &w.to_string();
                    line = line + ")";
                    line = line + "<";
                    line = line + &h.to_string();
                    line = line + ">";

                    for y in 0..h as i32{
                        for x in 0..w as i32{
                            flipcol = image.get_pixel(x as u32, y as u32);
                            r = (((flipcol.r - 0.0) /(1.0 - 0.0))*(255.0 - 0.0) )as u8;
                            g = (((flipcol.g - 0.0) /(1.0 - 0.0))*(255.0 - 0.0) )as u8;
                            b = (((flipcol.b - 0.0) /(1.0 - 0.0))*(255.0 - 0.0) )as u8;
                            a = (((flipcol.a - 0.0) /(1.0 - 0.0))*(255.0 - 0.0) )as u8;
                            line = line + &(r as char).to_string();
                            line = line + &(g as char).to_string();
                            line = line + &(b as char).to_string();
                            line = line + &(a as char).to_string();

                        }
                    }
                    write!(output.as_ref().expect("REASON"), "{}", line).expect("write failed");




                }


                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 96. && mousey <= 128.){



                    xsel = 1;
                    while xsel == 1 {
                        (mousex, mousey) = mouse_position();//get mouse position
                        clear_background(WHITE);
                        draw_text("Input file path eg. /home/user/Pictures/savefile.png", 10.0, 60.0, 16.0, BLACK);
                        draw_rectangle(550., 19., 21., 13., GRAY);
                        draw_rectangle(550., 19., 20., 12., LIGHTGRAY);
                        draw_text("OK", 550.0, 30.0, 16.0, BLACK);
                        //text input
                        let window_id = hash!();
                        root_ui().window(
                            window_id,
                            vec2(0.0, EDITOR_TOP_MARGIN),
                                         vec2(500.0, EDITOR_HEIGHT),
                                         |ui| {
                                             let input_text_id = hash!();
                                             InputText::new(input_text_id)
                                             .label("")
                                             .size(vec2(496.0, EDITOR_HEIGHT - 4.0))
                                             .ui(ui, &mut savepath);

                                         },

                        );

                        if is_key_pressed(KeyCode::Enter){
                            println!("ok you pressed it!");
                            xsel = 0;
                            draw_text("Saving", 10.0, 90.0, 16.0, BLACK);
                            if savepath == "" {
                                draw_text("No Path", 80.0, 60.0, 64.0, BLACK);
                            }
                        }
                        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 550. && mousex <= 571.) && (mousey >= 19. && mousey <= 32.){
                            println!("ok you pressed it!");
                            println!("ok you pressed it!");
                            xsel = 0;
                            draw_text("Saving", 10.0, 90.0, 16.0, BLACK);
                            if savepath == "" {
                                draw_text("No Path", 60.0, 60.0, 64.0, BLACK);
                            }
                        }
                        next_frame().await; //RENDER IT!
                    }




                    x = 0.0;
                    y = 1.0;
                    fx = 0;
                    fy = h as i32 -2;
                    while go == 1{
                        x += 1.0;
                        fx += 1;
                        flipcol = image.get_pixel(x as u32 ,y as u32);
                        flip.set_pixel(fx as u32 , fy as u32, flipcol);
                        if fx == w as i32 -1 {
                            x = 0.0;
                            fx = 0;
                            fy -= 1;
                            y += 1.0;
                            if y == h as f32 {
                                go =0;
                            }
                        }

                    }
                    (&flip).export_png(&savepath);
                }
                //colour swatch
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 64. && mousey <= 96.){
                    println!("you pressed it!"); state = 1;
                    colmenu = 1;
                    while colmenu == 1 {
                        (mousex, mousey) = mouse_position();
                        clear_background(WHITE);
                        draw_text("Colour menu and stuff  BACK", 1.0, 16.0, 16.0, BLACK);
                        draw_rectangle(1., 20., 32., 8., colour);

                        //red channel buttons
                        draw_texture(&plus, 0., 32., WHITE);
                        draw_texture(&down, 0., 64., WHITE);
                        draw_text(&cr.to_string(), 1.0, 110.0, 16.0, BLACK);
                        draw_text("Red", 1.0, 126.0, 16.0, BLACK);
                        //green channel buttons
                        draw_texture(&plus, 33., 32., WHITE);
                        draw_texture(&down, 33., 64., WHITE);
                        draw_rectangle(29., 94., 100., 16., WHITE);
                        draw_text(&cg.to_string(), 34.0, 110.0, 16.0, BLACK);
                        draw_text("Green", 32., 126.0, 16.0, BLACK);
                        //bleu channel buttons
                        draw_texture(&plus, 66., 32., WHITE);
                        draw_texture(&down, 66., 64., WHITE);
                        draw_rectangle(63., 94., 100., 16., WHITE);
                        draw_text(&cb.to_string(), 67.0, 110.0, 16.0, BLACK);
                        draw_text("Blue", 67.0, 126.0, 16.0, BLACK);
                        //alpha wolf channel buttons
                        draw_texture(&plus, 99., 32., WHITE);
                        draw_texture(&down, 99., 64., WHITE);
                        draw_rectangle(96., 94., 100., 16., WHITE);
                        draw_text(&ca.to_string(), 100.0, 110.0, 16.0, BLACK);
                        draw_text("Alpha", 100.0, 126.0, 16.0, BLACK);


                        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 230.) && (mousey >= 0. && mousey <= 16.){
                            colmenu = 0;
                            state = 0;
                        }

                        //up arrow
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 32. && mousey <= 64.){
                            if cr != 255{cr  += 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", cr);
                            if cr >= 255 {
                                cr = 255;
                            }
                            if cr <= 0 {
                                cr = 0;
                            }
                        }
                        //down arrow
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 64. && mousey <= 96.){
                            if cr != 0{cr -= 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", cr);
                            if cr >= 255 {
                                cr = 255;
                            }
                            if cr <= 0 {
                                cr = 0;
                            }
                        }


                        //up arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 66. && mousex <= 98.) && (mousey >= 32. && mousey <= 64.){

                            if cb != 255{cb += 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", cb);
                            if cb>= 255 {
                                cb = 255;
                            }
                            if cb <= 0{
                                cb = 0;
                            }
                        }
                        //down arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 66. && mousex <= 98.) && (mousey >= 64. && mousey <= 96.){
                            if cb != 0{cb -= 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", cb);
                            if cb >= 255 {
                                cb = 255;
                            }
                            if cb <= 0 {
                                cb = 0;
                            }
                        }



                        //up arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 99. && mousex <= 131.) && (mousey >= 32. && mousey <= 64.){
                            if ca != 255{ca += 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", ca);
                            if ca>= 255 {
                                ca = 255;
                            }
                            if ca <= 0 {
                                ca = 0;
                            }
                        }
                        //down arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 99. && mousex <= 131.) && (mousey >= 64. && mousey <= 96.){
                            if ca != 0{ca -= 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", cb);
                            if ca >= 255 {
                                ca = 255;
                            }
                            if ca <= 0 {
                                ca = 0;
                            }
                        }




                        //up arrow 3
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 33. && mousex <= 65.) && (mousey >= 32. && mousey <= 64.){
                            if cg != 255{cg += 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", cg);
                            if cg>= 255 {
                                cg = 255;
                            }
                            if cg <= 0 {
                                cg = 0;
                            }
                        }
                        //down arrow 3
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 33. && mousex <= 65.) && (mousey >= 64. && mousey <= 96.){
                            if cg != 0{cg -= 1;}
                            colour = Color::from_rgba(cr, cg, cb, ca);
                            println!("{}", cg);
                            if cg >= 255 {
                                cg = 255;
                            }
                            if cg <= 0 {
                                cg = 0;
                            }
                        }

                        next_frame().await; //RENDER IT!
                    }
                }




                //check if paint tool is selected
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 0. && mousey <= 32.){println!("you pressed paint!"); state = 1;}
                //eraser tool check
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 32. && mousey <= 64.){println!("you pressed erase!"); state = 2;}
                next_frame().await; //RENDER IT!
            }

        }



        //liscense button
        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 10. && mousex <= 507.) && (mousey >= screen_height() -23. && mousey <= screen_height()){
            println!("UWU~");//epic debug text
            liscense = 1; //sets to check if you are still looking
            while liscense == 1 {
                //clear and draw agreement
                clear_background(WHITE);
                draw_multiline_text("This program is free software: you can redistribute it and/or modify\nit under the terms of the GNU General Public License as published by\nthe Free Software Foundation, either version 3 of the License, or\n(at your option) any later version.\n\nThis program is distributed in the hope that it will be useful,\nbut WITHOUT ANY WARRANTY; without even the implied warranty of\nMERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the\nGNU General Public License for more details.\nhttps://www.gnu.org/licenses/gpl-3.0.txt", 20.0, 200.0, 16.0, Some(1.0), BLACK);

                //back button
                draw_rectangle(10., 581., 31., 11., GRAY);
                draw_rectangle(10., 581., 30., 10., LIGHTGRAY);
                draw_text("BACK", 10.0, 590.0, 16.0, BLACK);

                next_frame().await; //RENDER IT!
                sleep(time::Duration::from_millis(100));//this is nessescery for some reason
                (mousex, mousey) = mouse_position(); // get mouse pos

                //check if you pressed back
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 10. && mousex <= 80.) && (mousey >= 580. && mousey <= 590.){liscense = 0;}

            }

        }

        next_frame().await;//RENDER IT!





    }
}
