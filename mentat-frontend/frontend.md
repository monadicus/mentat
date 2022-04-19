# Mentat Frontend Planning

## Ideas

Rosetta API has the potential to be an aggregator on its own by connecting to other Rosetta APIs and bundling the output. If mentat decides to play this role, it needs to provide a config for aggregating rosetta endpoints.

## Future Thinking

- Location of construction api?
  - on account, transaction, network, top level, sidenav?
- Design Options
  - sidenav (hamburger menu)
  - top nav (effective as we have limited menu items)
  - mobile first (more applicable if we have a portal)
  - Palette (duo-tone, tri-tone, color choice)
  - dark mode (first), theming (not mvp)

## Views

### Network list/table

The network list is a top level entry as other objects (accounts, blocks, transactions) depend on them.

Methods of displaying networks could be:

- list selection before other views are seen
- selection dropdown at the top of the page
- header entries

Network List objectives:

- network status is fetched for each item
- block is fetched for each network
- depending on the format, selecting a network may present a list of accounts

Network List components:

- network name
- network status
- number of accounts linked to each network?
- latest block timestamp

Network individual view:

- lookup latest block for each network
- a tab for mempool
- transaction explorer (see transaction view)

### Account list/table

Accounts listed here are ones registered to this endpoint. Individual account views can also be linked to by a transaction view. Perhaps some form of star/pin/favorite system could add the account to the endpoint for tracking.

- add account button/modal (from list or from clicking into an account view)

  - select network (if not already selected)
  - add note/name
  - enter address/sub account address

- account list, either per network or global

  - table of accounts added, their networks, addresses

- account view should show current currencies
- account view may show recent transactions
- should there be a global account view that aggregates currencies from all accounts?

### Block View

- block view
  - show timestamp and links to block search
  - search on top for transactions
  - clicking on a block:
    - show block events
    - maybe show some transactions?
    - transactions should be paginated and clicking on a transaction id should navigate to the transaction view

### Transaction View

Not a list/table, but rather an individual transaction inspector.

May have value as a hover popup/tooltip in some regards.

Shows currencies, transaction times, accounts, related transactions

### Developer View

- json view (plain web requests?)
