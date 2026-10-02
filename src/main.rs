/*
By: <Zander Stephens>
Date: 2026-09-28
Program Details: <Says nursery rhymes>
*/



mod ui;
mod utils;

use crate::utils::scale::use_virtual_resolution;
use macroquad::prelude::*;
//use crate::ui::grid::draw_grid;
use crate::ui::text_button::TextButton;
use crate::ui::label::Label;
use crate::ui::still_image::StillImage;
use crate::utils::preload_image::TextureManager;

/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "nurseryrhymesmorelikepoo".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()

        
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let btn_bell = TextButton::new(
        50.0,
        700.0,
        200.0,
        60.0,
        "Ding Dong Bell",
        DARKBROWN,
        RED,
        30
    );
    
let btn_egg = TextButton::new(
        250.0,
        700.0,
        200.0,
        60.0,
        "Humpty Dumpty",
        BEIGE,
        GOLD,
        30
    );

    let btn_cake = TextButton::new(
        450.0,
        700.0,
        200.0,
        60.0,
        "Patty Cake",
        BROWN,
        BLUE,
        30
    );

let btn_exit = TextButton::new(
        650.0,
        700.0,
        200.0,
        60.0,
        "Exit",
        RED,
        BLACK,
        30
    );


    let mut lbl_out = Label::new("hello i like nursery rhymes", 450.0, 100.0, 30);
    let tm = TextureManager::new();
    lbl_out.with_border(BLACK, 2.0);
    tm.preload_with_loading_screen(&["assets/bell.png","assets/Empty.png","assets/EGG.png","assets/cake.png"], None, None).await;
 let mut img_out = StillImage::from_preload(
        tm.get_preload("assets/Empty.png").unwrap(),
        400.0,
        400.0,
        30.0,
        150.0,
        true,
        1.0,
    );

        
    
    loop {
        clear_background(WHITE);
  //draw_grid(50.0, BROWN);
         // Set the virtual resolution to 1024x768
        use_virtual_resolution(1024.0, 768.0);
      if btn_bell.click() { lbl_out.set_text("Ding Dong Bell,
Pussy's in the well.
Who put her in?
Little Johnny Flynn.
Who pulled her out?
Little Tommy Stout.
What a naughty boy was that,
To soak a Pussy cat,
Who never did him any harm,
But killed all the mice in the barn.");
 img_out.set_preload(tm.get_preload("assets/bell.png").unwrap());
      }

      if btn_egg.click() { lbl_out.set_text("Humpty Dumpty sat on a wall,
Humpty Dumpty had a great fall.
All the king's horses 
and all the king's men
Couldn't put Humpty together again.");
 img_out.set_preload(tm.get_preload("assets/EGG.png").unwrap());
      }

      if btn_cake.click() { lbl_out.set_text("Pat-a-cake, 
 pat-a-cake, 
baker's man,
Bake me a cake as fast as you can;
Pat it, and roll it, 
and mark it with a B,
And put it in the oven for 
baby and me.");
 img_out.set_preload(tm.get_preload("assets/cake.png").unwrap());
      }

if btn_exit.click() {
            return;
        }

lbl_out.draw();
img_out.draw();

        next_frame().await; //put everything that needs to be drawn in the loop before this line or you die forever
     
    }
}
