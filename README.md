# Slugout

by Team Slugout


## Team Members
* Advanced Topic Subteam 1: Physics
	* Lily Young
	* Nicole Poliski
	* Jimmy Lining Liu

* Advanced Topic Subteam 2: AI
	* Paul Ringenbach
	* Brayden Yan
	* Alex Zhou

* Advanced Topic Subteam 2: Networked Multiplayer
	* Luke Charlesworth
	* Rafael Hollero
	* Jake Ross


## Game Description

Slugout is a top down view multiplayer game. The main goal of the game is to be the last man standing. At the start, there will be a ball that is bouncing around the map. Each player will have a health bar that decreases when hit by the ball. More balls will be added as the game goes on. Each player will have a bat that they utilize to hit the ball and attempt to hit other players. Up to 4 players can play at once. Non playable characters can be added to play against as well. 


## Advanced Topic Description

### Physics

Advanced physics will be implemented when the ball, characters, walls, and surfaces interact. We will implement different surfaces and materials that will vary in friction and energy absorption which will effect the ball and characters differently. There will be multiple balls that vary in size, weight, elasticity, and density. The balls will also be implemented to allow spinning. 
    
### AI

AI will be used to create non-playable characters. We want to have a difficulty slider that will allow the players to customize what level of difficulty they want. This slider will correspond to probabilities within the behaviour tree. Players will be able to select the difficulty for each AI player that is added. The behaviour tree will have branches enabled/disabled based on the behaviour mode the AI is in, which is determined by the current state of the game. The difficulty slider would affect AI behaviour in aggression, hit accuracy, and quality of play.

### Networked Multiplayer

2-4 players will be able to compete against eachother. 


## Midterm Goals

* Basic visuals are on screen
  	Top down view
	Includes a playable cat character that can move up, down, left, and right around a static map and will stop when hitting the edge of the screen,
	a swinging (animated) bat that the player holds and swings,
	a map that is modeled off the living room of a cat grandma,
	2 - 3 objects around the map (recliner, a cat tree, a coffee table, etc.)
	and yarn balls moving around the map.
* Intro screen is implemented with all options available (1-4 players and AI players)
* Physics 
	Ball is bouncing off of surfaces and walls accounting for the angle that the ball hits.
	Ball is slowing down due to friction from the ground.
	Player is bouncing off of surfaces and walls.
	Does not include player to player, ball to ball, and ball to player physics.
* AI - A basic behavior tree is implemented. This will be the base for building off of for the difficulty slider. 
* Networked Multiplayer - Up to 4 players should be able to all be on the same server at once.


## Final Goals

* 10%: All visuals (map, characters, ball, and objects) make sense and can be easily understood and followed. Health is fully implemented including health decreasing when hit by a ball, "death" of characters when their health runs out, and a health bar on screen. If our character is 1 tile in our game, the map will be 1200 tiles or more.
* 10%: Player controls are working.
	The players can hit the ball and move accurately and can swing and hit a ball accurately.
* 20%: AI
	AI characters play with purpose and strategy. There is a fully implemented decision tree that can be adjusted by the player using a difficulty slider.
* 20%: Networked Multiplayer
	2-4 characters can compete at once.
* Physics
	5%: The ball accurately bounces around the map and off players and other balls accounting for the size, weight, elasticity, density, and angular velocity of the 	ball and energy absorption of the wall. This includes spinning when hit a certain way by the players bat.
	5%: The players will accurately bounce off of eachother and the walls accounting for the energy absorption of the wall.
	5%: There are different surfaces and materials that vary in friction and energy absorption and effect the ball and players differently. 
	5%: There are different balls that vary in size, weight, elasticity, and density.


## Stretch Goals

* Implement 3 more maps that are unique and vary.
	They will also have at least 1 extra/different characteristic, for example the balls may move significantly faster on one map, or there may be a new surface. 
* Implement 3 or more powerups that give various benefits or disadvantages to the players and AI, for example a larger bat. 
