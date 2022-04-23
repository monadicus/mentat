# Mentat Frontend

A Frontend for the Rosetta API. See [architecture.md](./architecture.md) for project architecture notes.

## Setup

1.  Install node via NVM:

    ```bash
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.3/install.sh | bash
    . ~/.nvm/nvm.sh
    nvm install 16
    ```

2.  Install project dependencies:

        npm i

3.  Build project with dev server:

        npm run server

4.  Open dev server in browser: https://127.0.0.1:3000 _(:8080 is taken by Rosetta API)_

## Access

The endpoint used for routes is based on the contents of the url:

- `http://127.0.0.1:3000/~/` uses endpoint `http://127.0.0.1:8080`
- `http://127.0.0.1:3000/8081/` uses endpoint `http://127.0.0.1:8081`
- `http://127.0.0.1:3000/192.168.1.2:8080/` uses endpoint `http://192.168.1.2:8080`

## Development

- Ensure your editor runs `prettier` on save for formatting
- Ensure your editor runs `eslint` for linting
- Run `npm run lint:diff` to lint only on staged files

## Internationalization

All translations are stored in the `translations` folder.
Use the `en_US.yaml` as a reference.
