mod image_io;
mod sketch;
fn main() {
    let inp_path = "/home/gp/Documents/pencil-sketch/assets/t1.jpg";
    let op_path = "/home/gp/Documents/pencil-sketch/assets/t1_pencil_sketch.jpg";

    let mut img = image_io::read(inp_path);
    let img = sketch::sketch(&mut img, 40);

    image_io::write(img, op_path);

    let inp_path = "/home/gp/Documents/pencil-sketch/assets/t2.jpg";
    let op_path = "/home/gp/Documents/pencil-sketch/assets/t2_pencil_sketch.jpg";

    let mut img = image_io::read(inp_path);
    let img = sketch::sketch(&mut img, 40);

    image_io::write(img, op_path);

    let inp_path = "/home/gp/Documents/pencil-sketch/assets/t3.jpg";
    let op_path = "/home/gp/Documents/pencil-sketch/assets/t3_pencil_sketch.jpg";

    let mut img = image_io::read(inp_path);
    let img = sketch::sketch(&mut img, 40);

    image_io::write(img, op_path);

    let inp_path = "/home/gp/Documents/pencil-sketch/assets/t4.jpg";
    let op_path = "/home/gp/Documents/pencil-sketch/assets/t4_pencil_sketch.jpg";

    let mut img = image_io::read(inp_path);
    let img = sketch::sketch(&mut img, 40);

    image_io::write(img, op_path);

    let inp_path = "/home/gp/Documents/pencil-sketch/assets/t6.jpg";
    let op_path = "/home/gp/Documents/pencil-sketch/assets/t6_pencil_sketch.jpg";

    let mut img = image_io::read(inp_path);
    let img = sketch::sketch(&mut img, 200);

    image_io::write(img, op_path);
}
