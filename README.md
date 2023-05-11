# Spaceapi Tuerpi

Feeding our SpaceAPI with Data from our entrance door.

## Setup

Please change `DOOR_PIN`, `OPEN_SPACE`, `CLOSE_SPACE` accordingly.

`DOOR_PIN` defaults to a closed door, when HIGH.

`OPEN_SPACE` and `CLOSE_SPACE` are scripts, that get launched when the corresponding event starts. (They have to inform
the SPACE_API server about the new status)
