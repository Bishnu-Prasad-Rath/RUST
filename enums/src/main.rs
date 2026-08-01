
#[derive(Debug)]
enum IPAddKind {
    v4,
    v6
}

struct IpAddress{
  address : String,
  kind : IPAddKind
}

fn main() {
    println!("Hello, world!");
    route("1.2.3.4",IPAddKind::v4);
}

fn route(ip : &str, kind : IPAddKind){
  println!("Routing request {} of kind {kind:?}",ip);
}