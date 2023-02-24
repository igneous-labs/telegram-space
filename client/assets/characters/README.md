## dir

user-specific spritesheet is stored in `./{chat_user_id}/spritesheet.png`. If none exists, user fallsback to `./spritesheet.png`

## spritesheet format

- Left is the "canonical" direction that's stored in the spritesheet, perform a h-flip to attain right
- Sprites do not have a set width and height due to different accessories
- Spritesheets are packed using the `Horizontal` layout setting on https://www.codeandweb.com/free-sprite-sheet-packer, resulting in 1 row
- Order should be in sequence following table below, with direction being left left = alert-0, right = walkingOneHanded-3

| action | frames | maplestory action prefix |
| -- | -- | -- |
| idle | 2 | alert |
| walking | 4 | walkingOneHanded |
