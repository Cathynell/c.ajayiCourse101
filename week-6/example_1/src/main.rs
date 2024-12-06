use std::io;

fn trapezuim(height:f32,base1:f32,base2:f32)->f32{
    let t:f32= (height/2.0)*(base1 + base2);
    return t;
}
fn rhombus(d1:f32,d2:f32)->f32{
    let r:f32=(d1*d2)/2.0;
    return r;
}
fn parallelogram(base:f32,altitude:f32)->f32{
    let p:f32=base*altitude;
    return p;
}
fn cube(length:i32)->i32{
    let c:i32=6*(length)^2;
    return c;
}
fn cylinder(_pie:i32,radius:i32,h:i32)->i32{
    let _pie:i32=22/7;
    let k:i32=_pie*((radius)^2)*h;
    return k;
}

fn main() {
    let mut userinput = String::new();

    //variables necessary for area of trapezium

    let mut height = String::new();
    let mut base1 = String::new();
    let mut base2 = String::new();


    //variables necessary for area of rhombus

    let mut diagonal1 = String::new();
    let mut diagonal2 = String::new();

    //variables necessary for area of parallelogram

    let mut altitude = String::new();
    let mut base = String::new();

    //variables necessary for area of cube

    let mut length = String::new();

    //variables necessary for area of cylinder

    let mut radius = String::new();
    let mut h = String::new();
    let _pie:i32 = 22/7;


    println!("\n.Please Enter the shape you want to work with!");

    println!("Input t to select Trapezuim");
    println!("Input r to select Rhombus");
    println!("Input p to select Parallelogram");
    println!("Input c to select Cube");
    println!("Input k to select Cylinder");
    println!("input your shape of choice");
    io::stdin().read_line(&mut userinput).expect("we do not have that formula input yet");
    let userinput = userinput.trim();


    if userinput == "t"{
         println!("Enter the height of the trapezuim:");
         io::stdin().read_line(&mut height).expect("Invalid");
         let height:f32 = height.trim().parse().expect("Invalid");


        println!("Enter the base1:");
        io::stdin().read_line(&mut base1).expect("Invalid");
        let base1:f32 = base1.trim().parse().expect("Invalid");

        println!("Enter base2:");
        io::stdin().read_line(&mut base2).expect("Invalid");
        let base2:f32 = base2.trim().parse().expect("Invalid");

        println!("Area of your trapezium is {}", trapezuim(height,base1,base2));

    } else if userinput =="r"{
        println!("Enter diagonal 1");
        io::stdin().read_line(&mut diagonal1).expect("Invalid");
        let diagonal1:f32 = diagonal1.trim().parse().expect("Invalid");

        println!("Enter diagonal 2");
         io::stdin().read_line(&mut diagonal2).expect("Invalid");
        let diagonal2:f32 = diagonal2.trim().parse().expect("Invalid");
        
        println!("Area of Rhombus is {}", rhombus(diagonal1,diagonal2));
        
    } else if userinput =="p"{
        println!("Enter the base");
        io::stdin().read_line(&mut base).expect("Invalid");
        let base:f32 = base.trim().parse().expect("Invalid");

        println!("Enter the altitude");
        io::stdin().read_line(&mut altitude).expect("Invalid");
        let altitude:f32 = altitude.trim().parse().expect("Invalid");

        println!("Area of Parallelogram is {}",parallelogram(base,altitude) );
        
    }else if userinput =="c"{
         println!("Enter the length");
         io::stdin().read_line(&mut length).expect("Invalid");
         let length:i32 = length.trim().parse().expect("Invalid");
         println!("Area of cube is:{}", cube(length));
        
    }else if userinput=="k"{
        println!("Enter height");
        io::stdin().read_line(&mut h).expect("Invalid");
        let h:i32 = h.trim().parse().expect("Invalid");

        println!("Enter the raduis of the cylinder");
        io::stdin().read_line(&mut radius).expect("Invalid");
        let radius:i32 = radius.trim().parse().expect("Invalid");

        println!("Area of Cylinder is: {}", cylinder(h,radius,_pie));
    }else {
        let b="We do not have that formula yet";
        println!("{}",b);
    }
} 

