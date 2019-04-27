# quine
a rust quine

oggi per tv fanno vedere un servizio sull'italia in miniatura. ovviamente come non si puo' individuare subito che il luogo piu interessante da visitare nell'italia in miniatura e' quel punto che rappresenta l'italia in miniatura stessa. cioe' cammini per "italia in miniatura" e cerchi il punto la cui mappa coincide con il punto; esiste per definizione se "italia in minuatura" e' una italia in miniatura (la contrazione di Banach-Caccioppoli piu o meno ). 
lo stesso lo si puo fare per i programmi e c'e' lo assicura Kleene (e l'omnipresente diagonale di Cantor): si puo sempre scrivere un programma che computa stampandolo esattamente il proprio codice sorgente( e senza barare ). 
ed e' quello che fanno le cellule con il loro dna (virus e tumori e particelle alfa a parte). 
poichè non posso andare a verificare nell'italia in minatura ed ho finito i popcorns ne ho scritto uno in Rust.
```rust
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
```
