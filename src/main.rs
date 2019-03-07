#[macro_use] extern crate serenity;
#[macro_use] extern crate serde_derive;

mod commands;

use dotenv::dotenv;
use std::{collections::HashSet, env, sync::Arc};
use serenity::{
	client::CACHE,
	client::bridge::gateway::{ShardManager},
	framework::standard::{help_commands, HelpBehaviour, StandardFramework},
	model::{gateway::Game, gateway::Ready, user::CurrentUser},
	prelude::*,
	http
};
// use serenity::model::channel::Message;
// use serenity::framework::standard::Args;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
	type Value = Arc<Mutex<ShardManager>>;
}

struct CommandNames;

impl TypeMapKey for CommandNames {
	type Value = Arc<Vec<&'static str>>;
}

struct Handler;

impl EventHandler for Handler {
	fn ready(&self, ctx: Context, ready: Ready) {
		ctx.set_game(Game::playing("Rust is hard"));

		println!("Ready! Username: {}", ready.user.name);
	}
}

fn main() {
	dotenv().ok();

	let token = env::var("DISCORD_TOKEN").expect("Expected a bot token environment variable");
	let mut client = Client::new(&token, Handler).expect("Error creating client");

	{
		let command_names = Arc::new(vec!["about", "avatar", "catgirl"]);

		let mut data = client.data.lock();
		data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
		data.insert::<CommandNames>(Arc::clone(&command_names));
	}

	let owners = match http::get_current_application_info() {
		Ok(info) => {
			let mut set = HashSet::new();
			set.insert(info.owner.id);

			set
		},
		Err(why) => panic!("Couldn't get application info: {:?}", why),
	};

	client.with_framework(
		StandardFramework::new()
			.configure(|c| c
				.owners(owners)
				.allow_dm(true)
				.on_mention(true)
				.prefix("m,")
				.prefix_only_cmd(about)
				.delimiter(" "))
			.before(|_, msg, _| !msg.author.bot)
			.after(|_, _, command_name, error| {
				match error {
					Ok(()) => println!("Executed command '{}'", command_name),
					Err(why) => println!("Command '{}' returned error {:?}", command_name, why)
				}
			})
			.unrecognised_command(|ctx, msg, attempted_name| {
				for name in ctx.data.lock().get::<CommandNames>().unwrap().iter() {
					if name.contains(attempted_name) {
						msg.channel_id.say(format!("Unknown command. Did you mean m,{}?", name))
							.expect("Error sending message");
						break;
					}
				}
			})
			.simple_bucket("about", 5)
			.command("about", |c| c.cmd(about))
			.customised_help(help_commands::with_embeds, |c| c
				.individual_command_tip("To get more information about a specific command, pass it as an argument.")
				.striked_commands_tip(None)
				.command_not_found_text("Command not found: `{}`")
				.max_levenshtein_distance(3)
				.lacking_permissions(HelpBehaviour::Hide)
				.lacking_role(HelpBehaviour::Hide))
			.simple_bucket("avatar", 2)
			.command("avatar", |c| c
				.known_as("a")
				.desc("Get a user's avatar")
				.usage("<username|id|mention>")
				.example("Mirai")
				.cmd(commands::avatar::avatar))
			.command("catgirl", |c| c
				.batch_known_as(vec!["neko", "nekos", "catgirls"])
				.desc("Get a catgirl")
				.cmd(commands::catgirl::catgirl))
	);

	if let Err(why) = client.start() {
		println!("Client error: {:?}", why);
	}
}

fn get_bot() -> CurrentUser {
	CACHE.read().user.clone()
}

command!(about(_ctx, msg, _args) {
	let bot = get_bot();

	if let Err(why) = msg.channel_id.send_message(|m| m
		.embed(|e| e
			.author(|a| a
				.name("Mirai Bot")
				.url("https://mirai.brussell.me")
				.icon_url(bot.avatar_url().unwrap().as_str()))
			.description("Hi, I'm Mirai!\n\
			I'm an experimental version of Mirai written in Rust\n\
			\n\
			You can learn more about me [on my website](https://mirai.brussell.me)"))) {
		println!("Error sending message: {:?}", why);
	}
});
