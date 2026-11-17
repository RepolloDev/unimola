#import "@preview/unimola:0.1.0": (
  show-unimola-blocks,
  unimola,
  unimola-version,
)

#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#show heading: set block(sticky: true)

= Unimola Demo

Engine: `plantuml-core` (PlantUML spec #unimola-version()).

== 1. Sequence diagram (`#unimola` inside `figure`)

Wraps a diagram in a standard Typst `figure` with caption and cross-reference (@fig-sequence):

#figure(
  caption: [Order creation sequence],
  unimola(
    height: 5.5cm,
    ```plantuml
    @startuml
    actor Client
    participant "API Gateway" as API
    database PostgreSQL as DB

    Client -> API : POST /orders
    activate API
    API -> DB : INSERT order
    DB --> API : order_id
    API --> Client : 201 Created
    deactivate API
    @enduml
    ```,
  ),
) <fig-sequence>

== 2. Fenced `plantuml` block (`show-unimola-blocks`)

Enabled locally with `#show raw.where(lang: "plantuml"): show-unimola-blocks(height: 5.5cm)`:

#show raw.where(lang: "plantuml"): show-unimola-blocks(height: 5.5cm)

```plantuml
@startuml
class Order {
  +id: UUID
  +status: OrderStatus
  +total(): Decimal
}

class Shipment {
  +tracking: String
  +weightKg: Float
}

Order "1" *-- "1..*" Shipment : contains
@enduml
```

== 3. Auto-wrapped `puml` block (`auto-wrap: true`)

Enabled locally with `#show raw.where(lang: "puml"): show-unimola-blocks(height: 5cm)` (no `@startuml` needed):

#show raw.where(lang: "puml"): show-unimola-blocks(height: 5cm)

```puml
[*] --> Registered
Registered --> InTransit : dispatch
InTransit --> Delivered : signature
Delivered --> [*]
```

#pagebreak(weak: true)

== 4. External `.puml` file (`read(...)`)

Loads diagram markup from an external file:

#figure(
  caption: [Unimola rendering pipeline],
  unimola(
    width: 100%,
    read("diagrams/architecture.puml"),
  ),
) <fig-arch>

== 5. Built-in theme (`theme: "amiga"`) and `scale`

Applies a bundled PlantUML theme and `scale: 90%`:

#unimola(
  theme: "amiga",
  scale: 90%,
  "
  start
  :Receive request;
  if (Stock available?) then (yes)
    :Reserve inventory;
  else (no)
    :Notify delay;
  endif
  stop
  ",
)

== 6. Custom `!theme` via `includes`

Loads a custom `.puml` theme file with vivid colors:

#unimola(
  includes: (corporate: read("themes/corporate.puml")),
  ```plantuml
  @startuml
  !theme corporate
  Alice -> Bob : Uses custom neon theme!
  @enduml
  ```,
)

== 7. Error handling

Invalid PlantUML syntax renders a framed error box without aborting compilation:

#unimola("@startuml\n???\n@enduml")

#pagebreak()

== 8. Document prose with `lorem` and PlantUML

#lorem(45)

#figure(
  caption: [Service request lifecycle],
  unimola(
    height: 5.5cm,
    ```plantuml
    @startuml
    Client -> Auth : Login
    Auth --> Client : JWT Token
    Client -> Service : Request + JWT
    Service --> Client : 200 OK
    @enduml
    ```,
  ),
) <fig-prose>

As shown in @fig-prose, #lorem(60)

#lorem(40)
