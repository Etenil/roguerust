# TODO

- Add unit tests
- Disassociate level to view, move view around if terminal too small
- Add equipment system
- Remove rendering logic from State


## Separate rendering logic

```
-------        ---------
| main| -----> | State |
-------    |   ---------
           |
           |   ------------
           +-> | ViewPort |
               ------------
```

- Main game logic mutates State in reaction to player input
- Render is triggered by giving it a read-only reference to current state
- Heavy use of traits in ViewPort allows multiple type of viewports (Piston etc.)
