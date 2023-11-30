# String, &str und Slice

Willkommen zum Kapitel über `String`, `&str` und `slice`.
Diese drei Datentypen sind verwandt und kommen in Rust häufig vor.
Der `String` Typ ist ein Wrapper um ein `Vec<u8>`.
Auf diese Weise repräsentiert der `String` Typ eine variable Anzahl von Bytes.
Der `&str` Typ hingegen besitzt keine Daten. Er besteht nur aus einem Zeiger und einer Länge.
Der `&str` Typ wird manchmal auch als `string slice` bezeichnet, da ein slice genau wie ein `&str` nur aus einem Zeiger und einer Länge besteht.

## String

Obwohl ein `String` nur ein Wrapper um ein `Vec<u8>` ist, gibt es einige Unterschiede.
Der vielleicht wichtigste Unterschied hat seinen Ursprung darin, dass ein `String` (wie auch ein `&str`) aus gültigen UTF-8 Zeichen besteht. Ein UTF-8 Zeichen kann aus mehr als einem Byte bestehen. Deshalb kann der Typ `String` nicht geindext werden. Weitere Informationen dazu findest du [hier](https://doc.rust-lang.org/book/ch08-02-strings.html#internal-representation).

Betrachte den folgenden String:

``Rust
let hello_world = String::from("Привіт світ");

````

Schreibe einen Test, um zu überprüfen, dass `hello_world` 11 Zeichen hat und durch 21 Bytes dargestellt wird.
Die Methoden [`len()`](https://doc.rust-lang.org/std/string/struct.String.html#method.len) und
[`chars()`](https://doc.rust-lang.org/std/string/struct.String.html#method.chars) sollten sich hierfür als nützlich erweisen.

### Deref Coercion

Formuliere diese Funktion aus:

```rust
fn get_first_char(daten: &str) -> Option<char> {
    todo!()
}
````

Dank Deref Coercion kann diese Funktion mit einem `&str`, `&String` oder sogar mit einem `Box<String>` aufgerufen werden.
Schreibe drei Tests, in denen du die Funktion `get_first_char` mit den drei obigen Typen aufrufst.

## &str

Wie eingangs erwähnt, besteht ein `&str` aus einem Zeiger und einer Länge.
Handelt es sich jedoch um ein Stringliteral `let a:&str="hello"`, so zeigt der Zeiger auf einen hardcoded Wert innerhalb der Binary.
Damit ist dieser Speicher für die Dauer des Programms gültig. Das folgende Code-Snippet zeigt einen Test, bei dem `a` eine Referenz auf ungültigen Speicher ist: es wird nicht kompiliert.

```rust
#[allow(non_snake_case)]
#[test]
fn string_reference_does_not_live_forever() {
    let a: &String;
    {
        let string_reference: &String = &String::from("Hello");
        a = string_reference;
    }
    assert_that!(a.as_str()).is_equal_to("Hello");
}//this will not compile
```

Schreibe diesen Test mit einem Stringliteral um.

## Slice

Ein Slice besteht ebenfalls aus einem Zeiger und einer Länge,
aber im Gegensatz zu einem `&str` gibt es Slices für beliebige Datentypen.
Im folgenden Test wird ein Slice vom Typ `&[i32] erzeugt:

```rust
#[test]
fn taking_a_slice_from_an_array() {
    let data = [0, 1, 2, 3, 4];
    let slice: &[i32] = &data[1..2];
    assert_that!(slice).is_equal_to([1].as_slice())
}
```

Vervollständige den folgenden Test:

```rust

#[test]
fn taking_a_slice_from_a_string() {
    let hello_world = "hello world";
    //todo get hello and world as a string slice
    assert_that!(hello).is_equal_to("hello");
    assert_that!(world).is_equal_to("world");
}
```

Als wir über den `String` Typ gesprochen haben,
haben wir gesehen, dass sowohl `String` als auch `&str` gültige UTF-8 Zeichen sind und nicht indiziert werden können.
Aber wie können wir dann den `&str` slicen?
Wenn wir einen String slicen, indizieren wir die Bytes, nicht die Zeichen.
Beachte das sowohl П als auch р aus 2 Bytes besteht und vervollständige den folgenden Test:

```rust
#[test]
fn slicing_utf8() {
    let hello_world = "Привіт світ";
    //todo get str_slice variable
    assert_that!(str_slice).is_equal_to("Пр")
}
```

Wir müssen jedoch darauf achten, dass wir nicht innerhalb eines UTF-8 Zeichens slicen.
Vervollständige den folgenden Test, indem du innerhalb eines UTF-8 Zeichens slicest.

```rust
#[test]
#[should_panic]
fn cannot_slice_within_character() {
    let hello_world = "Привіт світ";
}
```
