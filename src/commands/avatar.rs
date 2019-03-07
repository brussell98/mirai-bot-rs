use std::str::FromStr;
use serenity::model::channel::Message;
// use serenity::prelude::Context;
// use serenity::framework::standard::Args;
use serenity::model::user::User;
use serenity::model::id::UserId;

command!(avatar(_ctx, msg, args) {
	if args.is_empty() {
		send_avatar(msg, &msg.author);
	} else if !msg.mentions.is_empty() {
		send_avatar(msg, msg.mentions.first().unwrap());
	} else if let Some(guild) = msg.guild() {
		let guild = guild.read();

		let matched_members = guild.members_containing(args.full(), true, true);
		if !matched_members.is_empty() {
			send_avatar(msg, &matched_members.first().unwrap().user.read());
		} else if let Ok(potential_id) = UserId::from_str(args.full()) {
			if let Ok(member) = guild.member(potential_id) {
				send_avatar(msg, &member.user.read());
			} else {
				send_not_found(msg, args.full());
			}
		} else {
			send_not_found(msg, args.full());
		}
	} else {
		send_not_found(msg, args.full());
	};
});

fn send_avatar(msg: &Message, user: &User) {
	if user.avatar.is_none() {
		if let Err(why) = msg.channel_id.say(format!("{} doesn't have an avatar", user.name)) {
			println!("Error sending message: {:?}", why);
		}
	} else if let Err(why) = msg.channel_id.say(format!("{}'s avatar:\n{}", user.name, user.avatar_url().unwrap())) {
		println!("Error sending message: {:?}", why);
	}
}

fn send_not_found(msg: &Message, query: &str) {
	if let Err(why) = msg.channel_id.say(format!("No user found matching \"{}\"", query)) {
		println!("Error sending message: {:?}", why);
	}
}
