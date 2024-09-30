use nalgebra_glm::Vec3;
use super::framebuffer::Framebuffer;

pub trait Line {
    fn line(&mut self, start : Vec3, end : Vec3);
}

impl Line for Framebuffer {
    fn line(&mut self, start : Vec3, end : Vec3){
        let (mut x1, mut y1) = ( start.x as isize, start.y as isize);
        let (x2, y2) = ( end.x as isize, end.y as isize);

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx - dy;

        loop {
            self.draw_point(x1 as usize, y1 as usize); // Draw the point

            if x1 == x2 && y1 == y2 { break; }

            let e2 = err * 2;

            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }

            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
    }
}