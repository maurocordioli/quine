// this is a quine
fn main() {
let data = [
    r##"      println!("// this is a quine");"##,
    r##"      println!("fn main() {{");"##,
    r##"      println!("let data = [");"##,
    r##"      for r in &data {"##,
    r##"          print!("    r#");"##,
    r##"          print!("#\"{}\"#",r);"##,    
    r##"      println!("#,");"##,
    r##"      }"##,   
    r##"   println!("    ];");"##,  
    r##"   for r in &data {"##,    
    r##"       println!("{}",r);"##,
    r##"   }"##,
    r##"}"##,
    ];
    println!("// this is a quine");
    println!("fn main() {{");
    println!("let data = [");
    for r in &data {
        print!("    r#");
        print!("#\"{}\"#",r);
        println!("#,");        
    }
    println!("    ];");    
    for r in &data {
        println!("{}",r);
    }    
}
