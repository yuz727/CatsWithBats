# Cats With Bats

by Team Slugout


## Team Members
* Physics
	* Lily Young
	* Nicole Poliski
	* Jimmy Lining Liu

* AI
	* Paul Ringenbach
	* Brayden Yan
	* Yuxuan Zhou

* Networked Multiplayer
	* Luke Charlesworth
	* Rafael Hollero
	* Jake Ross


## Game Description

Slugout is a top down view multiplayer game. The main goal of the game is to be the last man standing. At the start, there will be a ball that is bouncing around the map. Each player will have a health bar that decreases when hit by the ball. Each player will have a bat that they utilize to hit the ball and attempt to hit other players. Up to 4 players can play at once. Non playable characters can be added to play against as well. There are four unique maps to choose to play on. Also three powersups are scattered on the map where it can be an benefit or disadvantage to the players and NPCs if picked up.

## Launching the Game

Enter ```cargo run``` in the command line inside the game's folder.

## Advanced Topic Description

### Physics

Advanced physics will be implemented when the ball, characters, walls, and surfaces interact. We will implement different surfaces and materials that will vary in friction and energy absorption which will effect the ball and characters differently. There will be multiple balls that vary in size, weight, elasticity, and density. The balls will also be implemented to allow spinning. 
    
### AI

AI will be used to create non-playable characters. A difficulty slider that will allow the players to customize what level of difficulty they want. This slider will correspond to probabilities within the behaviour tree. Players will be able to select the difficulty for each AI player that is added. Depending on the state of the game, the AI would either act aggressively, passively, or a more idle state. These three modes would be reflected by their change to the behaviour tree, enabling/disabling certain branches, and change some aspect of the random number generation in the decision-making. The difficulty slider would affect AI behaviour in aggression, hit accuracy, and quality of play.

### Networked Multiplayer

2-4 players will be able to compete against eachother. 
