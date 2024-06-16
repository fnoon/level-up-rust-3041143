fn sort_usernames<T: AsRef<str>>(usernames: &mut [T]) {
//  usernames.sort_by(|a,b| a.as_ref().to_lowercase().cmp(&b.as_ref().to_lowercase()));
    usernames.sort_by_cached_key(|a| a.as_ref().to_lowercase());
}

fn main() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];

    println!("unsorted: {:?}", &users);
    sort_usernames(&mut users);
    println!("sorted:   {:?}", &users);
}

#[test]
fn five_users_str() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}

#[test]
fn five_users_string() {
    let mut users = vec!["Todd".to_string(), "Amy".to_string(), "mike99".to_string(), "Jennifer".to_string(), "alison".to_string()];
    let sorted = vec!["alison".to_string(), "Amy".to_string(), "Jennifer".to_string(), "mike99".to_string(), "Todd".to_string()];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
