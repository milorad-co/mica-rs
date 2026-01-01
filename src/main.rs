use macroquad::prelude::*;
use std::{time};
use std::thread::sleep;
use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets::InputText},
    window::request_new_screen_size,
};
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
    let liscense: Texture2D = load_texture("mica_assets/oneshot.png").await.unwrap();
    let rubber: Texture2D = load_texture("mica_assets/image/erase.png").await.unwrap();
    let colsel: Texture2D = load_texture("mica_assets/image/colour.png").await.unwrap();
    let trans: Texture2D = load_texture("mica_assets/image/trans.png").await.unwrap();
    let down: Texture2D = load_texture("mica_assets/image/down.png").await.unwrap();
    let plus: Texture2D = load_texture("mica_assets/image/plus.png").await.unwrap();
    //input variables
    let mut mousex: f32;
    let mut mousey: f32;
    //previous positions
    let mut mousx: f32;
    let mut mousy: f32;
    let mut liscense = 0; //are you looking at that GNU gpl v3 again?
    let mut colmenu = 0; //colour menu
    //line drawing stuff
    let mut sdifx: f32;
    let mut sdify: f32;
    let mut stepy: f32;
    let mut stepx: f32;
    let mut step: i32;
    let mut x: f32;
    let mut y: f32;
    let mut size: i32  = 42; //life the universe and everything
    let (mut mouse_wheel_x, mut mouse_wheel_y) = mouse_wheel();
    let mut i: i128;
    let mut i2: i64;
    //colour stuff
    let mut colour = Color::new(0.50, 0.5, 0.5, 1.00);
    let erase = Color::new(0.00, 0.0, 0.0, 0.00);
    let mut state = 0;
    let mut cr = 0.0;
    let mut cg = 0.0;
    let mut cb = 0.0;
    let mut ca = 1.0;


    let mut xsel = 0;
    let mut ysel = 0;
    let mut datax = String::new();
    let mut datay = String::new();
(mousex, mousey) = mouse_position();

    //loop (woah no kidding)
    loop {
        mousx = mousex;
        mousy = mousey;

        (mousex, mousey) = mouse_position();//get mouse position
        //clears and draws menu
        clear_background(WHITE);
        draw_texture(&logo, (screen_width()/2. - 250.), 0., WHITE);
        draw_texture(&imageeditor, (screen_width()/2. - 150.), (screen_height()/2.), WHITE);

        //liscense button
        draw_rectangle(10., screen_height() - 19., 435., 13., GRAY);
        draw_rectangle(10., screen_height() - 19., 434., 12., LIGHTGRAY);
        draw_text("By using this software you agree to the terms of this liscense", 10.0, screen_height() - 10.0, 16.0, BLACK);


        //checks if you pressed image editor
        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= (screen_width()/2. - 150.) && mousex <= (screen_width()/2. + 150.)) && (mousey >= (screen_height()/2.) && mousey <= (screen_height()/2. + 100.)){

            xsel = 1;
            while(xsel == 1){
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
                    if(datax == ""){
                        ysel = 0;
                    }
                }
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 105. && mousex <= 126.) && (mousey >= 19. && mousey <= 32.){
                    println!("ok you pressed it!");
                    xsel = 0;
                    ysel = 1;
                    if(datax == ""){
                        ysel = 0;
                    }
                }
                next_frame().await; //RENDER IT!
            }
            while(ysel == 1){
                (mousex, mousey) = mouse_position();//get mouse position
                clear_background(WHITE);
                draw_text("Input Y size (draw area width)", 10.0, 60.0, 16.0, BLACK);
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
                    xsel = 0;
                    ysel = 0;
                }
                if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 105. && mousex <= 126.) && (mousey >= 19. && mousey <= 32.){
                    println!("ok you pressed it!");
                    xsel = 0;
                    ysel = 0;
                }
                next_frame().await; //RENDER IT!
            }


             println!("poraro");
            //init texture|
            let texture = Texture2D::empty();
            //image shit for drawing
            let mut w = 5 as usize;
            let mut h = 5 as usize;
            if(datax == ""){
                h = screen_height() as usize;
                w = screen_width() as usize;
            }else{
            let wint: f32 = datax.parse().unwrap();
            let hint: f32 = datay.parse().unwrap();
             w = wint as usize;
             h = hint as usize;
            }
            let mut image = Image::gen_image_color(w as u16, h as u16, WHITE);
            let image2 = Texture2D::from_image(&image);
            loop{
                mousx = mousex;
                mousy = mousey;
                (mousex, mousey) = mouse_position(); //get mouse pos

                //draw menu
                clear_background(WHITE);
                draw_texture(&trans, 0., 0., WHITE);

                (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();
                println!("{} {}",mouse_wheel_x, mouse_wheel_y);
                if mouse_wheel_y == -1. {size += -1;}
                if mouse_wheel_y == 1. {size += 1;}

                //draw code
                if is_mouse_button_down(MouseButton::Left) && state == 1 {
                    println!("down and in range!");
                    //line drawing code (sponsored by milorad)

                    step = ((mousex - mousx).abs()) as i32;
                   if(step >= ((mousey - mousy).abs()) as i32){
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
                                if(i + (x) as i32 >= w.try_into().unwrap()){
                                }else{
                                    if(i2 + (y) as i32 >= h.try_into().unwrap()){
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
                    if(step >= ((mousey - mousy).abs()) as i32){
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
                                if(i + (x) as i32 >= w.try_into().unwrap()){
                                }else{
                                    if(i2 + (y) as i32 >= h.try_into().unwrap()){
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

                draw_text("Size:", 0.0, (screen_height() - 26.), 16.0, BLACK);
                draw_text(&size.to_string(), 10.0, (screen_height() - 10.), 16.0, BLACK);
                draw_texture(&paint, 0., 0., WHITE);
                draw_texture(&rubber, 0., 32., WHITE);
                draw_texture(&colsel, 0., 64., WHITE);


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
                            cr += 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", cr);
                            if cr >= 1.0 {
                                cr = 1.;
                            }
                            if cr <= -0.001 {
                                cr = 0.0;
                            }
                        }
                        //down arrow
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 0. && mousex <= 32.) && (mousey >= 64. && mousey <= 96.){
                            cr -= 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", cr);
                            if cr >= 1.0 {
                                cr = 1.;
                            }
                            if cr <= -0.001 {
                                cr = 0.0;
                            }
                        }


                        //up arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 66. && mousex <= 98.) && (mousey >= 32. && mousey <= 64.){
                            cb += 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", cb);
                            if cb>= 1. {
                                cb = 1.;
                            }
                            if cb <= -0.001 {
                                cb = 0.0;
                            }
                        }
                        //down arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 66. && mousex <= 98.) && (mousey >= 64. && mousey <= 96.){
                            cb -= 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", cb);
                            if cb >= 1.0 {
                                cb = 1.;
                            }
                            if cb <= -0.001 {
                                cb = 0.0;
                            }
                        }



                        //up arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 99. && mousex <= 131.) && (mousey >= 32. && mousey <= 64.){
                            ca += 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", ca);
                            if ca>= 1.0 {
                                ca = 1.;
                            }
                            if ca <= -0.001 {
                                ca = 0.0;
                            }
                        }
                        //down arrow 2
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 99. && mousex <= 131.) && (mousey >= 64. && mousey <= 96.){
                            ca -= 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", cb);
                            if ca >= 1.0 {
                                ca = 1.;
                            }
                            if ca <= -0.001 {
                                ca = 0.0;
                            }
                        }




                        //up arrow 3
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 33. && mousex <= 65.) && (mousey >= 32. && mousey <= 64.){
                            cg += 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", cg);
                            if cg>= 1.0 {
                                cg = 1.;
                            }
                            if cg <= -0.001 {
                                cg = 0.0;
                            }
                        }
                        //down arrow 3
                        if is_mouse_button_down(MouseButton::Left) && (mousex >= 33. && mousex <= 65.) && (mousey >= 64. && mousey <= 96.){
                            cg -= 0.01;
                            colour = Color::new(cr, cg, cb, ca);
                            println!("{}", cg);
                            if cg >= 1.0 {
                                cg = 1.;
                            }
                            if cg <= -0.001 {
                                cg = 0.0;
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
        if is_mouse_button_pressed(MouseButton::Left) && (mousex >= 10. && mousex <= 507.) && (mousey >= screen_height() -30. && mousey <= screen_height()){
            println!("UWU~");//epic debug text
            liscense = 1; //sets to check if you are still looking
            while(liscense == 1){
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
