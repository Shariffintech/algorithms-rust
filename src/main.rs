use std::str::Split;


fn split_the_string(){
    let manchester_united = ["Ronadlo", "Sancho","Rashford", "Bruno","Antony"];
    let cities: Split<&str> = "Manchester United will win the prem in 2022".split(" ");

    for city in cities {

        println!("{}", city);
    }

    for squad in manchester_united {
        println!("{}", squad)
    }

}


// fn second_string_split(){

//     let text: &str = "Manchester United for lifers";
//     let cities: Split<&str> = text.split(" ");

//     for city in cities {
//         println!("City {}", city);
//     }
// }


// fn count_for_clicks(){
//     // Suppose that now we have a list of counts of clicks and corresponding urls such as follows:
//     let counts = [
//         "50,google.com",
//      "60,yahoo.com",
//      "10,yahoo.com",
//      "1,wikipedia.org",
//      "40,sports.yahoo.com",
//      "300,yahoo.com",
//      "2,wikipedia.org",
//      "1,stackoverflow.com",
//      "1,google.com"
//     ];

//     // Sample output:
//     // sum_clicks(counts) => 465

//     // Complexity variables:
//     // n: the number of strings in the list
//     // (The length of each input string has a constant upper limit.)


//     // Write a function that takes this list and returns the sum of all the clicks. 


//     // Write a function that takes in a string like this and returns the number preceding the url in "integer form".



//     // fn integerFormString(counts){
//     //     let value = "";
//     //     let result = loop {
//     //      }
//     // }
    
//     // fn integerFormString(counts){

//     //   stores the count of clicks per string/ url
//     //   let howManyClicks = [];

//     //   let value = "";
//     //   for (let i = 0; i < counts.length; i++){
//     //     if (counts[i] === ","){
//     //       break
//     //     } else {
//     //       let newValue = value += counts[i];
//     //       parseInt(newValue)

//     //     }

//     //     split the string, seperated by comma (regex?)
//     //     condition if ","

//     //     convert string to array

//     //     set a condition for null values, if there is an # after splitting convert to int
//     //   }
//     //   return 
    
//     //   return values in url
    
    
//     // }

//     // console.log(integerFormString(url_count1))
//     // console.log(integerFormString(url_count2))
//     // console.log(integerFormString(url_count3))
//     // console.log(integerFormString(url_count4))
// }

// pub fn at(&mut self, index: usize) -> Option<u64> {
//     if self.length > index {
//         self.buf[index]
//     } else {
// None }
// }


fn main() {

    let mut s = String::from("Manchester United will win");
    change(&mut s);
    }
    
    fn change(some_string: &mut String) {
    some_string.push_str(", the premier league 2022/23");


    split_the_string();


    fn sliding_window(){

    }

    fn two_pointer(){
        
    }

    fn fast_slow_pointer(){

    }

    fn merge_intervals(){

    }

    fn cyclic_sort(){

    }

    fn in_place_reversal_of_linked_list(){

    }

    fn tree_breadth_first_search(){

    }

    fn tree_depth_first_search(){

    }

    fn find_minimum_subtree(){

    }

    fn two_heaps(){

    }

    fn subsets(){

    }

    fn modified_binary_search(){

    }

    fn top_k_elements(){

    }

    fn k_way_merge(){

    }

    fn topological_sort(){
        
    }

}






