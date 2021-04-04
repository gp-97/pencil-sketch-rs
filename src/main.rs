mod image_io;
mod sketch;
fn main() {
    let inp_path = "/home/gp/Documents/pencil-sketch/assets/t1.jpg";
    let op_path = "/home/gp/Documents/pencil-sketch/assets/t1_pencil_sketch.jpg";

    let mut img = image_io::read(inp_path);
    let img = sketch::sketch(&mut img, 40);

    image_io::write(img, op_path);
}
