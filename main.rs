trait Shape {
  fn area(&self);
  }
  
  struct Circle{x:f32}
  struct Triangle{l:f32,h:f32}
  struct Square{x:f32}
  impl Shape for Circle {
    fn area(&self) { println!("x={}",3.14*self.x*self.x); }
  }
  
  impl Shape for Triangle {
    fn area(&self) { println!("x={}",0.5*self.l*self.h); }
  }
  impl Shape for Square {
    fn area(&self) { println!("x={}",self.x*self.x); }
  }
pub fn main(){   
    let c = Circle {x:2.0};
    c.area();
    let t = Triangle {l:2.0,h:1.0};
    t.area();
    let s = Square {x:2.0};
    s.area();
}