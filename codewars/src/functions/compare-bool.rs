// compare two boolean values
// false = not in love
// true = in love
fn lovefunc(flower1: u16, flower2: u16) -> bool {
    let timmy = flower1 % 2;
    let sarah = flower2 % 2;
    
    let mut in_love = true;
    
    if timmy + sarah == 0 {
        in_love = false
    }
    
    in_love
}