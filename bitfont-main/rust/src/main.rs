fn main() {
    let font = bitfont::LIBERTINUS_SERIF_18;
    let ch = '&';
    println!("\n  {}:", ch);
    for row in font.characters[ch as usize] {
        println!("\t{}", row);
    }
}
