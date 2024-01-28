# Idle Champion Codes

[![Build Status](https://github.com/Liefland/idle_champions_codes_api/actions/workflows/rust.yml/badge.svg)](https://github.com/Liefland/idle_champions_codes_api/actions)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](README#license)

Web API to list Idle Champions of the Forgotten Realms codes.

## Running it

`cargo run` should work, we use Rocket under the hood so you can use [Rocket.toml](https://rocket.rs/) configuration.

Example: Consuming the API with curl and jq:

```bash
curl -s http://localhost:8000/v1/codes | jq '.codes | .[] | select(.expired == false).code'
```

Example: Adding a new code:

```bash
curl -s -X PUT http://localhost:8000/v1/codes \
  -H 'Content-Type: application/json' \
  -H 'Accept: application/json' \
  -H 'X-Api-Key: API_KEY' \
  --data-binary @- << EOF
  {"code": "HELL-OWOR-LD!!", 
  "expires_at": 1806038430, 
  "creator_name": "Foo!", 
  "creator_url": "https://foo.creator.bar",
  "submitter_name": "Bar!",
  "submitter_url": "https://bar.submitter.bar"} 
EOF
```

## Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

To set up, we recommend the following:

- Familiarity with sqlx, rocket and postgresql are helpful.
- You will need a postgres service running, and that has the DB with migrations loaded: `db/migrations/*`
  - Optionally, import the seeds too: `db/dev_seeds.sql`
- Create an env file at `.env` using the template file (`.env.template`),
- You might want to add a `Rocket.toml`

## License

Licensed under the following licenses at your option:

- Apache License, Version 2.0 <[LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0>
- MIT license <[LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT>

Files in the project may not be copied, modified, or distributed except according to those terms.
