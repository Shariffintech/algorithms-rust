fn test(my_str:String) -> String {
    //initialize a va of type string with null value
     let mut sharifs_updated_string = "".to_string();

    //traverse the string using a for loop splitting the whitespace
    for c_words in my_str.split_whitespace(){
        // check IF starting letter of the word is c
        if c_words.starts_with("c"){
            //append the word in sharifs_updated_string
            sharifs_updated_string.push_str(c_words);
            // append the space in sharifs_updated_string
            sharifs_updated_string.push(' ');

        }
    }

    sharifs_updated_string.pop();
    sharifs_updated_string
}