# Mentat Frontend Architecture

All of the following architectures can be developed in tandem, though some will require more overhead.

## Table of Contents

- [Public Portal]
- [Public Aggregate Portal]
- [Private Interface]
- [Private Aggregate Portal]

[public portal]: #PublicPortal
[public aggregate portal]: #ServerPortal
[private interface]: #PrivateInterface
[private aggregate portal]: #PrivatePortal

## [](#PublicPortal) Public Portal Architecture

A frontend as a public portal is one public facing domain (cloud.mentat.tld) that allows uses to enter their Rosetta API endpoints and make local [Cross Origin Requests](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) without leaking any information to the site owner.

### [](#PublicPortalUserFlow) User Flow

1. Open `cloud.mentat.tld` in browser
1. User is prompted with an "Enter Rosetta API Url" input
1. Once url is input, webpack checks if the endpoint is accessible
   - If it is not accessible, a page with the respective url is opened in a new tab so the user can accept a certificate if necessary or check if the endpoint is available.
1. User sees no login/auth unless we provide some middleware and have that information accessible via api (perhaps `/.mentat/auth`)
1. User sees Dashboard

### Pros

- Every user is on the same version of the frontend
- Easily accessible
- No downloads besides a browser
- Static hosting and high availability

### Cons

- All Rosetta API endpoints require TLS
  - Can be solved by providing our own middleware
- If the TLS certificate is self signed, will require an extra step of "accepting an insecure certificate"
  - Can be solved by using a wildcard certificate to issue certificates to each ip. (eg. `192-168-1-40.PrivateSha256.mentat.tld`)
  - Wildcard certificates may require the endpoints to be public facing rather than local ips depending on Root CA.

### Development Overhead

- Inherits overhead from [Private Interface]
- Optional auth solution
- Cross Origin request solution for other Rosetta APIs

## [](#ServerPortal) Public Aggregate Portal Architecture

Similar to the [Private Interface] architecture, but user account information and endpoints is stored on our platform. Accounts can be optional when viewing a single endpoint, in which this architecture functions as the [Public Portal]

### [](#PublicAggregatePortalUserFlow) User Flow

1. User is prompted to sign in with Github/Gitlab/Google/email/etc
1. Endpoint selection modal
   - With user accounts:
     - User is prompted to select an endpoint or connect to one or view multiple simultaneously
   - Without user accounts:
     - User is prompted connect to an endpoint
1. Continued in [Public Portal Flow](#PublicPortalUserFlow)

### Pros

- Inherits pros from the [Public Portal] but without static hosting
- Easier to create analytics and provide support to users
- Convenience for users for aggregating multiple Rosetta endpoints

### Cons

- Inherits cons from the [Public Portal]
- Users may be required to create an account with us to store more than one endpoint

### Development Overhead

- Inherits overhead from [Public Portal]
- Aggregate Rosetta API code
- Public Facing Backend
  - Database for storing users, user endpoints
  - Oauth
  - Maintainence, high availability/uptime

## [](#PrivateInterface) Private Interface Architecture

A private interface is a webserver that renders a dashboard for a Rosetta API. This is the MVP for all of the other interfaces so it will be implemented by default. Extra features like a standalone webview application for accessing Rosetta APIs or bundling with Mentat can be added.

### User Flow

- User runs Mentat
  1. Mentat is configured with API location so user automatically sees Dashboard
- User runs webview application
  1. Webview application prompts for Rosetta API endpoint
  1. After entering endpoint url, user sees Dashboard

### Pros

- User has full control over local application
- If bundled with Mentat, user does not need to install anything
- Does not require Rosetta API endpoints to have https

### Cons

- Update distribution needs to be solved

### Development Overhead

- Minimum required development for a Rosetta Frontend

## [](#PrivatePortal) Private Aggregate Portal Architecture

Similar to the Private Portal architecture, however it is not bundled with Mentat or a specific Rosetta API. This would serve as an application that aggregated multiple Rosetta API endpoints into a single dashboard.

### User Flow

- Same as the [Public Aggregate Portal's User Flow](#PublicAggregatePortalUserFlow) except without users or login (with the exception of custom login middleware).
- If the user makes their private portal publicly accessible (internet instead of intranet), an auth flow is a requirement for non-demo purposes.

### Pros

- Inherits pros from the [Private Interface]
- Could have auth middleware and allow a user to login remotely to this frontend and access their private backends.

### Cons

- Inherits cons from the [Private Interface]
- Not as accessible as a public portal and requires software to be downloaded

### Development Overhead

- Auth middleware for public facing option (Can be outsourced to other auth software)
- Specialized frontend needs to be built for aggregation
