# Ownership

In der Datei `ownership/tests/compiler_ownership_rules.rs` wird getestet ob Programme in `ownership/tests/will_compile` kompilieren und Programme in `ownership/tests/will_not_compile` nicht kompilieren. Jedes Programm muss abgekapselt sein, damit `cargo test` ausgeführt werden kann.

1. Führe `cargo test` aus, um dich davon zu überzeugen, dass alle initialen tests durchlaufen.
2. Unkommentiere Zeile 2 in `ownership/tests/compiler_ownership_rules.rs`
   - `cargo test` sollte nun mit recht vielen Fehlern den test verweigern.
   - Dafür sollten allerdings die einzelnen Fehler von deiner IDE angezeigt werden.
   - Kommentiere die Zeile wieder.

Nun ist es Zeit weitere Tests zu schreiben.
In der Datei `ownership/tests/compiler_ownership_rules.rs` existiert ein test `ownership_enforced_by_compiler()`.
Dieser test ist recht speziell:
Er testet **nicht ob** ein Program sich korrekt verhält, sondern **ob es kompiliert** und falls nicht: welche Fehlermeldung gibt der Compiler zurück?

Handelt es sich um ein Programm, dass nicht kompilieren darf, referenziere es in `t.compile_fail()`.
Wenn nun `cargo test` ausgeführt wird, wird eine Datei unter `ownership/tests/wip/<your_test>.stderr` angelegt,
die den Compiler Output beinhaltet. Wenn dort die erwartete Fehlermeldung ist, kannst du die Datei neben den test kopieren.

Um das verhalten zu testen, kannst du eine neue Datei anlegen: `ownership/tests/<my_test>.rs`

```rust
#[test]
fn my_test(){
    //arrange
    //act
    //assert
}
```

Assertions können mit makros aus der Standardbibliothek geschrieben werden: https://doc.rust-lang.org/std/macro.assert.html#examples, aber auch mit dedizierten Crates, wie [spectral](https://crates.io/crates/spectral)

```bash
cargo add --dev spectral
```

3. Schreibe tests wie die Borrowing Rules zur Laufzeit anstatt der Compilezeit überprüft werden können: https://doc.rust-lang.org/std/cell/index.html
4. Schreibe tests wie die Ownership Rules zur Laufzeit anstatt der Compilezeit überprüft werden können: https://doc.rust-lang.org/std/rc/index.html
