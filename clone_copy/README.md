# Clone und Copy

[`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) und [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) sind zwei fundamentale traits aus der Standardbibliothek.
Mit dem [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) Trait können wir beliebigen, eigenen Code ausführen um einen Wert mit `clone()`zu duplizieren, während das [`Copy`](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy) trait
lediglich ein Marker Trait ist.
Implementieren wir für ein `struct` das [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait,
so signalisieren wir dem Compiler, das er den Speicher den unser `struct` auf dem Stack einnimmt bit für bit kopieren kann um eine eigenständige Kopie zu erstellen.
Wir schreiben also keinen eigenen Code.
Kreire in `clone_copy/src/lib.rs` zwei `structs` und implementiere das [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait für sie. Dafür kannst du auch ein [derive macro](https://doc.rust-lang.org/std/clone/trait.Clone.html#derivable) benutzen.

```rust
#[derive(Debug)]
pub struct Point2D{
    pub x:f32,
    pub y:f32
}

#[derive(Debug)]
pub struct PointVariableD{
    pub components:Vec<f32>
}
```

Für eins der beiden `structs` kann auch das [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait [implementiert](https://doc.rust-lang.org/std/marker/trait.Copy.html#how-can-i-implement-copy) werden.

Eine wichtige Konsequenz des [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) traits können wir uns in einem Integrationstest anschauen. Kreire dafür eine Datei `clone_copy/tests/exploratory_copy_test.rs`.
Implementiert ein `struct` das [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait, so werden Variablen nicht übergeben, sondern kopiert. Integers implementieren zum Beispiel das [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) trait, wodurch folgender Code valide ist:

```rust
#[test]
fn test_integer_copy(){
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy
    let x=5;
    let y=x;
    println!("x = {}, y = {}", x, y);
}
```

Schreibe ein paar tests um dich davon zu überzeugen, dass structs,
die [`Copy`](https://doc.rust-lang.org/std/marker/trait.Copy.html) implementieren nach dem Ownership Wechsel noch valide sind.
Die tests kannst du ausführen mit:

```bash
cargo test -- --show-output
```
