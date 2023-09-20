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

Advanced physics will be implemented when the ball, characters, walls, and surfaces interact. We will implement different surfaces and materials that will vary in friction and energy absorption which will effect the ball and characters differently. There will be multiple balls that vary in size, weight, elasticity, and density. The balls will also be implemented to allow spinning. Different players will also have different reactions when hitting eachother and other surfaces. 
    
### AI

AI will be used to create non-playable characters. We will have three options: easy, medium, and hard. Players will be able to select the difficulty for each AI player that is added. Each option will have a different behavior tree.

### Networked Multiplayer

2-4 players will be able to compete against eachother. 


## Midterm Goals

* Basic visuals are on screen
	Playable character that can move around and has a swinging animation 
	Map is setup with background visuals and objects around map
* Intro screen is implemented with all options available (1-4 players and AI players)
* Physics - Ball to map and player to map physics are done
	Ball is bouncing off of surfaces and walls accounting for the size, weight, elasticity, and density of the ball and energy absorption of the wall.
	Ball is slowing down due to friction from the ground
	Player is bouncing off of surfaces and walls
	Does not include player to player, ball to ball, and ball to player physics.
* AI - at least one of the non playable characters is fully functioning.
* Networked Multiplayer - Possesses basic functionality of client-server architecture 


## Final Goals

* 10%: All visuals make sense and can be easily understood and followed and scoring is implemented
* 10%: Player controls are working
	The players can hit the ball and move accurately
* 20%: AI
	AI characters play with purpose and strategy. There are fully implemented decision trees for an easy medium and hard mode.
* 20%: Networked Multiplayer
	2-4 characters can compete at once.
* Physics
	5%: The ball accurately bounces around the map and off players and other balls accordingly
		This includes spinning when hit a certain way
	5%: The players will accurately bounce off of eachother and the walls
	5%: There are different surfaces and materials that vary and effect the ball and players differently
	5%: There are different balls that vary and will move and be affected differently by the same surfaces


## Stretch Goals

* Implement multiple maps that are unique and vary.
	They will all have at least one new element such as a new material or surface, or a new type of ball. Some might also have different characteristics, for example the balls may move significantly faster on one map. 
* Implement powerups that give various benefits or disadvantages to the players and AI. 
