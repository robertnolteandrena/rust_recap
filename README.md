# Rust Recap

Du befindest dich gerade in einem [Cargo Workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html). (TL;DR: Workspaces kombinieren verwandte Packete). Hier sind vier Projekte drin um unser Wissen um Ownership, traits, Copy + Clone und den String types wieder aufzufrischen.
In den Packeten `ownership/` und `traits/` sind bereits Integrationsteste enthalten.
Wenn du alle Tests in diesem Workspace ausführen möchtes kannst du hier `cargo test` aufrufen,
wenn du nur die tests aus einem Packet laufen lassen möchtest,
verwende `cargo test <Packet>` oder wechsle in das Packetverzeichnis und rufe dort `cargo test` auf.
In den Unterverzeichnissen findest du immer eine `README.md` Datei in der steht, was du dort machen kannst.
Es sollte möglich sein die Verzeichnisse in einer beliebigen Reihenfolge zu besuchen, vorgesehen ist allerdings:

1. `ownership/`
2. `traits/`
3. `clone_copy/`
4. `string_types/`

Viel Spaß :)
