# Traits

Willkommen zu dem kleinen Kapitel über traits. Wir gucken uns kurz an wie man ein simples trait definiert und implementiert. Danach wenden wir uns der Fragestellung zu, wie wir mit traits Polymorphismus verwenden.

1. In der Datei `traits/src/lib.rs` findest du das Trait `HasDescription`.
   Implementiere dieses Trait für ein von dir definiertes `struct`, und für ein externes `struct`: Zum Beispiel `String`
2. Traits können default methods haben. Es kann zum beispiel möglich sein methoden zu schreiben, für die nur die existenz der `description()` methode ausreicht: https://doc.rust-lang.org/book/ch10-02-traits.html#default-implementations. Füge dem `HasDescription` trait eine default methode hinzu.
3. Wir wollen ein `struct` erzeugen, welches instanzen besitzt, die `HasDescription` implementieren.

```rust
pub struct LibraryTraitObject {
    pub described: Vec<Box<dyn HasDescription>>,
}
pub struct LibraryGeneric<T: HasDescription> {
    pub described: Vec<T>,
}
```

Schreibe für beide structs eine methode um ein element hinzuzufügen.

> Um in einem `impl` Generics zu verwenden kannst du die Schreibweise `impl<T: HasDescription> LibraryGeneric<T> { }` verwenden.

Um die Ausgabe von `println` in Test-Code zu sehen, kannst du `cargo test -- --show-output` verwenden.
Was fällt dir auf wenn du versuchst instanzen verschiedener Typen in den Library-structs zu speichern ?
