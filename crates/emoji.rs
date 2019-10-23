fn main(){

value(Emojis::Smile);
value(Emojis::Sad);
value(Emojis::Lol);
value(Emojis::Wink);
value(Emojis::Heart);


}

enum Emojis {
Smile,
Sad, 
Lol,
Wink,
Heart,
}

fn value(x: Emojis) {
    match x {
        Emojis::Smile => println!("{}",'\u{1f642}'),
        Emojis::Sad => println!("{}",'\u{1f641}'),
        Emojis::Lol => println!("{}",'\u{1f602}'),
        Emojis::Wink => println!("{}",'\u{1f609}'),
        Emojis::Heart => println!("{}",'\u{1f498}'),
    }

}