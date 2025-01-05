use std::io;


fn main () {
   //Office Administration vector
   let office_admin = vec!["Intern","Administrator","Senior Administrator","Office Manager",
  "Director","CEO"]; 

  //Academic
  let academic = vec!["Research Assisstant","PhD Candidate","Post-Doc Researcher",
  "Senior Lecturer","Dean"];

  //Lawyer
  let lawyer = vec!["Paralegal","Junior Associate","Associate","Senior Associate 1-2",
  "Senior Associate 3-4", "Partner"];

  //Teacher
  let teacher = vec!["Placement", "Classroom Teacher", "Snr Teacher", "Leading Teacher",
  "Deputy Principal", "Principal"];


  println!("\n WELCOME TO THE PUBLIC SERVICE LEVEL CHECKER FOR THE FEDERAL
        GOVERNMENT OF NIGERIA!");
        let v = vec!["Office Administrator","Academic","Lawyer","Teacher"];
    println!("{:?}",v);
    println!("Choose a department from the above list");
     let mut department = String::new();
    std::io::stdin().read_line(&mut department).expect("Invalid input");
    let mut level = String::new();
    std::io::stdin().read_line(&mut level).expect("Invalid input");
    let mut work_experience = String::new();
    std::io::stdin().read_line(&mut work_experience).expect("Invalid input");


    if department== "Office Administrator"{
         println!("Choose your level of service in Office Administration");
        println!("{:?}",office_admin);
         std::io::stdin().read_line(&mut level).expect("Invalid input");
        if level=="Intern"{
            println!("How many years work experience do you have?");
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 1 && work_experience <= 2{
            let postion= "APS 1-2";
        }
        }
       if level=="Administrator"{
         println!("How many years work experience do you have?");
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 3 && work_experience <= 5{
            let postion= "APS 3-5";
        }
       }
       if level =="Senior Administrator"{
        println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
         if work_experience >= 5 && work_experience <= 8{
            let postion= "APS 5-8";
         }
       } 
       if level =="Office Manager"{
        println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 8 && work_experience <= 10{
            let postion= "APS 8-10";
        }
       }
       if level =="Director"{
        println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 10 && work_experience <= 13{
            let postion= "APS 10-13";
        }
       }
       if level =="CEO"{
        let postion= "SES";
       }
    }
    else if department== "academic"{
        println!("Choose your level of service in Academics");
        println!("{:?}",academic);
        let mut level = String::new();
        std::io::stdin().read_line(&mut level).expect("Invalid input");
        if level=="Research Assisstant"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 3 && work_experience <= 5{
            let postion= "APS 3-5";
        }
        }
        if level=="PhD Candidate"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 5 && work_experience <= 8{
            let postion= "APS 5-8";
        }
        }
        if level=="Post-Doc"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 8 && work_experience <= 10{
            let postion= "APS 8-10";
        }
        }
        if level=="Senior Lecturer"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 10 && work_experience <= 13{
            let postion= "APS 10-13";
        }
        }
        if level=="Dean"{
            let postion= "SES";
        }
    }
    else if department== "Lawyer"{ println!("Choose your level of service in law");
        println!("{:?}",lawyer);
        let mut level = String::new();
        std::io::stdin().read_line(&mut level).expect("Invalid input");
        if level=="Paralegal"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 1 && work_experience <= 2{
            let postion= "APS 1-2";
        }
        }
        if level=="Junior Associate"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 3 && work_experience <= 5{
            let postion= "APS 3-5";
        }
        }
        if level=="Associate"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 5 && work_experience <= 8{
            let postion= "APS 5-8";
        }
        }
        if level=="Senior Associate 1-2"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 8 && work_experience <= 10{
            let postion= "APS 8-10";
        }
        }
        if level=="Senior Associate 3-4"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 10 && work_experience <= 13{
            let postion= "APS 10-13";
        }
        }
        if level=="Partner"{
            let postion= "SES";
        }
    }
    else if department== "Teacher"{
         println!("Choose your level of service in Teaching");
        println!("{:?}",teacher);
        let mut level = String::new();
        std::io::stdin().read_line(&mut level).expect("Invalid input");
        if level=="Placement"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 1 && work_experience <= 2{
            let postion= "APS 1-2";
        }
        }
        if level=="Classroom Teacher"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 3 && work_experience <= 5{
            let postion= "APS 3-5";
        }
        }
        if level=="Snr Teacher"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 5 && work_experience <= 8{
            let postion= "APS 5-8";
        }
        }
        if level=="Leading Teacher"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 8 && work_experience <= 10{
            let postion= "APS 8-10";
        }
        }
        if level=="Deputy Principal"{
            println!("Do you have any work experience?");
            let mut work_experience = String::new();
        std::io::stdin().read_line(&mut work_experience).expect("Invalid input");
        let work_experience:i32= work_experience.trim().parse().expect("Invalid");
        if work_experience >= 10 && work_experience <= 13{
            let postion= "APS 10-13";
        }
        }
        if level=="Principal"{
            let postion= "SES";
        }
    }
    else{
        println!("Your postion is not listed");
    }

    println!("\n Your Information: \n");
   println!("Department: {}", department);
   println!("level: {}", level);
  println!("Work Experience: {}", work_experience);
  println!("Position: {}", position);
    
        
}