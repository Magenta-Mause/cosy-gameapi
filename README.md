# COSY Gameapi

This service is currently just an ablated wrapper for [SteamGridDB](https://www.steamgriddb.com/).

Some functionality is taken from the [`steamgriddb_api`](https://crates.io/crates/steamgriddb_api) crate by [@PhilipK](https://github.com/PhilipK) on GitHub.

### Setup
The api can be setup either by manually compiling the project or running the docker compose script in the `docker/` directory of this project.

In order to work correctly, the environment variable `COSY_GAMEAPI_SGDB_API_KEY` has to be set or configured for the docker container to use.

### Endpoints

The following endpoints are exposed:
- GET `/games`
  - Search for general game information by their names / substrings contained in their names.
  - Query Parameters:
    - `query` String containing (fragment of) game name. E.g. `zel` for `zelda` 
    - (optional) `limit` Integer limiting the number of returned results (defaults to `15`)
    - (optional) `offset` Integer describing number of results to skip (defaults to `0`)
    - (optional) `include_hero` String (either `true` or `false`) deciding whether a game hero url should be attempted to be fetched (defaults to `none`)
    -  (optional) `include_logo` String (either `true` or `false`) deciding whether a game logo url should be attempted to be fetched (defaults to `none`)
  - Response:
    - `200 OK` - A JSON Object of the following shape:
         ```ts
            {
                success: boolean,
                timestamp: number,
                data: {
                    games: [
                        {
                            id: number,
                            name: string,
                            hero_url?: string,
                            logo_url?: string,
                        },
                        ...
                    ],
                    is_final: boolean,
                },
            }
         ```
    - `500 Internal Server Error` - A JSON Object of the following shape:
        ```ts
            {
                status: boolean,
                timestamp: number,
                message: string,
            }
        ``` 

- GET `/assets/{game_id}`
  - Fetch assets (images) for a specific game by its ID.
  - Path Parameters:
    - `game_id` (usize) - The unique ID of the game for which to fetch assets.
  - Query Parameters:
    - `limit` (optional, integer) - Maximum number of assets to return (defaults to `15` if not provided).
    - `offset` (optional, integer) - Number of assets to skip before returning results (defaults to `0` if not provided).
  - Response:
    - `200 OK` - A JSON object containing the assets for the game:
      ```ts
      {
          success: boolean,
          timestamp: number,
          data: {
              assets: [
                  {
                      width: number,
                      height: number,
                      url: string
                  },
                  ...
              ],
              is_final: boolean
          }
      }
      ```
      - `assets` - A list of images associated with the game. Each image includes its width, height, and URL.
      - `is_final` - Boolean indicating whether this is the last page of assets.
    - `500 Internal Server Error` - Returned if fetching assets fails:
      ```ts
      {
          status: boolean,
          timestamp: number,
          message: string
      }
      ```

## P.S.
Further, more specialized documentation may follow
