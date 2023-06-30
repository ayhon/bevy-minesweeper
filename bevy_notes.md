# Bevy
- Uses an [[Entity Component System]]  
	- An **entity** represents an instace of something in our game  
		- It can be the current game window, the player, the scoreboard  
		- In OOP, its equivalent would be instances of classes  
	- A **component** is just a piece of data that belongs to entities.  
	    - In OOP, its equivalent would be the attributes of clases. The key difference is that attributes are decoupled from their classes  
	- A **system** is a piece of logic that acts on certain components (or set of components)  
		- Basically logic that acts on data   
		- In OOP, its equivalet would be the class methods. The key difference is that methods are also decoupled from their classes.  
- Spawn an entity using the `Commands::spawn` method  
	-  
	  ``` rust
	  	  pub fn setup(mut commands: Commands){
	  	      commands.spawn(
	  	          (
	  	              Component1 { },
	  	              Component2 { ... },
	  	          )
	  	      )
	  	  }
	  ```
	-  
- Adding systems to an `App`  
	- Use `add_system` if it will run every time  
	- Use `add_startup_system` if it will run only once at startup  
- Systems are functions, they take as arguments queries for entities, resources, etc  
	- A `Query<...>` type takes some components as type arguments, the entities matched are those with all the components listed  
		- Use the `With<...>` type to indicate that the entity must have a component, but give no access to it  
		- Use `Without` to indicate the opposite  
		- Use `Query::get_single` if the result of the query should be only one object.
- You can a encapsulate systems and components in a project using plugins  
- Add plugins to an `App`  
	- Use `add_plugin` to add a specific plugin  
	- Use `add_plugins` to add a plugin-group (See `PluginGroup`)  
	- Bevy offers a `DefaultPlugins` plugin-group in `bevy::prelude` with the basic additions  
- Resources are elements or struct of which only one instance exists.  
	- They function like global values 
- Assets  
	- To load assets, use the `AssetServer`
- #### Useful things from bevy
	- [`SpriteBundle`](https://docs.rs/bevy/latest/bevy/prelude/struct.SpriteBundle.html)
	- [`]