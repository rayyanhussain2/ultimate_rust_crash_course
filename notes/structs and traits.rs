//Again like parameters, no use of let
#[warn(unused_variables, dead_code, non_snake_case)]
struct RedFox{
    enemy: bool, //field 
    life: u8,
}

//Methods are implemented in another block using impl keyword with the same struct name
impl RedFox {
    fn new() -> Self { //self is an instance of the struct
        Self{
            enemy: true, 
            life: 70,
        }
    }
}

trait Fly{
    fn printFly(&self) -> &str;
}

impl Fly for RedFox{
    fn printFly(&self) -> &str{
        return "I believe I can fly!";
    }
}

fn main(){
let fox = RedFox{
    enemy: true,
    life: 9,
};

let foxxy = RedFox::new(); //scope operator used for instantiating the struct via constructor
//after which you can use the dot notation to access the instance fields and methods
println!("{}", foxxy.printFly());
}

/*traits are general methods/functions definitions
which are then implemented for each struct individually*/

