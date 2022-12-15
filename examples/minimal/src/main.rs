extern crate initials_revamped;

use initials_revamped::AvatarBuilder;

pub fn main() {
    let image = AvatarBuilder::new("Anakin Skywalker").draw();
    image.save("minimal.jpg").unwrap();
}
