# Game

## How to handle input

We first listen for input events (keyboard, mouse, etc.)

The results will be in InputManager.

Then for continous input(such as movement), we will generate a immutable Copy+Clone key record from manager and sent it to receiver.