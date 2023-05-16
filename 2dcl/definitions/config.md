# The Config.toml File

The config.toml file allows you to customize how you view and experience 2dcl.
It is divided in three main sections: `avatar`, `world` and `player`.
If the file is not present, the client will use its default values.

## Avatar

In the avatar section you can define what your avatar looks like while you're exploring the world.
It has six keys: `eth_address`, `cell_shading`, `ambient_light`, `ambient_light_brightness`, `ambient_light_r`, `ambient_light_g` and `ambient_light_b`.

`eth_address` takes a string value and it represents the Ethereum address associated with the Decentraland avatar that you want to use in 2dcl. If the address defined is not valid or doesn't have a Decentraland avatar associated with it, it will load the last successfully imported avatar and prompt the following message:

```
could not find a decentraland avatar for the given ethereum address
```

If `eth_address` is not defined, it will load a default avatar.

`cell_shading` takes a boolean value and allows the avatar to be imported with cell shading. If not defined, the default value is `false`.

`ambient_light` takes a boolean value and allows an ambient light to project its light into the player avatar. If not defined, the default value is `true`

`ambient_light_brightness` takes a float value and if `ambient_light` is defined as `true` it allows you to change the brightness of the ambient light projecting into the player avatar. If not defined, the default value is `0.1`.

`ambient_light_r`, `ambient_light_g`and `ambient_light_b` all take a float value and defines the `red`, `green` and `blue` values for the `ambient_light` if it's defined as `true`. All these keys take the default value of `0.25` if not defined. 


## World

In the world section you can define how the world works. It has 5 keys: `starting_parcel_x`, `starting_parcel_y`, `min_render_distance`, `max_render_distance` and `camera_size`.

`starting_parcel_x` and `starting_parcel_y` both take integer values and defines the parcel where the avatar player spawns when running 2dcl. So for example, if `starting_parcel_x` is `10` and `starting_parcel_y` is `-15`, the player will spawn at the parcel `10, -15`. If not defined, the default value of both keys are `0`.

`min_render_distance` takes an unsigned integer value that defines how many parcels around the player will 2dcl download and render at all times. So if for example `min_render_distance`is `4` and the player is at parcel `0,0`, all parcels from `-3,-4` to `4,4` will be download and render.

`max_render_distance` takes an unsigned integer value that defines how many parcels away from the player will 2dcl despawn and stop rendering. So for example if `max_render_distance` is `7` and the player was at parcel `0,0` but now is at parcel `-8,0`, the parcel `0,0` will despawn.

`camera_size` takes a float value that defines how big the camera for the player is. The higher the value, the more of the world will be visible. If not defined, or defined with a negative value or `0.0` it will take the default value of `1.0`.

## Player

In the player section you can define how you move or interact with the world. It has 4 keys: `speed`, `scale`, `collider_size_x` and `collider_size_y`.

`speed` is a float value that defines how fast you move around the world. If not defined the default value is `400.0`.

`scale` defines how big the player avatar is relative to the world. If not defined the default value is `1.0`.

`collider_size_x` and `collider_size_y` both take float values and defines how big the box collider of the player is. So for example a value of `5` in `collider_size_x` and a value of `5` in `collider_size_y` will make the player collider a small square. If not defined, the default values are `18.0` for `collider_size_x` and `20.0` for `collider_size_y`.


