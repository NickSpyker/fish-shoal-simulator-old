<h1 align="center">Fish Shoal Simulator</h1>

<p align="center">Interactive fish shoal simulator based on local rules to explore collective motion, phase transitions, and realistic flock behaviors.</p>

## Downloads

|                                                           **Windows**                                                           |                                                         **Linux**                                                         |                                                         **MacOS**                                                         |
|:-------------------------------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------------------------------------------------------------------------:|:-------------------------------------------------------------------------------------------------------------------------:|
| [\>> Download <<](https://github.com/NickSpyker/fish-shoal-simulator/releases/latest/download/fish-shoal-simulator-windows.exe) | [\>> Download <<](https://github.com/NickSpyker/fish-shoal-simulator/releases/latest/download/fish-shoal-simulator-linux) | [\>> Download <<](https://github.com/NickSpyker/fish-shoal-simulator/releases/latest/download/fish-shoal-simulator-macos) |

## Usage

### In-App Configuration

|      **Field** |  **Value**   |          **Range**          | **Description**                                                                                                                                      |
|---------------:|:------------:|:---------------------------:|:-----------------------------------------------------------------------------------------------------------------------------------------------------|
|   **Entities** |  _Integer_   |       `0` → `10,000`        | The number of fish in the simulation.                                                                                                                |
|      **Width** |  _Integer_   | `100` → `max screen width`  | The width of the simulation area in pixels.                                                                                                          |
|     **Height** |  _Integer_   | `100` → `max screen height` | The height of the simulation area in pixels.                                                                                                         |
|  **Direction** | _Percentage_ |        `0%` → `100%`        | Influences the randomness (standard deviation) in heading changes when a fish is selecting a new direction.                                          |
|      **Speed** | _Percentage_ |        `0%` → `100%`        | Influences the average swimming speed and its variability, based on the Gamma distribution's parameters.                                             |
|     **Stress** | _Percentage_ |        `0%` → `100%`        | Could be used to scale reaction times or the magnitude of avoidance maneuvers, making fish more or less predictable.                                 |
| **Attraction** |  _Decimal_   |       `3.0` → `50.0`        | The maximum distance a fish can detect others for schooling behavior. Fish will approach neighbors within this range but outside the alignment zone. |
|  **Alignment** |  _Decimal_   |       `2.0` → `50.0`        | The outer boundary for alignment. A fish will try to match the heading of neighbors that are between the avoidance and alignment distances.          |
|  **Avoidance** |  _Decimal_   |       `1.0` → `50.0`        | The minimum comfortable distance. If a neighbor enters this zone, the fish will perform an avoidance maneuver to increase separation.                |

### Additional Information

> The simulation world uses **toroidal wrapping**, meaning that entities exiting one edge of the screen reappear on the opposite edge, so the left and right borders are connected and the top and bottom borders are connected.

### App Demo

![fish-shoal-simulator.gif](screenshots/fish-shoal-simulator.gif)

## License

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) for details.
