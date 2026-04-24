use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets::InputText};
const EDITOR_TOP_MARGIN: f32 = 10.0;
const EDITOR_HEIGHT: f32 = 40.0;
pub fn draw(mut image: Image, mousex: f32, mousey: f32, mousx: f32, mousy: f32, colour: Color, size: i32, w: usize, h: usize) -> Image{
    let mut sdifx: f32;
    let mut sdify: f32;
    let stepy: f32;
    let mut stepx: f32;
    let mut step: i32;
    let mut x: f32;
    let mut y: f32;
    let _i: i128;
    let _i2: i64;
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
    return image;
}



pub fn menu(paint: Texture2D, rubber: Texture2D, colsel: Texture2D, save: Texture2D,  mif: Texture2D,  mif_load: Texture2D,  import: Texture2D, size: i32, colour: Color){
    //draw the menu
    draw_rectangle(0., 0., 32., 1900., WHITE);
    draw_rectangle(32., 0., 2., 1900., GRAY);
    draw_text("Size:", 0.0, screen_height() - 26., 16.0, BLACK);
    draw_text(&size.to_string(), 10.0, screen_height() - 10., 16.0, BLACK);
    draw_texture(&paint, 0., 0., WHITE);
    draw_texture(&rubber, 0., 32., WHITE);
    draw_texture(&colsel, 0., 64., WHITE);
    draw_texture(&save, 0., screen_height()-160., WHITE);
    draw_texture(&mif, 0., screen_height()-128., WHITE);
    draw_texture(&mif_load, 0., screen_height()-192., WHITE);
    draw_texture(&import, 0., screen_height()-96., WHITE);
    draw_rectangle(0., screen_height()-65., 32., 16., BLACK);
    draw_rectangle(1., screen_height()-64., 30., 14., colour);
}

//defines a standard text box input
pub async fn textinput(save: bool, error: bool, extension: String) -> String{
    //declare nessescary variables
    let mut loadfile = String::new();
    let mut mousex: f32;
    let mut mousey: f32;
    let mut xsel = 1;//I COULD use break but nah I'm too cool for that!
    while xsel == 1 {
        (mousex, mousey) = mouse_position();//get mouse position
        clear_background(WHITE);
        if !save{
            draw_text(&("Input file path eg. /home/user/Pictures/loadfile".to_owned() + &extension), 10.0, 60.0, 16.0, BLACK);
        }
        if save{
            draw_text(&("Input file path eg. /home/user/Pictures/savefile".to_owned() + &extension), 10.0, 60.0, 16.0, BLACK);
        }
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
            draw_rectangle(78., 68., 284., 19., RED);
            draw_text("Incorrect path or incompatible file type", 79.0, 80.0, 16.0, WHITE);
            draw_text("Incorrect path or incompatible file type", 80.0, 79.0, 16.0, WHITE);
            draw_text("Incorrect path or incompatible file type", 81.0, 80.0, 16.0, WHITE);
            draw_text("Incorrect path or incompatible file type", 80.0, 81.0, 16.0, WHITE);
            draw_text("Incorrect path or incompatible file type", 80.0, 80.0, 16.0, BLACK);
            //guys I think the filetype is wrong or could it be the path?
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
    return loadfile;
}
