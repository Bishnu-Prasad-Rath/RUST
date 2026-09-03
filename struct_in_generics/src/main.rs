struct Point <T,U>{
    x: T,
    y: U,        //Here when we declare genetics in struct u can use any kind of datatpyes for a specifc struct
}

impl<T,U> Point<T,U>{
  fn new -> self{
    Self{
      x,y
    }
  }
}

fn main() {

let point_a = Point{
  x : 10,
  y : 20,
}
  let point_b = Point{   //Here u have to make sure that the datatypes for both x and y must ssame because wheneverr u use generics on struct it inferes the type and then the value have also the same types and to solve this problem use can use multiple generics to use random datatypes.
  x : 10.1,
  y : 5.8,
  }
  
}
//U can use generics in most concepts in RUST