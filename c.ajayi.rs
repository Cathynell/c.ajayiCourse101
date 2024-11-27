use std::io;
fn main() 
{
        print=("\n.Enter your siblings information!");
	 let mut first_name= get_input("first_name");
	 let mut age:u32= get_input("age").trim().parse().expect("Please input valid age");
	 let mut gender= get_input("gender");
	 let mut country= get_input("Country");
	 let  status= get_input("married","single","student","employed","relationship",);
	 let children=get_input("yes/no");

	 if age>=18
	{
	 
	 println!("Enter your marital status");

	  if  status=="married"{
	 	println!("Do you have children?");
       }
	 	else if children==yes{
	 		let mut child_name=get_input("child_name");
	 		let mut childs_age:u32=get_input("childs_age").trim().parse().expect("Input a valid age");
	 		let mut childs_school=get_input("childs_school");
	 		let  city= "abuja";
	 		println!("You live in ",city);
	 	}else if  status=="single"{
	 		println!("Are you a student?");
	 		if status="yes";
	 	{
	 		let university="Pan-Atlantic University";
	 		let course="Software Engineering";
	 		let level:u32="300";
	 		if status ="employed";
	      {
	 		let work=("Remote","hybrid","onsite");
	 		if work="onsite";
	 	{
	 		let company_name="Shell";
	 		let job_title="Accountant";
	 	} let indutry_sector="Accountancy";
	     }

	 	}else if status=="relationship"
	 	{
	 		let mut relationship_duration:u32=get_input("relationship_duration");
	 		let mut partners_firstname=get_input("partners_firstname");
	 		let living_together="yes/no"
	 		println!("Do you live with your partner?");
	 		if living_together="yes";
	 		let city="benin";
	 		println!("You live in",benin);
        }
    }else if age<18{
    	let waec_status="yes/no";
    	println!("Are you done with waec?",);
        if waec_status="yes"
        {
        let secondary_school="Dansol High School";
        let final_grade="A+";
        let school_year:u32="2024"; 
        }if waec_status="no"{
        	let mut class:alpha_numeric=("");
        	println!("What class are you in?");
        	let plan=("soon","Not now")
        	println!("When do you plan to take the exam");
        	if plan="soon"
        	{
        	let exam_year:u32=2028;
        	println!("You will write the exam in",exam_year);
           }
        }
    }

    println!("\n.Client siblings information
    	      first_name:
    	      age:
    	      gender:
    	      country:
    	      status:
    	      children:");
              if children=="yes";
              println!("
    	      child_name:
    	      childs_age:
    	      childs_school:
    	      city:"child_name,childs_age,childs_school,city);
    	      if status=="student";{
    	      	println!("university:
    	      	course:
    	      	level:" university,course,level);
    	      }
    	      if status=="employed";{
    	      	println!("work:"work);
    	      	if work=="onsite";
    	      	println!("company_name:
    	      		job_title:
    	      		indutry_sector:" company_name,job_title,indutry_sector);
    	      }if age<18;{
    	      	println!("waec_status:
    	      		"waec_status);
    	      	if waec_status=="yes";
    	      	println!("secondary_school:
    	      		final_grade:
    	      		school_year:"final_grade,school_year);
    	      	if waec_status=="no";
    	      	println!("class:
    	      		plan:"class,plan);
    	      	if plan=="soon";
    	      	println!("exam_year",exam_year);
    	      }
    	      println!("------------------------");
    	      
}
}
