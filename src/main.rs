//TODO: Create ppm image (https://netpbm.sourceforge.net/doc/ppm.html)
#[derive(Debug)]
struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Colour {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn r(&self) -> &u8 {
        &self.r
    }

    pub fn g(&self) -> &u8 {
        &self.g
    }

    pub fn b(&self) -> &u8 {
        &self.b
    }
}

#[derive(Debug)]
struct Point<'p> {
    x: usize,
    y: usize,
    c: &'p Colour,
}

impl<'p> Point<'p> {
    pub fn new(x: usize, y: usize, c: &'p Colour) -> Self {
        Self { x, y, c }
    }

    pub fn x(&self) -> &usize {
        &self.x
    }

    pub fn y(&self) -> &usize {
        &self.y
    }

    pub fn c(&self) -> &'p Colour {
        self.c
    }
}

//Writes ppm file to the default path
pub fn ppm() -> Result<()> {
    unimplemented!();
}

fn main() {
    let c = Colour::new(10, 15, 8);
    dbg!(c.r());
    dbg!(c.g());
    dbg!(c.b());

    let p = Point::new(100, 300, &c);
    dbg!(p.x());
    dbg!(p.y());
    dbg!(p.c());

    dbg!(c.g());
}
