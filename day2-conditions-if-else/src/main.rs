fn main() {
    
    // Condition operators
    // < > == <= >= != 
    // () are mandatory, see below:

    let condition = (2 as f32) < 2.2;
    // println!("{:?}", condition);

    // &&  ||  !
    // and or not

    let dinner = "Naan";
    if (condition == true) && !(dinner == "Paratha") {
        println!("Yay..!");
    }

    if (condition == true) || dinner == "Paratha" {
        println!("Yay..!");
    }

    if (condition == true) || dinner == "Paratha" {
        println!("Yay..!");
    }

    if (condition != true) && dinner != "Paratha" {
        println!("Yay..!");
    }

    else if dinner == "Naan" {
        println!("Someone's favourite...");
    } else {
        println!("Let me bring it..");
    }

}
