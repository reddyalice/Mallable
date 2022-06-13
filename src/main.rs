mod consts;
use bevy::prelude::*;

fn main() {
   App::new()
       .insert_resource(ClearColor(consts::CLEAR))
       .insert_resource(WindowDescriptor{
          width : consts::MIN_WIDTH,
          height : consts::MIN_HEIGHT,
          transparent : true,
          title : String::from("Test"),
          .. Default::default()
       })
       .add_plugins(DefaultPlugins)
       .run();
}
