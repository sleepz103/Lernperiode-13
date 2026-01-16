fn main() {


    // comperable
    let mut cond = 2 < 3;

    // uncomperable because different types
    // let cond = 2 <= 2.2;

    // comperable - int converted to float 32
    cond = (2 as f32) <= 2.2;


    let logicANDCondition = true && cond;

    let logicORNOTCondition = logicANDCondition || !true;
    
    println!{"---------------------------"}
    println!{"cond: {}", cond};
    println!{"logicANDCondition: {}", logicANDCondition};
    println!{"logicORNOTCondition: {}", logicORNOTCondition};

    let food = "apple";

    if food == "cookie" {
        println!("I like cookies too!")
    } else if food == "apple"{
        println!("healthy!")
    } else {
         println!("Oh that's too bad!")
    }
}
